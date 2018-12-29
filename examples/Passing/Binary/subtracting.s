    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl   %eax
    movl    $6, %eax
    neg     %eax
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax
    ret
