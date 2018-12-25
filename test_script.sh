#!/bin/bash

fail() {
    echo "Failed ${2} test ${1}"
}

pass() {
    echo "Passed ${2} test ${1}"
}

test_compile_run() {
    gcc -m32 ${1} -o "${1%.*}" &> /dev/null
    if [ "$?" -ne 0 ]; then
        fail ${1} "compile"
    else
        pass ${1} "compile"
    fi

    ${1%.*} > /dev/null
    ret_val="$?"
    rm ${1%.*}

    gcc -m32 "${1%.*}.c" -o "${1%.*}" > /dev/null
    ${1%.*} > /dev/null

    if [ "$?" -eq ${ret_val} ]; then
        pass ${1} "run"
    else
        fail ${1} "run"
    fi
}

run_test() {
    cargo run ${1} &> /dev/null
    if [ ${2} -eq 0 ]; then
        if [ "$?" -eq 0 ]; then
            pass ${1} "Rust"
        else 
            fail ${1} "Rust"
        fi
    else
        if [ "$?" -ne 0 ]; then
            pass ${1} "Rust"
        else 
            fail ${1} "Rust"
        fi
    fi
}

for dir in ./examples/*; do
    for topic in ${dir}/*; do
        for tests in "${topic}/*.c"; do
            for test in ${tests}; do
                #echo ${test}
                if [ $dir = *"Passing"* ]; then
                    run_test "${test}" 0
                else
                    run_test "${test}" 1
                fi
            done
        done
    done
done

for dir in ./examples/Passing/*; do
    for topic in "${dir}/*.s"; do
        for test in ${topic}; do
            test_compile_run "${test}"
            
        done
    done
done
