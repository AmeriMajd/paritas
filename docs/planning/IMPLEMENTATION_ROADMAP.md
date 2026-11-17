# Paritas Implementation Roadmap

| Field | Value |
| --- | --- |
| Status | Draft for implementation |
| Roadmap version | 0.1 |
| Date | 17 November 2026 |
| Source | Product Requirements Document v0.2 |
| Target | Open-source public beta |
| Initial protocols | REST/OpenAPI and MCP |
| Implementation | Rust Edition 2024 Cargo workspace |

## Purpose and delivery outcome

This roadmap turns the Paritas product requirements into an executable delivery
plan. It defines the order of work, the output of each phase, the quality gates
required to advance, and the first implementation backlog.

The public beta is complete when a developer can install one binary, discover
REST and MCP operations, map equivalent operations to a business capability,
run safe checks with at least three personas, inspect an evidence-backed
authorization matrix, save a deterministic baseline, and fail CI when access
expands unexpectedly.

The first release is deliberately narrow. GraphQL, assisted mapping, SARIF,
hosted services, dashboards, and advanced mutation workflows must not delay a
trustworthy REST and MCP vertical slice.

## Product constraints

The following rules apply throughout implementation:

- Read-only execution is the default. Mutations require both a command-line
  opt-in and a per-capability configuration opt-in.
- Manual YAML capability mappings are the source of truth for the MVP.
- Unknown is a first-class outcome and must never silently become allow or deny.
- Protocol adapters preserve protocol-specific success and error semantics.
- Secrets are referenced, materialized only at execution time, and redacted
  from logs, reports, baselines, errors, and test snapshots.
- Output order, fingerprints, and exit codes are deterministic.
- The core authorization model has no dependency on REST, GraphQL, or MCP.
- One target failure may reduce coverage but must not corrupt successful results.
- No telemetry or cloud upload is enabled by default.

## Delivery sequence

The estimated path to public beta is 12 to 14 weeks. Estimates are planning
ranges, not release commitments; every phase ends at an acceptance gate.

| Phase | Indicative length | Outcome | Release gate |
| --- | ---: | --- | --- |
| 0. Validate the workflow | 1–2 weeks | Confirm users, terminology, evidence needs, and identity flows | Validation findings recorded and MVP assumptions accepted |
| 1. Build the Rust foundation | 2 weeks | Compiling workspace, domain model, configuration, CLI, and report contracts | Workspace tests pass and core is protocol-independent |
| 2. Complete the REST vertical slice | 2–3 weeks | OpenAPI discovery and safe persona verification | Reference REST fixture produces correct normalized decisions |
| 3. Complete the MCP vertical slice | 2–3 weeks | MCP discovery, invocation, and cross-protocol comparison | Seeded REST/MCP parity violation is detected with useful evidence |
| 4. Add baselines and CI | 2 weeks | Snapshot, diff, machine output, fixtures, and GitHub Actions | Permission expansion fails CI deterministically |
| 5. Harden and publish the beta | 2 weeks | Security review, portability, packaging, and contributor documentation | Beta acceptance checklist passes on Linux, macOS, and Windows |
| 6. Expand after beta | Ongoing | GraphQL, ownership scenarios, SARIF, and assisted mapping | Each feature passes its adapter and safety contracts |

## Proposed repository structure

The initial workspace should remain small enough to refactor while preserving
clear dependency boundaries.

```text
paritas/
├── Cargo.toml
├── rust-toolchain.toml
├── crates/
│   ├── paritas-cli/          # Commands, exit codes, and user-facing errors
│   ├── paritas-core/         # Canonical model, policy, diff, and fingerprints
│   ├── paritas-config/       # YAML schema, validation, interpolation, redaction
│   ├── paritas-rest/         # OpenAPI discovery, HTTP planning, normalization
│   ├── paritas-mcp/          # MCP discovery, invocation, normalization
│   └── paritas-report/       # Terminal, JSON, and Markdown output
├── fixtures/
│   └── reference-app/        # Secure and intentionally vulnerable REST/MCP app
├── examples/                 # Safe example configuration and baseline
├── schemas/                  # Versioned configuration and report schemas
├── docs/
│   ├── product/
│   ├── planning/
│   └── decisions/            # Architecture decision records
└── .github/workflows/
```

`paritas-core` may define adapter-facing traits and canonical types, but it must
not import protocol SDKs. Each adapter converts its native data into the core
`Operation`, `PlannedCall`, `Observation`, and `VerificationEvidence` types.

## Phase 0 Validate the workflow

### Objective

Reduce the risk of building a technically correct tool that teams cannot
configure, trust, or run in CI.

### Work

- Interview 10 to 15 developers, AppSec engineers, and platform engineers who
  maintain REST, GraphQL, or MCP interfaces.
- Collect two or more representative applications exposing an equivalent
  capability through REST and MCP.
- Validate whether policy ownership belongs to developers, AppSec, or platform
  teams in the intended workflow.
- Identify the identity mechanisms required for the MVP, including bearer
  tokens, headers, cookies, process environment, and token refresh commands.
- Test the proposed YAML vocabulary with users before stabilizing schema v1.
- Prototype one authorization matrix and one detailed finding to determine the
  minimum evidence needed for trust and remediation.
- Decide whether CI targets are normally ephemeral fixtures or shared staging
  systems, and document the safety implications of both.

### Deliverables

- Validation summary and interview evidence.
- Prioritized identity-flow list.
- Reviewed configuration example.
- Reviewed terminal finding and authorization matrix mockups.
- Accepted MVP assumptions or a versioned PRD revision.

### Exit gate

Proceed when at least three target users can explain the discover, map, verify,
snapshot, and diff workflow; the initial identity flows are known; and no
validation result invalidates the REST + MCP product wedge.

## Phase 1 Build the Rust foundation

### Objective

Create a deterministic, testable foundation without coupling the core model to
any protocol.

### Work

- Create the Rust Edition 2024 Cargo workspace and pin the initial minimum
  supported Rust version.
- Add formatting, linting, dependency audit, unit-test, and documentation checks.
- Implement canonical types for capability, operation, persona, expectation,
  observation, decision, evidence, finding, and run summary.
- Define the adapter contract for discovery, planning, invocation,
  normalization, and optional verification.
- Implement configuration schema versioning, YAML parsing, environment
  references, validation, and precise YAML-path errors.
- Validate stable capability IDs, operation references, duplicate mappings,
  expectations, and unmapped inventory.
- Implement centralized redaction primitives before any transport code exists.
- Create the `discover`, `map`, `verify`, `snapshot`, `diff`, and `explain`
  command shells with documented exit codes.
- Define versioned JSON structures for operations, observations, findings,
  baselines, and reports.
- Add deterministic sorting, canonical serialization, and stable fingerprinting.
- Record architecture decisions for workspace boundaries, fingerprints, schema
  compatibility, and evidence retention.

### Deliverables

- Compiling workspace and command skeleton.
- Configuration loader with fixture-based tests.
- Protocol-independent domain model and adapter traits.
- Redaction library with leakage tests.
- Terminal and JSON report skeletons.
- Continuous integration for format, lint, test, audit, and documentation.

### Exit gate

`cargo test --workspace` and all CI checks pass. Reordered equivalent inputs
produce identical serialized output and fingerprints. Test secrets do not
appear in captured logs, errors, or snapshots. No protocol SDK is present in
the dependency graph of `paritas-core`.

## Phase 2 Complete the REST vertical slice

### Objective

Deliver the first complete discover-to-finding path using OpenAPI and HTTP.

### Work

- Load OpenAPI 3.x documents from local paths and URLs.
- Discover paths, methods, security schemes, input and output schemas, and
  operation identifiers.
- Classify effects as read, create, update, delete, execute, or unknown.
- Resolve persona credentials from environment references without persisting
  their values.
- Plan safe requests with configurable timeouts, size limits, concurrency, and
  rate limits.
- Block mutation-like operations by default and expose every skipped reason.
- Invoke read-only operations with Reqwest and Rustls.
- Normalize HTTP status and selected body semantics into allowed, denied,
  unknown, or error.
- Produce sanitized evidence and reproduction guidance.
- Render a deterministic persona by capability authorization matrix.
- Build REST fixture cases for anonymous access, vertical escalation, cross-user
  access, field exposure, authentication failure, and ambiguous outcomes.

### Deliverables

- Working OpenAPI inventory.
- REST request planner and executor.
- REST decision normalizer.
- Terminal, JSON, and Markdown output for REST observations.
- Secure and intentionally vulnerable REST fixtures.

### Exit gate

The CLI discovers the reference OpenAPI specification without executing
business operations. Read-only verification correctly classifies every seeded
REST case, preserves ambiguous cases as unknown, and emits no configured secret.

## Phase 3 Complete the MCP vertical slice

### Objective

Prove the central product promise by comparing one business capability through
REST and MCP.

### Work

- Implement MCP initialization and `tools/list` discovery over stdio.
- Add Streamable HTTP discovery and invocation.
- Normalize tool names, schemas, annotations, and risk hints without treating
  metadata as proof of safety.
- Materialize MCP personas through bearer or OAuth headers and process
  environment where applicable.
- Invoke `tools/call` only after the execution planner approves the operation.
- Interpret JSON-RPC failures, tool-level errors, structured results, and
  ambiguous responses independently of transport success.
- Compare persona decisions and field-level evidence across equivalent REST and
  MCP operations.
- Generate stable parity findings with capability, persona, protocol,
  expectation, evidence, severity, and remediation context.
- Seed a reference flaw where MCP loses caller context or uses a privileged
  service credential while REST correctly denies access.

### Deliverables

- MCP stdio and Streamable HTTP adapters.
- Cross-protocol authorization matrix.
- Cross-protocol policy evaluator and finding fingerprints.
- REST + MCP reference application with secure and vulnerable modes.

### Exit gate

Paritas detects every seeded critical cross-protocol violation, reports no
high-confidence finding against the secure fixture, and explains the protocol
path and identity responsible for each discrepancy.

## Phase 4 Add baselines and CI

### Objective

Make authorization drift reviewable in source control and enforceable in pull
requests.

### Work

- Implement deterministic baseline creation with tool, schema, and adapter
  versions.
- Compare current observations with an approved baseline.
- Report operation additions, removals, decision changes, field changes, and
  coverage gaps separately.
- Treat a baseline deny changing to current allow as permission expansion.
- Implement configurable severity thresholds and stable exit codes.
- Complete terminal, JSON, and Markdown reporters.
- Provide a GitHub Actions workflow with artifact upload and Markdown summary.
- Add golden tests for terminal and machine-readable output.
- Test partial target failure without losing valid observations.
- Measure the reference workload of 100 read-only operations across three
  personas under controlled latency.

### Deliverables

- `snapshot` and `diff` commands.
- Versioned, reviewable baseline format.
- GitHub Actions integration example.
- Golden output suite and performance harness.

### Exit gate

A pull request that expands access beyond the baseline fails at the configured
threshold with the documented exit code. Repeated runs with identical inputs
produce byte-stable baselines and logically stable reports. The target CI
workload completes within five minutes under the defined test conditions.

## Phase 5 Harden and publish the beta

### Objective

Prepare a safe, portable, supportable public beta.

### Work

- Threat-model configuration loading, URL and path handling, subprocess use,
  report permissions, secret lifetime, and dependency supply chain.
- Fuzz or property-test parsers, canonical serialization, redaction, and
  protocol normalizers where valuable.
- Verify that mutation execution requires both opt-ins and that dry-run shows
  identity, effect, and planned request without revealing secrets.
- Add production-host guards, request budgets, response limits, and clear
  authorized-use warnings.
- Test Linux, macOS, and Windows through the complete CLI workflow.
- Produce signed binaries and checksums; document container use.
- Write installation, quick-start, configuration, CI, troubleshooting,
  security, and contributor documentation.
- Define the vulnerability disclosure process and security support policy.
- Select and add an open-source license before announcing the beta.
- Run the complete acceptance suite against secure and vulnerable fixtures.

### Deliverables

- Cross-platform release artifacts.
- Security review and resolved high-severity findings.
- Complete user and contributor documentation.
- Public beta release notes and known limitations.

### Exit gate

All MVP acceptance criteria pass. The tool detects all seeded critical cases,
has no high-confidence false positive in the secure fixture, leaks no test
secret, and blocks all unapproved mutation scenarios. Release artifacts are
reproducible and signed according to the release procedure.

## Phase 6 Expand after beta

Post-beta work should be prioritized using real adoption evidence:

1. Add the GraphQL adapter without changing the core model or adapter contract.
2. Add owner and peer fixture references for BOLA and cross-tenant scenarios.
3. Add setup, attack, verify, and cleanup mutation scenarios.
4. Export SARIF with stable rules and fingerprints.
5. Suggest capability mappings using names and schemas, with explicit human
   approval before configuration changes.
6. Add traffic import, repository analysis, historical trends, and enterprise
   workflows only after the local CLI workflow is stable.

## Requirements to milestone mapping

| Requirement area | Delivery phase |
| --- | --- |
| Configuration, mapping, core model, CLI contract | Phase 1 |
| OpenAPI discovery and REST verification | Phase 2 |
| MCP discovery and verification | Phase 3 |
| Cross-protocol matrix and mismatches | Phase 3 |
| Baseline snapshots, diffs, thresholds, and GitHub Actions | Phase 4 |
| Terminal, JSON, and Markdown reports | Phases 2–4 |
| Safety controls, redaction, and deterministic output | Every phase; final audit in Phase 5 |
| Cross-platform binaries and public documentation | Phase 5 |
| GraphQL, SARIF, assisted mapping, and advanced scenarios | Phase 6 |

## Quality strategy

### Unit tests

Cover configuration validation, environment interpolation, mapping rules,
redaction, policy evaluation, normalization, stable ordering, fingerprints, and
diff classification.

### Contract tests

Run each adapter against valid, malformed, denied, ambiguous, oversized, timed
out, and partial protocol responses. The same adapter contract suite should be
reusable for future protocols.

### Integration tests

Exercise REST and MCP against a disposable reference application with anonymous,
user A, user B, and administrator personas. Seed vertical escalation, BOLA,
field exposure, identity loss, and unknown-result cases.

### End-to-end and golden tests

Run the CLI from discovery through baseline diff on every supported platform.
Review stable terminal, JSON, and Markdown fixtures as code changes modify them.

### Security tests

Continuously scan captured output for known secret markers. Verify double opt-in
for mutations, restrictive artifact permissions, bounded responses, safe path
and URL handling, and dependency advisories.

## First implementation sprint

The first two-week sprint begins after the Phase 0 assumptions are accepted.

### Sprint goal

Produce a compiling workspace that can load and validate a minimal
`paritas.yaml`, represent its policy in protocol-independent types, and print a
deterministic dry-run summary without materializing or exposing secrets.

### Backlog

1. Decide the open-source license, minimum supported Rust version, and crate
   publishing namespace.
2. Create the Cargo workspace and the initial six crates.
3. Configure `rustfmt`, Clippy, tests, documentation, dependency audit, and CI.
4. Define the canonical domain types and adapter traits.
5. Define configuration schema v1 and add valid and invalid fixtures.
6. Implement YAML parsing, environment-reference preservation, and path-aware
   validation errors.
7. Implement centralized redaction and automated secret-marker tests.
8. Add CLI command shells, help text, and exit-code mapping.
9. Add deterministic serialization and initial fingerprint tests.
10. Record architecture decisions and update contributor documentation.

### Sprint acceptance criteria

- A clean checkout passes format, lint, unit-test, and documentation checks.
- A valid example configuration loads into canonical domain types.
- Invalid mappings identify the exact configuration path and corrective action.
- Two equivalent configurations produce deterministic normalized output.
- A known secret marker never appears in logs, errors, reports, or snapshots.
- `paritas verify --dry-run` performs no network or subprocess invocation.

## Definition of done for every issue

An issue is done only when:

- Acceptance criteria are implemented and tested.
- Safety, secret handling, determinism, and partial-failure behavior were
  considered where relevant.
- User-facing behavior includes actionable errors and accessible output.
- Public types and behavior are documented.
- Required schema, fixture, golden output, and architecture records are updated.
- CI passes without ignored failures or unexplained flaky tests.
- No unrelated protocol dependency enters `paritas-core`.

## Decisions required before public beta

The following decisions remain explicit roadmap blockers:

- Select the open-source license and confirm package namespace and trademark use.
- Decide whether GraphQL joins the first public beta or follows immediately
  afterward; it must not block REST + MCP readiness.
- Choose mandatory MCP transports and authentication flows from validation data.
- Define default evidence retention for field-level comparisons.
- Decide whether baselines store observed access, expected policy, or both.
- Define the approved mutation-test workflow for multi-tenant fixtures.
- Assign product, engineering, security review, and release ownership.

## Beta acceptance checklist

- [ ] Installs and runs on Linux, macOS, and Windows.
- [ ] Discovers REST/OpenAPI and MCP inventory without invoking business actions.
- [ ] Validates mappings and reports newly discovered unmapped operations.
- [ ] Runs read-only checks for at least three personas.
- [ ] Preserves protocol-native allow, deny, unknown, and error semantics.
- [ ] Produces an evidence-backed cross-protocol authorization matrix.
- [ ] Saves and compares deterministic baselines.
- [ ] Fails CI at a configurable severity threshold with stable exit codes.
- [ ] Redacts configured secrets from every supported output.
- [ ] Blocks mutations unless both required opt-ins are present.
- [ ] Detects every seeded critical case without a high-confidence finding in the
      secure fixture.
- [ ] Includes a reference application, quick start, CI guide, and contributor
      documentation.
- [ ] Publishes signed release artifacts and checksums.

## Roadmap maintenance

Update this roadmap when a phase gate changes, a dependency alters delivery
order, or validation changes an MVP assumption. Changes to the product promise,
safety defaults, protocol scope, evidence handling, or acceptance criteria also
require a versioned update to the source PRD.
