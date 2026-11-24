# Fission Plugin API

A Rust-first, headless plugin SDK and cross-tool compatibility runtime for [Fission](https://github.com/fission-systems/Fission).

**Status: experimental bootstrap (protocol v0).** The goal is to run existing IDA Pro, Ghidra, Binary Ninja and angr scripts with minimal modification, without requiring those products for supported operations. This is not full compatibility or binary ABI emulation.

## Architecture

```text
Existing scripts / native SDK clients
          |             |
 IDA / Ghidra / BN / angr profiles
          |
 Versioned JSON-lines protocol + capabilities
          |
 Rust host (fixture or Fission CLI)
          |
 Fission analysis engine
```

The host uses Fission's existing `--contract v1 list --json` for real function discovery; fixture mode additionally supports mapped byte reads. All other capabilities must be negotiated. No silent stubs, invented IR, or fabricated analysis results.

## Quick start

```sh
cargo build --workspace
PYTHONPATH=python python3 -m fission_plugin_api --fixture fixtures/minimal.json --profile ida examples/ida_functions.py
PYTHONPATH=python python3 -m fission_plugin_api --binary ./target.bin --profile ida examples/ida_functions.py
```

Set `FISSION_PLUGIN_HOST` to the built host executable if it is not at `target/debug/fission-plugin-host`. Set `FISSION_CLI` to the Fission CLI executable (default: `fission_cli`).

Run only trusted scripts: the worker is a separate process but **not a security sandbox**. No plugin code is downloaded or executed automatically.

## Current coverage

| Profile | Experimental implementation | Not implemented |
| --- | --- | --- |
| IDA Python | idautils.Functions, idc.get_func_name, ida_bytes.get_bytes (fixture only) | plugin lifecycle, Hex-Rays, Qt, native modules |
| Binary Ninja | BinaryView.functions, get_function_at, read (fixture only) | IL, analysis completion, UI, native modules |
| Ghidra | Python script globals: currentProgram, getFirstFunction, getFunctionAt, toAddr | JVM/Java APIs, transactions, analyzers, HighFunction |
| angr | Project(binary), kb.functions lookup | CFG analyses, CLE, SimState, SimProcedure, AIL |

These are small source-level compatibility subsets; real third-party plugin compatibility has not been measured. Unsupported operations raise errors rather than masquerading as successful.

## Development

```sh
cargo test --workspace
cargo build --workspace
PYTHONPATH=python python3 -m unittest discover -s tests -v
```

- [Architecture](docs/ARCHITECTURE.md)
- [Wire protocol](docs/PROTOCOL.md)
- [Roadmap and conformance gates](docs/ROADMAP.md)
- [Compatibility matrix](docs/COMPATIBILITY.md)

This repository is separate from Fission's internal `fission-plugin` crate and the pseudocode post-processing `fission-plugins` project. License: Apache-2.0. Third-party plugins retain their own licenses.
