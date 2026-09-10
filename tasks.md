# monkeynuthead-upnp task plan

## Goal
Build a small Rust UPnP client/server implementation incrementally.

Approach:
- Start with the smallest useful protocol slice.
- Keep each step testable and understandable.
- Add protocol pieces only after the previous step works.
- Prefer clear code and docs over early optimization.

## Known concrete tasks

### Repository and project bootstrap
- [x] Create GitHub repository under georgenicoll named upnp.
- [x] Track work on the master branch.
- [x] Initialize Cargo application named monkeynuthead-upnp.
- [x] Add baseline developer tooling:
  - [x] rustfmt
  - [x] clippy
  - [x] CI checks (format/lint/build/test)
  - [x] dependency and security scans (cargo-audit, cargo-deny)

### Documentation and planning
- [x] Create this tasks.md as the source of truth for planned and completed work.
- [ ] Keep this file updated as we discover new technical tasks.

## Next technical milestones (initial backlog)

### Milestone 1: Minimal networking foundation
- [x] Add a tiny UDP networking layer shared by client and server modules.
- [x] Add config constants for SSDP multicast address/port.
- [x] Add structured logging for packet send/receive events.
- [x] Add smoke test(s) for message parsing helpers.

### Milestone 2: SSDP discovery (simplest UPnP slice)
- [x] Implement a minimal SSDP M-SEARCH sender (client).
- [x] Implement a minimal SSDP responder (server) for one fake device/service.
- [x] Parse essential SSDP headers needed for request/response flow.
- [x] Verify discovery works on local network loop (or local simulation if multicast is restricted).

Milestone 2 implementation notes:
- `src/ssdp/client.rs`: send one M-SEARCH and collect valid SSDP HTTP 200 responses.
- `src/ssdp/server.rs`: receive one request and reply to valid M-SEARCH packets.
- `src/ssdp/message.rs`: build wire-format M-SEARCH and 200 OK responses.
- `src/bin/ssdp-client.rs`: runnable discovery client (supports env-var overrides).
- `src/bin/ssdp-server.rs`: runnable single-request responder (supports env-var overrides).
- Local simulation is validated by the loopback test `client_discovers_local_responder`.

### Milestone 3: Device description endpoint
- [x] Expose a minimal HTTP endpoint that serves a UPnP device description XML.
- [x] Link SSDP LOCATION header to that endpoint.
- [x] Validate client can fetch and parse basic XML fields.

Milestone 3 implementation notes:
- `src/http/device_description.rs`: one-shot HTTP endpoint and XML builder/parser.
- `src/http/client.rs`: fetch a device description over plain HTTP and extract fields.
- `src/bin/device-description-server.rs`: runnable HTTP endpoint for local demo use.
- SSDP LOCATION already points at `http://127.0.0.1:8000/device.xml` by default.

### Milestone 4: Basic control action
- [ ] Add one simple SOAP action endpoint on server.
- [ ] Implement one matching client action call.
- [ ] Handle success and common error responses cleanly.

### Milestone 5: Robustness and quality
- [ ] Expand tests for parser edge cases and protocol compliance basics.
- [ ] Add integration test flow: discover -> fetch description -> invoke action.
- [ ] Add packet capture notes and troubleshooting guide.

## Working rules for this project
- Keep tasks small and shippable.
- Every new protocol feature should include:
  - clear task entry in this file,
  - at least one validation step (manual or automated),
  - notes on assumptions and limitations.
- If a task becomes unclear, split it into smaller tasks before coding.

## Change log for this plan
- 2026-09-10: Initial version created with bootstrap status and first UPnP milestones.
- 2026-09-10: Milestone 1 implemented (shared UDP transport, SSDP constants, packet logging hooks, parser smoke tests).
- 2026-09-10: Milestone 2 implemented (minimal M-SEARCH client, one-shot responder, loopback discovery validation).
- 2026-09-10: Milestone 3 implemented (one-shot HTTP device description endpoint, HTTP client fetch/parser, LOCATION wiring).
