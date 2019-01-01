    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl   %eax
    movl    $0, %eax
    popl    %ecx
    cmpl    $0, %ecx
    setne   %cl
    cmpl    $0, %eax
    movl    $0, %eax
    setne   %al
    andb    %cl, %al
    ret
