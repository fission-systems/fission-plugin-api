import os
from pathlib import Path
import subprocess
import sys
import unittest
from fission_plugin_api import Client, ProtocolError

ROOT = Path(__file__).resolve().parents[1]
HOST = os.environ.get("FISSION_PLUGIN_HOST", str(ROOT / "target/debug" / ("fission-plugin-host.exe" if os.name == "nt" else "fission-plugin-host")))
FIXTURE = str(ROOT / "fixtures/minimal.json")

class Conformance(unittest.TestCase):
    def test_host(self):
        with Client([HOST, "--fixture", FIXTURE]) as client:
            self.assertEqual(client.hello["backend"], "fixture")
            self.assertEqual([f["name"] for f in client.functions()], ["main", "helper"])
            self.assertEqual(client.read(0x401000, 2), bytes([144, 195]))
            for method, params, code in [
                ("memory.read", {"address": "0x401003", "size": 2}, "unmapped"),
                ("memory.read", {"address": "0x401000", "size": 1048577}, "invalid_params"),
                ("analysis.fake", {}, "unsupported"),
            ]:
                with self.assertRaises(ProtocolError) as cm:
                    client.call(method, **params)
                self.assertEqual(cm.exception.code, code)

    def test_scripts(self):
        for profile, script, expected in [
            ("ida", "ida_functions.py", "0x401000 main"),
            ("binaryninja", "bn_functions.py", "0x401002 helper"),
            ("ghidra", "ghidra_functions.py", "401000 main"),
        ]:
            with self.subTest(profile=profile):
                env = dict(os.environ, PYTHONPATH=str(ROOT / "python"), FISSION_PLUGIN_HOST=HOST)
                out = subprocess.check_output([sys.executable, "-m", "fission_plugin_api",
                    "--fixture", FIXTURE, "--profile", profile, str(ROOT / "examples" / script)],
                    env=env, text=True)
                self.assertIn(expected, out)

if __name__ == "__main__":
    unittest.main()
