#!/bin/bash

cargo run ${1} &> /dev/null
gcc -m32 "${1%.*}.c" -o "${1%.*}" > /dev/null
