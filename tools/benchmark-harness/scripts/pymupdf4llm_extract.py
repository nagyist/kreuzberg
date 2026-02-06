# /// script
# requires-python = ">=3.10"
# dependencies = [
#     "pymupdf4llm>=0.0.17",
# ]
# ///
"""PyMuPDF4LLM extraction wrapper for benchmark harness."""

from __future__ import annotations

import json
import os
import sys
import time

import pymupdf4llm

# Suppress MuPDF C-level error/warning messages that get written directly to
# stdout, which corrupts the persistent server's line-based JSON protocol.
# See: https://github.com/pymupdf/PyMuPDF/issues/606
import pymupdf
pymupdf.TOOLS.mupdf_display_errors(False)

# As an extra safety net, redirect the C-level stdout (fd 1) to stderr (fd 2)
# during extraction so any remaining C library output goes to stderr.
_original_stdout_fd = os.dup(1)
_stderr_fd = os.dup(2)


def _redirect_c_stdout_to_stderr():
    """Redirect C-level fd 1 to fd 2 so MuPDF noise goes to stderr."""
    sys.stdout.flush()
    os.dup2(_stderr_fd, 1)


def _restore_c_stdout():
    """Restore C-level fd 1 for our JSON output."""
    sys.stdout.flush()
    os.dup2(_original_stdout_fd, 1)


def extract_sync(file_path: str) -> dict:
    """Extract using PyMuPDF4LLM."""
    start = time.perf_counter()
    markdown = pymupdf4llm.to_markdown(file_path, show_progress=False, write_images=False)
    duration_ms = (time.perf_counter() - start) * 1000.0

    return {
        "content": markdown,
        "metadata": {"framework": "pymupdf4llm"},
        "_extraction_time_ms": duration_ms,
    }


def run_server() -> None:
    """Persistent server mode."""
    for line in sys.stdin:
        file_path = line.strip()
        if not file_path:
            continue
        try:
            # Redirect C-level stdout to stderr during extraction to prevent
            # MuPDF C library noise from corrupting our JSON protocol.
            _redirect_c_stdout_to_stderr()
            payload = extract_sync(file_path)
            _restore_c_stdout()
            print(json.dumps(payload), flush=True)
        except Exception as e:
            _restore_c_stdout()
            print(json.dumps({"error": str(e), "_extraction_time_ms": 0}), flush=True)


def main() -> None:
    ocr_enabled = False
    args = []
    for arg in sys.argv[1:]:
        if arg == "--ocr":
            ocr_enabled = True
        elif arg == "--no-ocr":
            ocr_enabled = False
        else:
            args.append(arg)

    if len(args) < 1:
        print("Usage: pymupdf4llm_extract.py [--ocr|--no-ocr] <mode> <file_path>", file=sys.stderr)
        print("Modes: sync, server", file=sys.stderr)
        sys.exit(1)

    mode = args[0]
    if mode == "server":
        run_server()
    elif mode == "sync":
        if len(args) < 2:
            print("Error: sync mode requires a file path", file=sys.stderr)
            sys.exit(1)
        file_path = args[1]
        try:
            payload = extract_sync(file_path)
            print(json.dumps(payload), end="")
        except Exception as e:
            print(f"Error extracting with PyMuPDF4LLM: {e}", file=sys.stderr)
            sys.exit(1)
    else:
        # Legacy fallback for direct file path
        try:
            payload = extract_sync(args[0])
            print(json.dumps(payload), end="")
        except Exception as e:
            print(f"Error extracting with PyMuPDF4LLM: {e}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()
