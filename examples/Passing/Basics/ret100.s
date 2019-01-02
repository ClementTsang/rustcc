    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $100, %eax # Constant integer reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
