# rustcc - a Rust C compiler 
A basic C compiler written in Rust that compiles C code into x86 assembly.  Inspired by [this post by Nora Sandler](https://norasandler.com/2017/11/29/Write-a-Compiler.html).

## Features
Currently, rustcc supports the following features:
* Unary operators (logical negation, bitwise complements, negation)
* Binary operators (basic arithmetic, bitwise operations, comparisons)
* Local variables (assignment, declaration, variable calling, postfix and prefix incrementing)
* If-else branching
* Ternary operator
* While loops, do-while loops, for loops, break, continue
* Function calling and creation

## Installation
To install, ensure beforehand that you have [Rust and Cargo installed.](https://www.rust-lang.org/tools/install)  After that, clone the repository.  Then, run ``cargo build --release``.

## Usage
To use the compiler, run the `rustcc` script as follows:
```
./rustcc /path/to/source.c
```
Upon running, the compiled executable file will be in the same directory and name as the input source file.  The created assembly ``source.s`` file will be deleted upon running the script.

Alternatively, you can directly run ``./target/release/rustcc /path/to/source.c`` (or ``target/release/rustcc.exe /path/to/source.c`` on Windows) to retain the assembly file.

## Disclaimer
rustcc is a project done purely out of personal interest.  The compiled x86 code is most likely not optimized and the possibility of something not working or being supported is quite probable.  I am not responsible for anything going wrong with the use of this.

