"""Small, explicitly scoped source-compatibility profiles."""
import sys
from types import ModuleType, SimpleNamespace
from . import ProtocolError

class Function:
    def __init__(self, row):
        self.start = self.addr = int(row["address"], 16)
        self.name = row["name"]
        self.size = row["size"]

    def getName(self):
        return self.name

    def getEntryPoint(self):
        return Address(self.start)

class Address:
    def __init__(self, value):
        self.value = value

    def getOffset(self):
        return self.value

    def __str__(self):
        return format(self.value, "x")

def install(profile, client, binary=None):
    def functions():
        return sorted((Function(row) for row in client.functions()), key=lambda f: f.start)

    def at(addr):
        return next((f for f in functions() if f.start == addr), None)

    def module(name, **attrs):
        if name in sys.modules:
            raise ProtocolError("profile_conflict", name)
        obj = ModuleType(name)
        obj.__dict__.update(attrs)
        sys.modules[name] = obj
        return obj

    if profile == "ida":
        def addresses(start=None, end=None):
            return (f.start for f in functions()
                    if (start is None or f.start >= start) and (end is None or f.start < end))

        def name(addr):
            f = next((f for f in functions() if f.start <= addr < f.start + max(f.size, 1)), None)
            return f.name if f else ""

        def get_bytes(addr, size):
            try:
                return client.read(addr, size)
            except ProtocolError as exc:
                if exc.code == "unmapped":
                    return None
                raise

        module("idautils", Functions=addresses)
        module("idc", get_func_name=name)
        module("ida_bytes", get_bytes=get_bytes)
        module("idaapi", BADADDR=(1 << 64) - 1)
        return {}

    if profile == "binaryninja":
        class BinaryView:
            @property
            def functions(self):
                return functions()

            def get_function_at(self, addr):
                return at(addr)

            def read(self, addr, size):
                return client.read(addr, size)

        module("binaryninja", BinaryView=BinaryView)
        return {"bv": BinaryView()}

    if profile == "ghidra":
        class Manager:
            def getFunctionAt(self, addr):
                return at(addr.getOffset())

        def first():
            return next(iter(functions()), None)

        manager = Manager()
        program = SimpleNamespace(getFunctionManager=lambda: manager)
        return {
            "currentProgram": program,
            "toAddr": lambda value: Address(int(value, 0) if isinstance(value, str) else int(value)),
            "getFirstFunction": first,
            "getFunctionAt": manager.getFunctionAt,
        }

    if profile == "angr":
        from pathlib import Path
        class Project:
            def __init__(self, filename, **kwargs):
                if binary is None or Path(filename).resolve() != Path(binary).resolve():
                    raise ProtocolError("unsupported", "Project must refer to the active binary")
                if kwargs:
                    raise ProtocolError("unsupported", "Project options are not implemented")
                self.kb = SimpleNamespace(functions={f.addr: f for f in functions()})

        module("angr", Project=Project)
        return {}

    raise ProtocolError("unsupported", profile)
