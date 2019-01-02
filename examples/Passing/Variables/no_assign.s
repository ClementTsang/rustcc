    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $0, %eax # Constant integer reference
    pushl   %eax
    movl    $0, %eax # Constant integer reference
    pushl   %eax
    movl    $0, %eax # Constant integer reference
    pushl   %eax
    movl    $5, %eax # Constant integer reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
