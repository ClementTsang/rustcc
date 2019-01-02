    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $2, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $2, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    $3, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $3, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    $4, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $4, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    $5, %eax # Constant integer reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
