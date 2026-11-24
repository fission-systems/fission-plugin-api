# Wire protocol v0 (experimental)

One UTF-8 JSON request per line on stdin, one response per line on stdout. Logs to stderr. Synchronous single-client transport; malformed JSON terminates the reference host. A production host must bound frame sizes, support cancellation and apply resource budgets.

Request:
```json
{"version":0,"id":0,"method":"hello","params":{}}
```

Response:
```json
{"version":0,"id":0,"result":{"backend":"fixture","capabilities":["functions.list","memory.read"]}}
```

Error:
```json
{"version":0,"id":1,"error":{"code":"unsupported","message":"unknown method example"}}
```

Exactly one result/error; version and ID echo the supported protocol and request ID. Addresses use canonical lowercase hexadecimal strings with no leading zeroes (except 0x0). Sizes are nonnegative integers. No floating-point address conversion.

| Method | Parameters | Result |
| --- | --- | --- |
| hello | {} | backend, capabilities |
| functions.list | {} | array of address/name/size records |
| memory.read | address, size (max 1 MiB) | byte array; wholly mapped range or error |

Error codes: version_mismatch, invalid_params, unmapped, unsupported, backend, transport (client-side). Clients must negotiate capability before use; hosts still enforce it. No arbitrary execution or mutation RPC.
