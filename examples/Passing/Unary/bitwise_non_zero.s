    .globl    main
    .type main, @function
main:
    movl    $4, %eax
    not     %eax
    ret
