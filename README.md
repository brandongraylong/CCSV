# CCSV - **Still Very Much a WIP**
CSV library. Written in Rust. Statically linked C lib available with C++ compatability.

## About
Simple implementation based off of [RFC4180](https://datatracker.ietf.org/doc/html/rfc4180).
This is a hobby project, mostly for learning how to work with FFI bindings.
Currently ignores non-header rows that are not the exact length as the number of header columns.
Currently just parsing strings for cells in non-header rows.

## Prerequisites
- Make
- GCC
- Rust Toolchain
- Valgrind

## Directory Structure
### bin
A place to hold your executables.

### examples
Examples you can run.

### include
Generated C header file (with C++ compatability).

### lib
Compiled library to be statically linked.

### src
Rust library source.

### target
Rust generated output.

## Sample Input - *./data/examples/input.csv*
```csv
Header 1, Header2, "Header 3", "Header4" , Header 5
0, 1, 2, 3, 4
5, 6, 7, 8, 9
1, 2, 3
```

## Building and Running
### C
- `make example_build_c`
- `make example_run_c`
- `make example_valgrind_c`

### C++
- `make example_build_c++`
- `make example_run_c++`
- `make example_valgrind_c++`

## TODO
- Better documentation
- Better method to run examples (e.g. Docker containers)
- Windows support and documentation
- Shared library generation & more language examples
- Typecasting for cells in non-header rows
- Unit tests
- And more
