    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl   %eax
    movl    $6, %eax
    popl    %ecx
    imul    %ecx, %eax
    push    %eax
    movl    $8, %eax
    pushl   %eax
    movl    $4, %eax
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx
    pushl   %eax
    movl    $5, %eax
    popl    %ecx
    imul    %ecx, %eax
    popl    %ecx
    addl    %ecx, %eax
    pushl   %eax
    movl    $2, %eax
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax
    neg     %eax
    pushl   %eax
    movl    $3, %eax
    popl    %ecx
    imul    %ecx, %eax
    ret
