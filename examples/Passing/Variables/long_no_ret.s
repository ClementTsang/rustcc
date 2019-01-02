    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $0, %eax # Constant integer reference
    pushl   %eax
    movl    $0, %eax # Constant integer reference
    pushl   %eax
    movl    $5, %eax # Constant integer reference
    movl    %eax, -4(%ebp) # Assigning new value
    movl    $5, %eax # Constant integer reference
    movl    %eax, -8(%ebp) # Assigning new value
    movl    -8(%ebp), %eax # Variable reference
    push    %eax # Generating binary (+)
    movl    -4(%ebp), %eax # Variable reference
    popl    %ecx
    addl    %ecx, %eax # End +
    push    %eax # Generating binary (+)
    movl    $5, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    movl    %eax, -4(%ebp) # Assigning new value
    movl    $0, %eax # Default return value
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
