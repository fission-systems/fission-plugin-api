# Roadmap

- M0 (bootstrap): versioned protocol, Rust SDK, Python worker, fixture host, real CLI function discovery, three minimal fixture script tests. angr profile has only active-binary knowledge-base lookup.
- M1: native host adapter, stable snapshots, real binary byte reads, disassembly, xrefs, symbols, types, deterministic errors, OS sandbox and timeout.
- M2: IDAPython and Binary Ninja source API expansion; events and read-only analysis conformance.
- M3: Ghidra Java/headless script host and angr CFG/knowledge base integration; pinned target API versions.
- M4: transactional analysis DB writes, annotations, names and types; permission scopes and rollback.
- M5: explicit IR mappings and hook lifecycle; differential semantic checking.
- M6: emulator/solver state integration, advanced plugin support, published compatibility benchmark.

## Release gates

1. Reproducible synthetic API conformance on every platform.
2. Real Fission CLI integration with pinned version and test binaries.
3. Licensed third-party plugin corpus with observable expected behavior; compare against upstream where lawful and available.
4. Separate counts for supported, unsupported, mismatch, crash, timeout and infrastructure failure. Never omit failures from denominator.
5. Document source compatibility, API coverage and semantic compatibility independently.
