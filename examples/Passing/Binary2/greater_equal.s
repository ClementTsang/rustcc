    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl    %eax
    movl    $5, %eax
    popl     %ecx
    cmpl     %eax, %ecx
    movl     %ecx, %eax
    setge    %al
    ret
