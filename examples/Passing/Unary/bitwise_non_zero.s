    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $4, %eax # Constant integer reference
    not     %eax # Generating ~
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
