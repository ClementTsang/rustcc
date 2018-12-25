#!/bin/bash

fail() {
    echo "Failed test ${1}"
}

pass () {
    echo "Passed test ${1}"
}

run_test() {
    cargo run ${1} &> /dev/null
    if [ ${2} -eq 0 ]; then
        if [ "$?" -eq 0 ]; then
            pass ${1}
        else 
            fail ${1}
        fi
    else
        if [ "$?" -ne 0 ]; then
            pass ${1}
        else 
            fail ${1}
        fi
    fi
}

for dir in ./examples/*; do
    for topic in ${dir}/*; do
        for test in "${topic}/*.c"; do
            if [ $dir = *"Passing"* ]; then
                run_test ${test} 0
            else
                run_test ${test} 1
            fi
        done
    done
done
