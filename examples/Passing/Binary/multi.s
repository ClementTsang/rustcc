    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    pushl   %eax # Generating binary (*)
    movl    $6, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    push    %eax # Generating binary (+)
    movl    $8, %eax # Constant integer reference
    pushl   %eax # Generating binary (/)
    movl    $4, %eax # Constant integer reference
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    pushl   %eax # Generating binary (*)
    movl    $5, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (-)
    movl    $2, %eax # Constant integer reference
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    neg     %eax # Generating -
    pushl   %eax # Generating binary (*)
    movl    $3, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
