;;; This file contains interrupt handlers.

extern rust_interrupt_handler
global general_interrupt_handler

section .text
bits 64

general_interrupt_handler:

        ;; disable interrupts
        cli

        push rax
        push rcx
        push rdx
        push r8
        push r9
        push r10
        push r11
        push rdi
        push rsi

        ;; Save floating-point registers
        ;; Note: this is SLOW
        fxsave [saved_floats]

        call rust_interrupt_handler

        ;; Restore floating-point registers
        fxrstor [saved_floats]

        pop rsi
        pop rdi
        pop r11
        pop r10
        pop r9
        pop r8
        pop rdx
        pop rcx
        pop rax

        ;; FIXME: re-enable interrupts

        iretq

segment .data
        align 16
saved_floats:
        ;; Reserve space for FXSAVE
        resb 512
