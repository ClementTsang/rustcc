    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $2, %eax # Constant integer reference
    pushl   %eax # Generating binary (-)
    movl    $1, %eax # Constant integer reference
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    push    %eax # Generating binary (+)
    movl    $3, %eax # Constant integer reference
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax
    movl    $5, %eax # Constant integer reference
    pushl   %eax # Generating binary (*)
    movl    $6, %eax # Constant integer reference
    popl    %ecx
    imul    %ecx, %eax # End *
    pushl   %eax # Generating binary (/)
    movl    $2, %eax # Constant integer reference
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    pushl   %eax
    movl    $0, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    push    %eax # Generating binary (+)
    movl    $6, %eax # Constant integer reference
    pushl   %eax # Generating binary (/)
    movl    $2, %eax # Constant integer reference
    pushl  %eax
    popl   %ecx
    popl   %eax
    movl    $0, %edx
    idivl   %ecx # End /
    popl    %ecx
    addl    %ecx, %eax # End +
    push    %eax # Generating binary (+)
    movl    $1, %eax # Constant integer reference
    cmpl    $0, %eax # Generating !
    movl    $0, %eax
    sete    %al
    popl    %ecx
    addl    %ecx, %eax # End +
    push    %eax # Generating binary (+)
    movl    $1, %eax # Constant integer reference
    not     %eax # Generating ~
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (-)
    movl    $5, %eax # Constant integer reference
    neg     %eax # Generating -
    pushl   %eax
    popl    %ecx
    popl    %eax
    subl    %ecx, %eax # End -
    pushl   %eax
    movl    -4(%ebp), %eax # Variable reference
    push    %eax # Generating binary (+)
    movl    -8(%ebp), %eax # Variable reference
    popl    %ecx
    addl    %ecx, %eax # End +
    pushl   %eax # Generating binary (*)
    movl    -12(%ebp), %eax # Variable reference
    popl    %ecx
    imul    %ecx, %eax # End *
    neg     %eax # Generating -
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
