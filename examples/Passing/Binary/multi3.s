    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    push    %eax
    movl    $0, %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    neg     %eax
    popl    %ecx
    addl    %ecx, %eax
    pushl   %eax
    movl    $5, %eax
    pushl   %eax
    movl    $7, %eax
    popl    %ecx
    imul    %ecx, %eax
    pushl   %eax
    movl    $0, %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    neg     %eax
    popl    %ecx
    imul    %ecx, %eax
    not     %eax
    neg     %eax
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx
    push    %eax
    movl    $5, %eax
    not     %eax
    neg     %eax
    popl    %ecx
    addl    %ecx, %eax
    ret
