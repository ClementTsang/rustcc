    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $0, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $6, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
