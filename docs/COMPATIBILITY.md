# Compatibility policy

Each profile targets documented public source APIs, pinned to explicit versions before release. Current version coverage is not yet measured. No native ABI compatibility claim.

| Target | Priority interfaces | Conformance criteria |
| --- | --- | --- |
| IDA | idautils, idc, ida_bytes, ida_funcs, ida_xref, ida_name | addresses, names, xrefs, byte and sentinel semantics |
| Ghidra | GhidraScript, FlatProgramAPI, Program, Address, FunctionManager | address spaces, iterators, transactions; separate JVM runner |
| Binary Ninja | BinaryView, Function, Symbol, Type, PluginCommand | object lifetime, IL mapping, analysis lifecycle |
| angr | Project, KnowledgeBase, CFG, SimStatePlugin, SimProcedure | CFG, state and solver semantics; do not claim unsupported analysis |

Public API references:
- https://python.docs.hex-rays.com/
- https://ghidra.re/ghidra_docs/api/
- https://api.binary.ninja/
- https://docs.angr.io/

Report each plugin with source URL, license, upstream revision, profile version, required capabilities, run command, and outcome. Outcomes: pass, mismatch, unsupported, crash, timeout, infrastructure failure. A script importing successfully does not count as a pass.
