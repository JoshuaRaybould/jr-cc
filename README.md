# jr-cc

A compiler for a subset of C, written from scratch in Rust. It compiles C
source to x86-64 assembly, then uses gcc to assemble and link it into an
executable.

The compiler has a full multi-stage pipeline: a hand-written lexer, a
recursive-descent parser, semantic analysis, an intermediate representation and
x86-64 code generation. No parser generators or compiler libraries are used.

The design follows [*Writing a C Compiler*](https://norasandler.com/book/) by
Nora Sandler.

## What it supports

Programs are a single `int main(void)` function working with `int` values,
using:

| Feature | Example |
| --- | --- |
| Integer constants and `return` | `return 2;` |
| Unary operators | `-x`, `~x`, `!x` |
| Arithmetic operators | `+ - * / %` |
| Bitwise operators | `& \| ^ << >>` |
| Relational and equality operators | `== != < > <= >=` |
| Logical operators, with short-circuiting | `&& \|\|` |
| Local variables and assignment | `int a = 1; a = b = a * 2;` |
| Expression and null statements | `a + 1;`, `;` |

Operators follow C's precedence and associativity rules, and `main` returns 0
if it reaches the end without a `return`.

Invalid programs are rejected, including syntax errors, use of undeclared
variables, duplicate declarations and assignment to something that isn't a
variable (`2 = a`).

Not supported yet: functions other than `main`, control flow (`if`, loops),
compound assignment (`+=`), increment/decrement (`++`, `--`) and types other
than `int`.

## Requirements

- Linux on x86-64
- Rust 1.85 or newer, with Cargo
- Python 3
- gcc (used to preprocess the source and to assemble and link the output)
- pytest (only needed to run the tests)

## Usage

Run `compiler_driver.py` from the project root. It builds and runs the
compiler with `cargo run`, so it won't work from another directory.

```sh
./compiler_driver.py path/to/program.c
./path/to/program
echo $?   # the value main returned
```

The source is preprocessed with `gcc -E`, compiled to assembly by jr-cc, then
assembled and linked by gcc. The executable is written next to the source file,
with the same name minus the `.c`.

### Stopping early

Each stage can be run on its own, which helps when debugging. These options
stop after the named stage and don't produce an executable:

| Option | Stops after | Prints |
| --- | --- | --- |
| `--lex` | Lexing | Nothing; succeeds if the source is made of valid tokens |
| `--parse` | Parsing | The AST |
| `--validate` | Semantic analysis | The AST with every variable renamed to a unique name |
| `--tacky` | Intermediate representation | The validated AST, then the TACKY IR |
| `--codegen` | Assembly generation | The assembly AST |
| `-S` | Code emission | Nothing; writes the `.s` assembly file next to the source |

If the program is invalid, the compiler exits with a non-zero status.

## How it works

```
source.c ─► preprocess ─► lex ─► parse ─► semantic analysis ─► TACKY ─► assembly ─► emit ─► gcc ─► executable
```

| Stage | File | What it does |
| --- | --- | --- |
| Lexer | `src/lexer.rs` | Turns source text into tokens |
| Parser | `src/parser.rs`, `src/parser/` | Builds the C AST using recursive descent, with precedence climbing for binary operators |
| Semantic analysis | `src/semantic_analysis.rs` | Gives each variable a unique name and reports undeclared variables, duplicate declarations and invalid assignment targets |
| TACKY | `src/tacky.rs`, `src/tacky/` | Lowers the AST to TACKY, a three-address-code intermediate representation |
| Assembly generation | `src/assembly.rs`, `src/assembly/` | Converts TACKY to an assembly AST, gives each variable and temporary a stack slot, and rewrites instructions that x86-64 can't encode directly |
| Code emission | `src/emit.rs` | Writes AT&T-syntax assembly to a `.s` file |

## Tests

The tests live in `test_programs/`, with one folder of C programs for each
stage of development:

```
test_programs/
├── tests.py
└── local_vars/   operators, local variables and assignment
```

Every program is also compiled with gcc, which is used as the reference:

- if gcc compiles it, jr-cc must compile it too, and both executables must
  exit with the same code
- if gcc rejects it, jr-cc must reject it too (these files start with
  `invalid_`)

Run the tests from the project root:

```sh
pytest test_programs/tests.py                     # everything
pytest test_programs/tests.py -k "[local_vars/"   # one folder
pytest test_programs/tests.py -k add_sub          # tests with "add_sub" in the name
```

Each test is named after its file, e.g. `test_program[local_vars/add_sub.c]`, and
`-k` runs the tests whose name contains the given text. Include the `[` and `/`
to match exactly one folder, and not every test that happens to contain that
text, such as a folder whose name ends the same way.

When a new feature is added, its tests go in a new folder in `test_programs/`.
New `.c` files are picked up automatically, and the earlier folders keep
running, so regressions in older features are caught.
