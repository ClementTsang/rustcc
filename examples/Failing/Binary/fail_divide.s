    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    pushl   %eax
    movl    $0, %eax
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx
    ret
