    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $1, %eax # Constant integer reference
    neg     %eax # Generating -
    not     %eax # Generating ~
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    neg     %eax # Generating -
    not     %eax # Generating ~
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
