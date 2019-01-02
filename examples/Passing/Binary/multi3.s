    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $0, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    neg     %eax # Generating -
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (/)
    movl    $5, %eax # Constant integer reference
    pushl   %eax # Generating binary (*)
    movl    $7, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    pushl   %eax # Generating binary (*)
    movl    $0, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    neg     %eax # Generating -
    popl    %ecx
    imul    %ecx, %eax # End *
    not     %eax # Generating ~
    neg     %eax # Generating -
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    push    %eax # Generating binary (+)
    movl    $5, %eax # Constant integer reference
    not     %eax # Generating ~
    neg     %eax # Generating -
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
