    .globl    main
    .type main, @function
main:
    movl    $0, %eax
    push    %eax
    movl    $5, %eax
    popl    %ecx
    addl    %ecx, %eax
    ret
