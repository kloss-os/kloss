;;; This file contains interrupt handlers.

extern rust_interrupt_handler
extern rust_exception_handler
global general_interrupt_handler
global general_exception_handler

section .text
bits 64

general_exception_handler:
        cli

        push rax
        push rbx
        push rcx
        push rdx
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15
        push rdi
        push rsi

        call rust_exception_handler

        pop rsi
        pop rdi
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rdx
        pop rcx
        pop rbx
        pop rax

        ;; Re-enable interrupts
        sti

        iretq



general_interrupt_handler:

        ;; disable interrupts
        cli

        push rax
        push rbx
        push rcx
        push rdx
        push r8
        push r9
        push r10
        push r11
        push r12
        push r13
        push r14
        push r15
        push rdi
        push rsi


        ;; Save floating-point registers
        ;; Note: this is SLOW
        ;; fxsave [saved_floats]

        call rust_interrupt_handler

        ;; Restore floating-point registers
        ;; fxrstor [saved_floats]

        pop rsi
        pop rdi
        pop r15
        pop r14
        pop r13
        pop r12
        pop r11
        pop r10
        pop r9
        pop r8
        pop rdx
        pop rcx
        pop rbx
        pop rax

        ;; Re-enable interrupts
        sti

        iretq

;; segment .data
;;         align 16
;; saved_floats:
;;         ;; Reserve space for FXSAVE
;;         resb 512
