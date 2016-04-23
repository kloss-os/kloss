global _load_idt

section .text
bits 64

;;; Wrapper function to load the IDT
;;; Will be called from Rust
;;; Note that it is following the x86_64 convention of
;;; Expecting the first argument in register rdi.
_load_idt:
        lidt    [rdi]
        ret
