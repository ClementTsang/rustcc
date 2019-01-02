    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    neg     %eax # Generating -
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
