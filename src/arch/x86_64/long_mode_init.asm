;;;  HOLY COW WE ARE IN 64-BIT MODE!
;;; Most of this file is stolen from phil-opp:
;;; http://os.phil-opp.com/entering-longmode.html

global long_mode_start

extern rust_main
extern idt

section .text
bits 64
long_mode_start:

        ;; Set up interrupt table:
        lidt [idt]

        ;; Call Rust main
        call rust_main

        .os_returned:
        ;; rust main returned, print `OS returned!`
        mov rax, 0x4f724f204f534f4f
        mov [0xb8000], rax
        mov rax, 0x4f724f754f744f65
        mov [0xb8008], rax
        mov rax, 0x4f214f644f654f6e
        mov [0xb8010], rax
        hlt
