#!/usr/bin/env python3

import subprocess
import argparse
from pathlib import Path

parser = argparse.ArgumentParser(
    prog="CompilerDriver",
    description="Does everything")

parser.add_argument("filename")
parser.add_argument("--lex", action="store_true")
parser.add_argument("--parse", action="store_true")
parser.add_argument("--codegen", action="store_true")
parser.add_argument("-S", action="store_true")

args = parser.parse_args()

file_name = args.filename
if not file_name:
    raise Exception("You must provide the path to a C program")
else:
    f = Path(file_name)
    if not f.exists():
        raise Exception("The C file provided does not exists")

file_name = args.filename
preprocess_name = file_name[0:-1] + "i"

subprocess.run(["gcc", "-E", "-P", file_name, "-o", preprocess_name])

assembly_name = file_name[0:-1] + "s"
# pass # Our compiler goes here!
if args.lex:
    res = subprocess.run(["cargo", "run", "--", "--fname", preprocess_name, "--lex"])
    exit(res.returncode)
elif args.parse:
    res = subprocess.run(["cargo", "run", "--", "--fname", preprocess_name, "--parse"])
    exit(res.returncode)
elif args.codegen:
    res = subprocess.run(["cargo", "run", "--", "--fname", preprocess_name, "--codegen"])
    exit(res.returncode)
elif args.S:
    res = subprocess.run(["cargo", "run", "--", "--fname", preprocess_name])
    exit(res.returncode)
else:
    res = subprocess.run(["cargo", "run", "--", "--fname", preprocess_name])
    if res.returncode != 0:
        exit(res.returncode)

    subprocess.run(['rm', preprocess_name])
    executable_name = file_name[0:-2]
    subprocess.run(['gcc', assembly_name, '-o', executable_name])
    subprocess.run(['rm', assembly_name])

