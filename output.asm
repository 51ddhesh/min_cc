global _start
        global main
        section .text

        _start:
            call main
            mov rdi, rax
            mov rax, 60
            syscall

        main:
            mov rax, 4
    push rax
    mov rax, 3
    pop rcx
    add rax, rcx
    push rax
    mov rax, 2
    pop rcx
    imul rax, rcx
    push rax
    mov rax, 1
    pop rcx
    add rax, rcx
    ret
