# Response Draft: Safe Edges Security Audit Inquiry

---

## Subject: Re: Security Audit Services for TTL-Legacy Contracts

Hi Safe Edges,

Thank you for reaching out. We're actively planning security reviews for TTL-Legacy before mainnet deployment and would be interested in discussing your services.

### Current Status

**Project**: TTL-Legacy is a Soroban smart contract implementing a "Dead Man's Switch" vault on Stellar. Key features include:
- TTL-based automatic fund release to beneficiaries
- Passkey/WebAuthn authentication
- Multi-beneficiary support with conditional acceptance
- Vesting schedules and withdrawal audit trails

**Security Posture**: We have comprehensive internal security documentation including a threat model and pre-audit checklist, but have not yet engaged an external auditor.

**Deployment Timeline**: Q3 2026

### Questions for Your Team

Before we proceed, we'd like to understand:

1. **Soroban Experience** — Do you have prior experience auditing Soroban/Rust smart contracts? (We're on Stellar, not EVM)
2. **Audit Scope** — We have a detailed [security audit checklist](docs/security-audit-checklist.md) covering 10 areas (~40 items). Can you provide an estimate for a full audit against this scope?
3. **Timeline & Pricing** — What's your typical turnaround time and cost structure?
4. **References** — Can you share references from previous Soroban or Stellar protocol audits?
5. **Deliverables** — What does your final audit report include (findings severity levels, remediation guidance, etc.)?

### Next Steps

We'd welcome:
- A sample audit report (redacted if needed)
- Your team's relevant credentials and past engagements
- A preliminary scope and timeline estimate

Please feel free to review our codebase at https://github.com/OxDev-max/TTL-Legacy and security documentation in `docs/security.md` and `docs/security-audit-checklist.md`.

Looking forward to discussing further.

Best regards,  
TTL-Legacy Team

---

## Notes for Internal Discussion

- [ ] Confirm mainnet deployment timeline
- [ ] Decide on audit budget and priority
- [ ] Verify Safe Edges' Soroban/Rust expertise before committing
- [ ] Request references from similar projects
- [ ] Schedule follow-up call if they meet criteria
