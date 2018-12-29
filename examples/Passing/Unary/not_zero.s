    .globl    main
    .type main, @function
main:
    movl    $0, %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete    %al
    ret
