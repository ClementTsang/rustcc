    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $2, %eax # Constant integer reference
    pushl   %eax
    movl    $0, %eax # Default return value
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
