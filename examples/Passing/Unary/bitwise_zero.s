    .globl    main
    .type main, @function
main:
    movl    $0, %eax
    not     %eax
    ret
