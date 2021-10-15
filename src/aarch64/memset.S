/*
 * Fill in the buffer with character c (alignment handled by the hardware)
 *
 * Parameters:
 *	x0 - buf
 *	x1 - c
 *	x2 - n
 * Returns:
 *	x0 - buf
 */

#include "asm.h"

ENTRY(memset_asm)
	
	cbz x0, memset_exit
	cbz x1, memset_exit
	cbz x2, memset_exit

	mov x8,x0
	and	x7, x1, #255
	orr	w7, w7, w7, lsl #8
	orr w7, w7, w7, lsl #16
	orr x7, x7, x7, lsl #32

set_loop:
	strb w7, [x0], #1
	sub	x2, x2, #1
	cbnz x2, set_loop
	mov x0,x8
memset_exit:
	ret
END(memset_asm)

