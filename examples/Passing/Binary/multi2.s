    .globl    main
    .type main, @function
main:
    movl    $4, %eax
    neg     %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    not     %eax
    push    %eax
    movl    $6, %eax
    neg     %eax
    pushl   %eax
    movl    $0, %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    popl    %ecx
    imul    %ecx, %eax
    pushl   %eax
    movl    $5, %eax
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx
    popl    %ecx
    addl    %ecx, %eax
    pushl   %eax
    movl    $9, %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax
    neg     %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    neg     %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    ret
