# Architecture

## ADR-0001: compatibility above a stable semantic host boundary

Fission's internal PluginAPI currently exposes binary metadata, functions, memory, decompilation and disassembly as Rust traits. It is not a stable cross-language ABI. Keep the core authoritative; do not expose Rust trait object layout or pin consumers to internal IR structures.

- Protocol: typed values, version, request IDs, capability discovery, explicit failures.
- Native SDK: Rust transport and typed data; Python SDK and future Java binding share the wire contract.
- Host: fixture for reproducible conformance; CLI bridge for initial actual analysis; eventually an in-process Fission adapter with immutable snapshots and transactions.
- Profiles: distinct source-compatible namespaces and object models. Never map one tool's IR to another by pretending the semantics are identical.
- Worker: separate process for failure containment. OS-level sandboxing, resource limits, timeouts and permission enforcement are future work. Trusted scripts only for this bootstrap.

## Semantic boundaries

Use stable address-space-qualified handles in the future (v0 only supports a single flat 64-bit VA space). Distinguish file bytes, mapped bytes and runtime memory. Distinguish not-found from unsupported. Future mutations require a session revision, optimistic concurrency and atomic commit/rollback. Events require ordering and cancellation semantics.

For decompiler IR, represent provenance and loss explicitly. Hex-Rays microcode/CTree, Ghidra P-code/HighFunction, BN IL and angr AIL are different contracts; an adapter must document unsupported constructs. Do not synthesize a plausible but wrong analysis result.

## Integration

Current CLI bridge uses the existing Fission v1 envelope and FunctionsResponse. The native host should wrap fission-plugin contracts and fission-analysis-db rather than fork analysis implementations. Avoid a dependency cycle: shared protocol types are independent of the Fission engine.

## Security / licensing

Use independently authored shims against documented public interfaces. Do not redistribute proprietary SDK binaries, headers or plugin code without permission. Each plugin retains its license. Native binary plugins and GUI-dependent plugins are out of scope for v0. No product license bypass is intended.
