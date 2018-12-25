#!/bin/bash

fail() {
    echo "Failed test ${1}"
}

pass () {
    echo "Passed test ${1}"
}

count=1
run_test() {
    cargo run ${1} &> /dev/null
    if [ ${2} -eq 0 ]; then
        if [ "$?" -eq 0 ]; then
            pass ${count}
        else 
            fail ${count}
        fi
    else
        if [ "$?" -ne 0 ]; then
            pass ${count}
        else 
            fail ${count}
        fi
    fi

    count=$((count + 1))
}


echo "Test ${count}: Basic return .c files."
run_test "./examples/ret100.c" 0

echo "Test ${count}: Changing return value."
run_test "./examples/ret42.c" 0

echo "Test ${count}: Missing return value."
run_test "./examples/no_ret.c" 1

echo "Test ${count}: Missing identitfier."
run_test "./examples/no_func_name.c" 1

echo "Test ${count}: Missing semicolon."
run_test "./examples/no_semi.c" 1

echo "Test ${count}: Missing parameter brackets."
run_test "./examples/no_bracket.c" 1

echo "Test ${count}: Missing braces."
run_test "./examples/no_brace.c" 1 


