.section .multiboot
.align 8

# Multiboot2 header
.long 0xe85250d6        # magic
.long 0                 # architecture (i386)
.long header_end - header_start
.long -(0xe85250d6 + 0 + (header_end - header_start))

header_start:

# ---- Framebuffer REQUEST TAG (TYPE 5) ----
.short 5                # type = 5 (FRAMEBUFFER REQUEST)
.short 0                # flags
.long 20                # size
.long 1024              # width
.long 768               # height
.long 32                # depth (bpp)

# ---- END TAG ----
.short 0
.short 0
.long 8

header_end:

.section .text
.global _start
.extern kernel_main

_start:
    mov %eax, %edi      # multiboot info pointer
    call kernel_main

.hang:
    hlt
    jmp .hang
