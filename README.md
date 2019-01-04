# rustcc - a Rust C compiler 
A basic C compiler written in Rust.  Inspired by [this post by Nora Sandler](https://norasandler.com/2017/11/29/Write-a-Compiler.html).

## Features
Currently, rustcc supports the following features:
* Unary operators (logical negation, bitwise complements, negation)
* Binary operators (basic arithmetic, bitwise operations, comparisons)
* Local variables (assignment, declaration, variable calling, postfix and prefix incrementing)

## Installation
To install, ensure beforehand that you have Rust and Cargo installed.  After that, clone the repository.  Then, run ``cargo build --release``.

## Usage
To use the compiler, run the `rustcc` script as follows:
```
./rustcc /path/to/source.c
```
Upon running, the compiled executable file will be in the same directory and name as the input source file.  The created assembly ``source.s`` file will be deleted upon running the script.

Alternatively, you can directly run ``./target/release/rustcc /path/to/source.c`` (or ``target/release/rustcc.exe /path/to/source.c`` on Windows) to retain the assembly file.
