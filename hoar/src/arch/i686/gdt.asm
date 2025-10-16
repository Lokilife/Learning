section .text
global load_gdt

load_gdt:
    mov eax, [esp + 4]      ; gdt_ptr
    lgdt [eax]

    mov ax, [esp + 8]       ; code_segment selector
    and eax, 0x0000FFFF
    push eax                ; push selector
    push .reload_cs         ; push offset
    retf

.reload_cs:
    mov ax, [esp + 8]       ; code_segment
    add ax, 8               ; data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret
