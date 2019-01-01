    .globl    main
    .type main, @function
main:
    movl    $5, %eax
    neg     %eax
    ret
