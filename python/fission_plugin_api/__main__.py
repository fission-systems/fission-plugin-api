import argparse
import os
import runpy
import sys
from pathlib import Path
from . import Client
from .compat import install

def main():
    parser = argparse.ArgumentParser(description="Run a trusted script with a Fission compatibility profile")
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--fixture")
    mode.add_argument("--binary")
    parser.add_argument("--profile", choices=["ida", "binaryninja", "ghidra", "angr"], required=True)
    parser.add_argument("script")
    parser.add_argument("script_args", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    default = root / "target" / "debug" / ("fission-plugin-host.exe" if os.name == "nt" else "fission-plugin-host")
    host = os.environ.get("FISSION_PLUGIN_HOST", str(default))
    command = [host, "--fixture", args.fixture] if args.fixture else [host, "--binary", args.binary]
    with Client(command) as client:
        globals_ = install(args.profile, client, args.binary)
        sys.argv = [args.script, *args.script_args]
        sys.path.insert(0, str(Path(args.script).resolve().parent))
        runpy.run_path(args.script, init_globals=globals_, run_name="__main__")

if __name__ == "__main__":
    main()
