#!/bin/bash

fail() {
    echo "Failed test ${1}"
}

pass () {
    echo "Passed test ${1}"
}

run_test() {
    cargo run ${1} &> /dev/null
    #cargo run ${1} 
    if [ ${3} -eq 0 ]; then
        if [ "$?" -eq 0 ]; then
            pass ${2}
        else 
            fail ${2}
        fi
    else
        if [ "$?" -ne 0 ]; then
            pass ${2}
        else 
            fail ${2}
        fi
    fi
}

count=1

echo "Test ${count}: Basic return .c files."
run_test "./examples/ret100.c" ${count} 0
count=$((count + 1))

echo "Test ${count}: Changing return value."
run_test "./examples/ret42.c" ${count} 0
count=$((count + 1))

echo "Test ${count}: Missing return value."
run_test "./examples/no_ret.c" ${count} 1
count=$((count + 1))

echo "Test ${count}: Missing identitfier."
run_test "./examples/no_func_name.c" ${count} 1
count=$((count + 1))

echo "Test ${count}: Missing semicolon."
run_test "./examples/no_semi.c" ${count} 1
count=$((count + 1))

echo "Test ${count}: Missing parameter brackets."
run_test "./examples/no_bracket.c" ${count} 1
count=$((count + 1))

echo "Test ${count}: Missing braces."
run_test "./examples/no_brace.c" ${count} 1 
count=$((count + 1))


