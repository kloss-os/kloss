;;; This file contains interrupt handlers.

extern rust_interrupt_handler
extern rust_exception_handler
global general_interrupt_handler
global null_interrupt_handler
global general_exception_handler


section .text
bits 64

;;; Push all registers to the stack
%macro push_all 0
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
%endmacro

;;; Pop all registers to the stack
%macro pop_all 0
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
%endmacro

;;; Define an interrupt handler for the given interrupt, named
;;; isr_N where N is the number (first argument) and P is the name
;;; the function to call.
%macro def_interrupt_handler 2
global isr_%1
isr_%1
        ;; Disable interrupts
        cli

        push_all

        ;; Place the interrupt number as the first argument.
        mov edi, %1
        call %2

        pop_all

        ;; Re-enable interrupts
        sti

        iretq
%endmacro

;;; Define a set of exception handlers for interrupts 0-31
%assign i 0
%rep 32
        def_interrupt_handler i, rust_interrupt_handler
%assign i i+1                   ; i++
%endrep

;;; Define a set of interrupt handlers using the rep macro
;;; Numbers 32--255
%rep 256-32
        def_interrupt_handler i, rust_interrupt_handler
%assign i i+1                   ; i++
%endrep

;;; General (do-nothing) handlers:
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

;;; Do nothing and return
null_interrupt_handler:
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
