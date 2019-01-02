    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    pushl   %eax
    movl    $6, %eax # Constant integer reference
    pushl   %eax
    movl    $7, %eax # Constant integer reference
    push    %eax # Generating binary (+)
    movl    $9, %eax # Constant integer reference
    movl    %eax, -8(%ebp) # Assigning new value
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %eax, -4(%ebp) # Assigning new value
    pushl   %eax
    movl    -12(%ebp), %eax # Variable reference
    pushl   %eax # Generating binary (-)
    movl    $5, %eax # Constant integer reference
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    movl    %eax, -12(%ebp) # Assigning new value
    movl    -12(%ebp), %eax # Variable reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
