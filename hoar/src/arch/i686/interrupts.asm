section .text

%macro isr_noerror 1
global isr%1
isr%1:
    cli
    push byte 0
    push byte %1
    jmp isr_common_stub
%endmacro

%macro isr_error 1
global isr%1
isr%1:
    cli
    push byte %1
    jmp isr_common_stub
%endmacro

%macro irq 1
global irq%1
irq%1:
    cli
    push byte 0
    push byte %1 + 32
    jmp irq_common_stub
%endmacro

isr_noerror 0   ; #DE - zero division
isr_noerror 1   ; #DB - debug
isr_noerror 2   ; NMI - Non Maskable Interrupt
isr_noerror 3   ; #BP - break point
isr_noerror 4   ; #OF - overflow
isr_noerror 5   ; #BR - beyond boundary 
isr_noerror 6   ; #UD - invalid instruction
isr_noerror 7   ; #NM - no math coprocessor
isr_error 8     ; #DF - double exception (error code always 0)
isr_noerror 9   ; Coprocessor segment overrun
isr_error 10    ; #TS - invalid task-switch
isr_error 11    ; #NP - missing segment
isr_error 12    ; #SS - stack error
isr_error 13    ; #GP - general protection
isr_error 14    ; #PF - page fault
isr_noerror 15  ; reserved
isr_noerror 16  ; #MF - error x87 FPU
isr_error 17    ; #AC - align check
isr_noerror 18  ; #MC - machine check
isr_noerror 19  ; #XM - SIMD exception
isr_noerror 20  ; #VE - virtualization
isr_noerror 21  ; reserved
isr_noerror 22  ; reserved
isr_noerror 23  ; reserved
isr_noerror 24  ; reserved
isr_noerror 25  ; reserved
isr_noerror 26  ; reserved
isr_noerror 27  ; reserved
isr_noerror 28  ; reserved
isr_noerror 29  ; reserved
isr_noerror 30  ; reserved
isr_noerror 31  ; reserved

irq 0   ; timer (PIT)
irq 1   ; keyboard
irq 2   ; cascade
irq 3   ; COM2
irq 4   ; COM1
irq 5   ; LPT2
irq 6   ; floppy-disk controller
irq 7   ; LPT1
irq 8   ; Real Time Clock
irq 9   ; ACPI
irq 10  ; Free | USB, SCSI, etc
irq 11  ; Free | videocard, SCSI
irq 12  ; PS/2 mouse
irq 13  ; FPU
irq 14  ; Primary ATA
irq 15  ; Secondary ATA

extern isr_dispatch
isr_common_stub:
    pusha
    push ds
    push es
    push fs
    push gs
    
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    push esp
    call isr_dispatch
    add esp, 4
    
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8
    iret

extern irq_dispatch
irq_common_stub:
    pusha
    push ds
    push es
    push fs
    push gs
    
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    push esp
    call irq_dispatch
    add esp, 4
    
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8
    iret
