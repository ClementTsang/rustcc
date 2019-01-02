    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $0, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
