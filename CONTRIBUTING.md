# Contributing to TTL-Legacy

Thank you for contributing to TTL-Legacy!

## Getting Started

1. Fork the repository
2. Clone: `git clone https://github.com/YOUR_USERNAME/TTL-Legacy.git`
3. Create branch: `git checkout -b feature/your-feature-name`

## Branch Naming

- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation
- `test/` - Tests

## Commit Messages

Format: `<type>(#issue): Brief description`

Types: `feat`, `fix`, `test`, `docs`, `refactor`

## Pull Requests

**Before submitting:**
- Run: `cargo test --package ttl-vault`
- Check: `cargo fmt --all -- --check`
- Lint: `cargo clippy --package ttl-vault -- -D warnings`
- Audit: `cargo audit`

## Debugging in VS Code

We have pre-configured VS Code debugging settings in the repository to make it easy to run and debug tests in smart contracts and the backend without manual configuration.

### Prerequisites

1. Install the following extensions in VS Code:
   - **rust-analyzer** (by `rust-lang`)
   - **CodeLLDB** (by `Vadim Chugunov` / `vadimcn.vscode-lldb`)
2. Ensure your build profile allows debugging symbols (default for development profiles).

### Debugging with CodeLens (Recommended)

1. Open any Rust test file (e.g., [test.rs](file:///c:/Users/opulencechuks/TTL-Legacy/contracts/ttl_vault/src/test.rs) or [tests.rs](file:///c:/Users/opulencechuks/TTL-Legacy/backend/src/tests.rs)).
2. You will see a small **Run** and **Debug** button (CodeLens) above each `#[test]` function attribute.
3. Click **Debug** to build and launch the debugger. The workspace settings are pre-configured to use **CodeLLDB** automatically.
4. Set breakpoints by clicking in the left margin next to the line number in your contract or backend code.

### Debugging with VS Code Run & Debug Panel

For more control, you can use the pre-configured profiles in the VS Code **Run & Debug** panel (accessible via `Ctrl+Shift+D` or by clicking the Play icon with the bug symbol in the activity bar):

1. **Debug Specific Test (Prompt)**: Prompts you for a test name substring (e.g., `test_search_vaults_by_owner`), compiles the workspace, and runs matching tests under the debugger.
2. **Debug 'ttl-vault' Library Tests**: Debugs unit tests inside the `ttl-vault` library.
3. **Debug 'ttl-vault' Integration Tests**: Debugs tests in [integration_tests.rs](file:///c:/Users/opulencechuks/TTL-Legacy/contracts/ttl_vault/tests/integration_tests.rs).
4. **Debug 'ttl-vault' Property Tests**: Debugs tests in [property_tests.rs](file:///c:/Users/opulencechuks/TTL-Legacy/contracts/ttl_vault/tests/property_tests.rs).
5. **Debug 'zk-verifier' Library Tests**: Debugs unit tests inside the `zk-verifier` library.
6. **Debug 'ttl-legacy-backend' Library Tests**: Debugs unit tests inside the `ttl-legacy-backend` library.
7. **Debug All Workspace Tests**: Runs the entire test suite under the debugger.

All debug configurations automatically pass `--nocapture` to cargo test so that standard output (`println!`, logs, etc.) is visible in the Debug Console.

## Security Audit Process

We use `cargo audit` to automatically detect and report security vulnerabilities in dependencies.

### Running Audit Locally

```bash
# Install cargo-audit
cargo install cargo-audit

# Run audit (fails on CRITICAL or HIGH severity)
cargo audit --deny warnings
```

### Audit Configuration

The audit configuration is managed in `.cargo/audit.toml`:

```toml
# Denies CI build on these severity levels
deny = ["unmaintained", "unsound"]

# Advisory allowlist with justifications
[advisories]
# "ADVISORY_ID" = { reason = "Justification" }
```

### Handling Vulnerabilities

1. **Immediate Patch**: If a CRITICAL or HIGH vulnerability is found, update the dependency immediately
2. **Minor Patch**: For MEDIUM vulnerabilities, plan an update in the next release cycle
3. **Accepted Advisories**: If a vulnerability cannot be fixed immediately, document the acceptance in `.cargo/audit.toml` with a clear justification

Example accepted advisory:

```toml
[advisories]
"RUSTSEC-2021-0001" = { reason = "Affects unused feature X; scheduled for removal in v2.0" }
```

### CI Integration

The CI pipeline runs `cargo audit` on every PR. Builds will fail if:
- Any CRITICAL or HIGH severity vulnerabilities are detected
- Accepted advisories lack proper justification



## License

Contributions are licensed under MIT License.

