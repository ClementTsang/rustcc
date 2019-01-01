    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl    %eax
    movl    $2, %eax
    popl     %ecx
    cmpl     %eax, %ecx
    movl     %ecx, %eax
    setg     %al
    ret
