# Paritas

Authorization parity across every interface.

Paritas is an open-source, local-first security tool that verifies whether the
same business capability receives the same authorization treatment across
REST/OpenAPI, GraphQL, and MCP interfaces.

The initial product will be a Rust command-line application and CI integration
with deterministic terminal, JSON, Markdown, and SARIF-compatible reporting.

## Status

Pre-alpha. Product requirements are defined in the
[v0.2 PRD](docs/product/Paritas-Cahier-des-Charges-PRD-v0.2.pdf);
implementation has not started yet.

## Product principles

- Evidence over inference
- Safe and read-only by default
- Explicit capability mapping
- Deterministic, explainable results
- Local-first operation with no telemetry enabled by default

## Planned foundation

Paritas will use Rust Edition 2024 in a Cargo workspace. The protocol-independent
core will be separated from REST/OpenAPI, GraphQL, and MCP adapters.

## License

To be selected before the first public release.
