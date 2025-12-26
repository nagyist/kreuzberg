#!/usr/bin/env bash
set -euo pipefail

cd packages/python
# maturin sdist reads the manifest-path from pyproject.toml [tool.maturin] section
# The warning about manifest path not existing is benign and can be ignored
uv run maturin sdist --out ../../target/wheels 2>&1 | grep -v "error: manifest path"
