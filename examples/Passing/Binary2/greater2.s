    .globl    main
    .type main, @function
main:
    pushl   %ebp # Opening function
    movl    %esp, %ebp
    movl    $5, %eax # Constant integer reference
    pushl    %eax # Generating rel: >
    movl    $2, %eax # Constant integer reference
    popl     %ecx
    cmpl     %eax, %ecx
    movl     %ecx, %eax
    setg     %al # End >
    movl    %ebp, %esp # Close function
    popl    %ebp
    ret
