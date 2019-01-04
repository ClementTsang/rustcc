#!/bin/bash

if [[ "$#" -ne 1 ]]; then
    echo "Invalid number of arguments."
fi

./target/release/rustcc ${1} &> /dev/null

if [[ $? != 0 ]]; then
    echo "Failed to properly produce a file."
    exit 101
fi

gcc -m32 "${1%.*}.s" -o "${1%.*}"
rm "${1%.*}.s"

