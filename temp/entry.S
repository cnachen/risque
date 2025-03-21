.global _entry
.section .text
_entry:
	auipc t0, 0x12345
	lui t0, 0x12345
	lui t0, 0
	jalr t1, 0(t0)
