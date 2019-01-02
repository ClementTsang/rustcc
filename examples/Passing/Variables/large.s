    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $123, %eax # Constant integer reference
    pushl   %eax
    movl    $1435, %eax # Constant integer reference
    pushl   %eax
    movl    -8(%ebp), %eax # Variable reference
    pushl   %eax
    movl    $5, %eax # Constant integer reference
    movl    %eax, -12(%ebp) # Assigning new value
    pushl   %eax
    movl    -16(%ebp), %eax # Variable reference
    push    %eax # Generating binary (+)
    movl    -12(%ebp), %eax # Variable reference
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (-)
    movl    -4(%ebp), %eax # Variable reference
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    push    %eax # Generating binary (+)
    movl    -8(%ebp), %eax # Variable reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
