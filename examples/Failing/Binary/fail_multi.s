    .globl    main
    .type main, @function
main:
    pushl   %eax
    movl    $5, %eax
    popl    %ecx
    imul    %ecx, %eax
    ret
