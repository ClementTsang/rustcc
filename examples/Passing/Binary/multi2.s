    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $4, %eax # Constant integer reference
    neg     %eax # Generating -
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    not     %eax # Generating ~
    push    %eax # Generating binary (+)
    movl    $6, %eax # Constant integer reference
    neg     %eax # Generating -
    pushl   %eax # Generating binary (*)
    movl    $0, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    popl    %ecx
    imul    %ecx, %eax # End *
    pushl   %eax # Generating binary (/)
    movl    $5, %eax # Constant integer reference
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (-)
    movl    $9, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    neg     %eax # Generating -
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    neg     %eax # Generating -
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
