.global _entry
.section .text
_entry:
	lui t0, 0xfdecb
	lui t1, 0x78888
	lui t2, 0x88888
	lui t0, 0xfdecb
	lui t1, 0x78888
	lui t2, 0x88888

.section .rodata
msg:
	.ascii "Hello, World!\n"