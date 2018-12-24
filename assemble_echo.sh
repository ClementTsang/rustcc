#!/bin/bash

./gcc -m32 ${1}.s -o ${1}
./${1}
echo $?
