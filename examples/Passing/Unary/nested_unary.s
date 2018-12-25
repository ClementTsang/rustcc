    .globl    main
    .type main, @function
main:
    movl    $1, %eax
    neg    %eax
    not    %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete   %al
    neg    %eax
    not    %eax
    cmpl    $0, %eax
    movl    $0, %eax
    sete   %al
    ret
