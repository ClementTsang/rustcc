    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $11, %eax # Constant integer reference
    pushl   %eax # Generating binary (/)
    movl    $5, %eax # Constant integer reference
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
