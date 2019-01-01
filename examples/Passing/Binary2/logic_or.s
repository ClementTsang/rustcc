    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    push    %eax
    movl    $0, %eax
    pop     %ecx
    orl     %ecx, %eax
    movl    $0, %eax
    setne   %al
    ret
