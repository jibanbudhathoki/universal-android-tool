# Security and Ethical Servicing Policy

Universal Android Tool (UAT) adheres to strict security, compliance, and ethical standards.

## Core Security Requirements

1. **Zero Exploit Chains / Zero Bypass**:
   - Universal Android Tool does NOT include, distribute, or execute unauthorized FRP (Factory Reset Protection) bypasses, Google account verification circumventions, unauthorized screen lock removals, or private credential extraction.
   - Any operation targeting device recovery strictly verifies device ownership and follows official manufacturer or Google-authorized procedures.

2. **Accurate Status Reporting**:
   - Never simulate or fabricate success indicators.
   - Every operation status is grounded in verified hardware responses and protocol acknowledgments.

3. **Explicit Permissions & Data Loss Transparency**:
   - The user must explicitly approve any USB connection request.
   - Serial numbers are treated as sensitive data and extracted only when explicitly authorized.
   - Destructive operations (factory reset, wipe, recovery) enforce clear warnings and user confirmation acknowledging total data loss.

4. **Independent Local Operations**:
   - Local device servicing does not depend on cloud databases or remote servers.
