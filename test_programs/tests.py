#!/usr/bin/env python3
# Regression tests: every stage gets its own folder of C programs, so after
# finishing a chapter you can check nothing from earlier stages broke.
#
#   pytest test_programs/tests.py           run everything
#   pytest test_programs/tests.py -k "[local_vars/"  run one folder
#
# gcc is the reference: each program is compiled with both gcc and our
# compiler. If gcc rejects it, ours must too; otherwise both executables
# must exit with the same code.

import shutil
import subprocess
from pathlib import Path

import pytest

cur_folder = Path(__file__).resolve().parent
parent_dir = cur_folder.parent
compiler_path = parent_dir / "compiler_driver.py"

GCC_FLAGS = ["-std=c17", "-pedantic-errors", "-w"]

programs = sorted(str(f.relative_to(cur_folder)) for f in cur_folder.glob("*/*.c"))


@pytest.fixture(scope="session", autouse=True)
def build_compiler():
    # Build once up front rather than inside the first test
    assert compiler_path.exists(), f"Compiler not at expected path: {compiler_path}"
    res = subprocess.run(["cargo", "build"], cwd=parent_dir, capture_output=True, text=True)
    assert res.returncode == 0, res.stderr


def gcc_result(src, tmp_path):
    """Exit code of gcc's executable, or None if gcc rejects the program."""
    exe = tmp_path / "gcc_exe"
    res = subprocess.run(["gcc", *GCC_FLAGS, src, "-o", exe], capture_output=True)
    if res.returncode != 0:
        return None
    return subprocess.run([exe]).returncode


@pytest.mark.parametrize("name", programs)
def test_program(name, tmp_path):
    # Work on a copy so the .i/.s/executable don't end up in the repo
    src = tmp_path / Path(name).name
    shutil.copy(cur_folder / name, src)

    expected = gcc_result(src, tmp_path)

    res = subprocess.run(["python3", compiler_path, src], cwd=parent_dir,
                         capture_output=True, text=True)

    if expected is None:
        assert res.returncode != 0, "gcc rejects this program but our compiler accepted it"
        return

    assert res.returncode == 0, f"compiler failed:\n{res.stdout}\n{res.stderr}"
    run = subprocess.run([src.with_suffix("")])
    assert run.returncode == expected, f"returned {run.returncode}, gcc's returned {expected}"
