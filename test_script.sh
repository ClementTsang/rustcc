#!/bin/bash

rust_fail_count=0
rust_total_count=0
compile_fail_count=0
compile_total_count=0
run_fail_count=0
run_total_count=0


fail() {
    echo "${2} test => ${1} ----- FAILED"
}

pass() {
    echo "${2} test => ${1} ----- PASSED"
}

test_compile_run() {
    if [ -e "${1%.*}" ]; then
        rm "${1%.*}"
    fi
   
    compile_total_count=$((compile_total_count+1))

    gcc -m32 ${1} -o "${1%.*}" &> /dev/null
    if [ "$?" -ne 0 ]; then
        fail ${1} "compile"
        compile_fail_count=$((compile_fail_count+1))
        return 101;
    else
        pass ${1} "compile"
    fi

    run_total_count=$((run_total_count+1))
    "./${1%.*}" > /dev/null
    ret_val="$?"
    if [ -e "${1%.*}" ]; then
        rm "${1%.*}"
    fi

    gcc -m32 "${1%.*}.c" -o "${1%.*}" > /dev/null
    "./${1%.*}" > /dev/null

    actual_val="$?"

    if [ -e "${1%.*}" ]; then
        rm "${1%.*}"
    fi
    
    if [ ${actual_val} -eq ${ret_val} ]; then
        pass ${1} "run"
    else
        fail ${1} "run"
        echo "Saw ${ret_val}, wanted ${actual_val}."
        run_fail_count=$((run_fail_count+1))
        return 101;
    fi
}

run_test() {
    if [ -e "${1%.*}.s" ]; then
        rm "${1%.*}.s"
    fi

    cargo run ${1} &> /dev/null

    ret_val="$?"


    if [ ${2} -eq 0 ]; then
        if [ $ret_val -eq 0 ]; then
            pass ${1} "Rust"
        elif [ $ret_val -ne 0 ]; then 
            fail ${1} "Rust"
            rust_fail_count=$((rust_fail_count+1))
        fi
    elif [ ${2} -eq 1 ]; then
        if [ $ret_val -ne 0 ]; then
            pass ${1} "Rust"
        elif [ $ret_val -eq 0 ]; then
            fail ${1} "Rust"
            rust_fail_count=$((rust_fail_count+1))
        fi
    fi

    rust_total_count=$((rust_total_count+1))
}

for dir in ./examples/*; do
    for topic in ${dir}/*; do
        for tests in "${topic}/*.c"; do
            for test in ${tests}; do
                #echo ${test}
                if [[ ${test} = *"Passing"* ]]; then
                    run_test "${test}" 0
                    #echo "SHOULD PASS: $test"
                elif [[ ${test} = *"Failing"* ]]; then
                    run_test "${test}" 1
                    #echo "SHOULD \'FAIL\': $test"
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

echo "Rust: Failed ${rust_fail_count} test cases out of ${rust_total_count}."
echo "Compiling: Failed ${compile_fail_count} test cases out of ${compile_total_count}."
echo "Return result: Failed ${run_fail_count} test cases out of ${run_total_count}."
