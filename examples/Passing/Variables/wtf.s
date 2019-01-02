    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $1, %eax # Constant integer reference
    pushl   %eax
    movl    $6, %eax # Constant integer reference
    movl    %eax, -4(%ebp) # Assigning new value
    pushl   %eax
    movl    -4(%ebp), %eax # Variable reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
