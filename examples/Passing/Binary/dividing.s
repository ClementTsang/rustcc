    .globl    main
    .type main, @function
main:
    movl    $11, %eax
    pushl   %eax
    movl    $5, %eax
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx
    ret
