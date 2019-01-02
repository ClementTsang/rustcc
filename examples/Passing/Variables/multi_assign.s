    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $2, %eax # Constant integer reference
    pushl   %eax
    movl    $3, %eax # Constant integer reference
    pushl   %eax
    movl    $2, %eax # Constant integer reference
    pushl   %eax # Generating binary (*)
    movl    $2, %eax # Constant integer reference
    movl    %eax, -8(%ebp) # Assigning new value
    popl    %ecx
    imul    %ecx, %eax # End *
    movl    %eax, -4(%ebp) # Assigning new value
    movl    -4(%ebp), %eax # Variable reference
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
