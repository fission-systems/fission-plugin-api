"""Experimental Python client; no third-party dependencies."""
import json
import subprocess

class ProtocolError(RuntimeError):
    def __init__(self, code, message):
        super().__init__(f"{code}: {message}")
        self.code = code

class Client:
    def __init__(self, command):
        self.process = subprocess.Popen(command, stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, encoding="utf-8")
        self.next_id = 0
        try:
            self.hello = self.call("hello")
            self.capabilities = frozenset(self.hello["capabilities"])
        except BaseException:
            self.close()
            raise

    def call(self, method, **params):
        ident = self.next_id
        self.next_id += 1
        self.process.stdin.write(json.dumps(dict(version=0, id=ident, method=method, params=params)) + "\n")
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        if not line:
            raise ProtocolError("transport", "host closed its output")
        response = json.loads(line)
        if response.get("id") != ident or response.get("version") != 0:
            raise ProtocolError("transport", "response mismatch")
        if "error" in response:
            error = response["error"]
            raise ProtocolError(error["code"], error["message"])
        return response["result"]

    def require(self, capability):
        if capability not in self.capabilities:
            raise ProtocolError("unsupported", capability)

    def functions(self):
        self.require("functions.list")
        return self.call("functions.list")

    def read(self, address, size):
        self.require("memory.read")
        return bytes(self.call("memory.read", address=hex(address), size=size))

    def close(self):
        try:
            self.process.stdin.close()
            self.process.wait(timeout=2)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait()
        finally:
            self.process.stdout.close()

    def __enter__(self):
        return self

    def __exit__(self, *exc):
        self.close()
