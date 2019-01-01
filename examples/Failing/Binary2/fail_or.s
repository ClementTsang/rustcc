    .globl    main
    .type main, @function
main:
    push    %eax
    movl    $6, %eax
    pop     %ecx
    orl     %ecx, %eax
    movl    $0, %eax
    setne   %al
    ret
