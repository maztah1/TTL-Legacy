use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde::Deserialize;

use crate::{
    audit,
    db::Db,
    error::{ApiError, AppError},
    models::{AuditLogEntry, AuditLogQuery, ReminderPreferences, SetPreferencesRequest},
};

#[derive(Deserialize)]
pub struct RemindersQuery {
    pub include_deleted: Option<bool>,
}

pub async fn list_vault_reminders(
    State(db): State<Arc<Db>>,
    Path(vault_id): Path<u64>,
    Query(query): Query<RemindersQuery>,
) -> Result<Json<Vec<ReminderPreferences>>, AppError> {
    let records = if query.include_deleted.unwrap_or(false) {
        db.all_reminders_including_deleted(vault_id)?
    } else {
        match db.get(vault_id) {
            Ok(p) => vec![p],
            Err(_) => vec![],
        }
    };
    Ok(Json(records))
}

pub async fn delete_preferences(
    State(db): State<Arc<Db>>,
    Path(vault_id): Path<u64>,
) -> Result<StatusCode, AppError> {
    db.soft_delete_reminder(vault_id)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn set_preferences(
    State(db): State<Arc<Db>>,
    Path(vault_id): Path<u64>,
    headers: HeaderMap,
    Json(body): Json<SetPreferencesRequest>,
) -> Result<(StatusCode, Json<ReminderPreferences>), AppError> {
    if body.channels.is_empty() {
        return Err(AppError::InvalidInput("channels must not be empty".into()));
    }
    if body.hours_before_expiry == 0 {
        return Err(AppError::InvalidInput(
            "hours_before_expiry must be > 0".into(),
        ));
    }

    // #825: Idempotency key support
    if let Some(idem_key) = headers.get("idempotency-key").and_then(|v| v.to_str().ok()) {
        if let Some(cached) = db.check_idempotency(idem_key) {
            let cached_prefs: ReminderPreferences =
                serde_json::from_str(&cached.response_body).unwrap();
            return Ok((StatusCode::OK, Json(cached_prefs)));
        }
    }

    let prefs = ReminderPreferences {
        vault_id,
        channels: body.channels,
        hours_before_expiry: body.hours_before_expiry,
        frequency: body.frequency,
        deleted_at: None,
    };
    db.upsert(&prefs)?;

    // Store idempotency record if key was provided
    if let Some(idem_key) = headers.get("idempotency-key").and_then(|v| v.to_str().ok()) {
        let body_json = serde_json::to_string(&prefs).unwrap();
        db.store_idempotency(idem_key, 200, &body_json);
    }

    Ok((StatusCode::OK, Json(prefs)))
}

pub async fn get_preferences(
    State(db): State<Arc<Db>>,
    Path(vault_id): Path<u64>,
) -> Result<Json<ReminderPreferences>, AppError> {
    match db.get(vault_id) {
        Ok(prefs) => Ok(Json(prefs)),
        Err(_e) => Err(AppError::NotFound),
    }
}

// ── Unsubscribe endpoint (#828) ─────────────────────────────────────────────

#[derive(Deserialize)]
pub struct UnsubscribeQuery {
    pub token: String,
}

pub async fn unsubscribe(
    State(db): State<Arc<Db>>,
    Query(query): Query<UnsubscribeQuery>,
) -> Result<(StatusCode, String), AppError> {
    match db.process_unsubscribe(&query.token) {
        Ok(owner) => Ok((
            StatusCode::OK,
            format!("You ({owner}) have been unsubscribed from reminder emails."),
        )),
        Err(_) => Err(AppError::InvalidInput(
            "Invalid or expired unsubscribe token".into(),
        )),
    }
}

// ── Audit Log endpoint (#961) ──────────────────────────────────────────────

#[derive(Deserialize)]
pub struct AuditLogParams {
    pub user_id: Option<String>,
    pub action: Option<String>,
    pub resource: Option<String>,
    pub result: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// GET /api/audit-logs
///
/// Returns audit log entries with optional filtering.
/// Requires `Authorization: Bearer <ADMIN_API_KEY>`.
pub async fn get_audit_logs(
    State(db): State<Arc<Db>>,
    headers: HeaderMap,
    Query(params): Query<AuditLogParams>,
) -> Result<Json<Vec<AuditLogEntry>>, ApiError> {
    audit::authorize_admin(&headers)?;

    let after = params
        .after
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));
    let before = params
        .before
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let query = AuditLogQuery {
        user_id: params.user_id,
        action: params.action,
        resource: params.resource,
        result: params.result,
        after,
        before,
        limit: params.limit,
        offset: params.offset,
    };

    let entries = db.query_audit_logs(&query).map_err(|e| {
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            format!("failed to query audit logs: {e}"),
        )
    })?;

    Ok(Json(entries))
}

