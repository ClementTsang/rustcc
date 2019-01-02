    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $1, %eax # Constant integer reference
    pushl   %eax
    movl    -4(%ebp), %eax # Variable reference
    push    %eax # Generating binary (+)
    movl    $1, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %eax, -4(%ebp) # Assigning new value
    movl    -4(%ebp), %eax # Variable reference
    pushl   %eax # Generating binary (*)
    movl    $2, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
