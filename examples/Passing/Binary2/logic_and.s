    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    pushl   %eax # Generating &&
    movl    $0, %eax # Constant integer reference
    popl    %ecx
    cmpl    $0, %ecx
    setne   %cl
    cmpl    $0, %eax
    movl    $0, %eax
    setne   %al
    andb    %cl, %al # End &&
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
