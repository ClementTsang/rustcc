    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    pushl   %eax # Generating binary (-)
    movl    $6, %eax # Constant integer reference
    neg     %eax # Generating -
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
