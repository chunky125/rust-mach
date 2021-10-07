#include "sysregs.h"
#include "mm.h"
#include "vm_param.h"
#include "asm.h"

/* For Multiboot2 defines */
#include "multiboot2.h"

/* Bit of a fudge for now */
#define INIT_STACK_SIZE 0x2000

.section ".text.boot"


ENTRY(start)
ENTRY(_start)
	bl	preserve_cmdline_args
	b	multiboot_entry

.align 8

multiboot2_header:
	/* Multiboot2 magic for GRUB */
    .long	MULTIBOOT2_HEADER_MAGIC
	/* ISA: aarch64 */
    .long  MULTIBOOT2_ARCHITECTURE_AARCH64 
	/* Header length */
    .long  multiboot2_header_end - multiboot2_header
	/* Checksum */
	.long	-(MULTIBOOT2_HEADER_MAGIC + MULTIBOOT2_ARCHITECTURE_AARCH64 + (multiboot2_header_end - multiboot2_header))
framebuffer_tag_start:
	.short	MULTIBOOT_HEADER_TAG_FRAMEBUFFER
	.short	MULTIBOOT_HEADER_TAG_OPTIONAL
	.long	framebuffer_tag_end - framebuffer_tag_start
	.long	1024
	.long	768
	.long 	32
framebuffer_tag_end:
	/* Padding to 8 byte boundary */
	.long 	0
/*efi_bs_tag_start:
	.short 	MULTIBOOT_HEADER_TAG_EFI_BS
	.short 	0
	.long 	8
efi_bs_tag_end:	 */
end_tag_start:
	.short	MULTIBOOT_HEADER_TAG_END
	.short	0
	.long	8
multiboot2_header_end:

preserve_cmdline_args:
	
	mov 	x21, x0
	mov		x22, x1
	mov		x23, x2
	mov		x24, x3	
	dmb		sy
	ret	

/*
 * Main Entry point
 */
ENTRY(multiboot_entry)
	
	mrs	x0, mpidr_el1
	and	x0, x0, #0xFF
	cbz	x0, master

/*
 * Used to place all cores except first 
 * in a wait mode
 */
proc_wait:
	wfi
	b 	proc_wait

/*
 * Called on the first CPU core to init
 * We should not be entered at EL3, should be EL2
 * configure and drop to EL1
 */
master:
	/* Clear BSS */
	adr     x0, _bss_begin
	mov			x1, xzr
	adr     x2, _bss_end
	sub     x2, x2, x0
	bl      memset 

	mrs	x5,sctlr_EL2

	/* If we're already at EL1 - go there */
	mrs	x0, currentel
	lsr	x0, x0, #2
	cmp	x0, #2
	blt	el1_entry

	/* Make sure there is no offset in vttbr_el2 */
	msr vttbr_el2, xzr

	/* Configure SCTLR_EL1 register */
	ldr	x9,=SCTLR_VALUE_MMU_DISABLED
	msr	sctlr_EL1, x9
	
	/* Configure EL1 execution state */
	mrs 	x1, hcr_el2
	orr 	x0, x1, #(1<<31)
	msr	hcr_el2, x0
	
	mrs	x8,spsr_el2
	mov x4,#0b00101
	msr	spsr_el2, x4

	mrs	x10,ttbr0_el2
	mrs x11,ttbr0_el1

	adr	x0, el1_entry
	msr	elr_el2, x0

	/* Disable trapping of FP/NEON */
	msr	cptr_el2, xzr

	/* Flush the dcache as we have 
	 * disabled MMU and cache in EL1 */
	mov x0, xzr
	bl __asm_dcache_all

	eret

el1_entry:
	mrs	x0, currentel
	lsr	x0, x0, #2
	cmp	x0, #1
	blt	entry_error /* We are trying to be an OS, we cannot be in EL0! */


	/* Initialise EL1 */
	ldr	x1, =LOW_MEMORY
	mov	sp, x1

//	ldr	x1, =Vector_table_el1
//	msr 	VBAR_EL1, x1
	
	/* Turn off trapping for NEON/FP */
	mov	x6, #(0x3 << 20)
	msr	CPACR_EL1, x6
	isb

	// Restore args
	mov	x0,x21
	mov	x1,x22
	mov 	x2,x23
	mov 	x3,x24

	bl	boot_entry

/*
 * Something has gone wrong if we're in here
 * We need to be at EL1 or EL2 to be an operating system
 */
entry_error:
	b	entry_error

.section ".text"
.globl delay
delay:
	subs x0,x0,#1
	bne delay
	ret

ENTRY(aarch64_get_el)
	mrs x0, CurrentEL
	lsr x0, x0, #2
	ret


ENTRY(__asm_dcache_level)
  lsl x12, x0, #1
  msr csselr_el1, x12   /* select cache level */
  isb       /* sync change of cssidr_el1 */
  mrs x6, ccsidr_el1    /* read the new cssidr_el1 */
  and x2, x6, #7    /* x2 <- log2(cache line size)-4 */
  add x2, x2, #4    /* x2 <- log2(cache line size) */
  mov x3, #0x3ff
  and x3, x3, x6, lsr #3  /* x3 <- max number of #ways */
  clz w5, w3      /* bit position of #ways */
  mov x4, #0x7fff
  and x4, x4, x6, lsr #13 /* x4 <- max number of #sets */
  /* x12 <- cache level << 1 */
  /* x2 <- line length offset */
  /* x3 <- number of cache ways - 1 */
  /* x4 <- number of cache sets - 1 */
  /* x5 <- bit position of #ways */

loop_set:
  mov x6, x3      /* x6 <- working copy of #ways */
loop_way:
  lsl x7, x6, x5
  orr x9, x12, x7   /* map way and level to cisw value */
  lsl x7, x4, x2
  orr x9, x9, x7    /* map set number to cisw value */
  tbz w1, #0, 1f
  dc  isw, x9
  b 2f
1:  dc  cisw, x9    /* clean & invalidate by set/way */
2:  subs  x6, x6, #1    /* decrement the way */
  b.ge  loop_way
  subs  x4, x4, #1    /* decrement the set */
  b.ge  loop_set

  ret
/*
 x0 = 0 clean and invalidate, 1 invalidate only
 */
.pushsection .text.__asm_dcache_all, "ax"
ENTRY(__asm_dcache_all)
  mov x1, x0
  dsb sy
  mrs x10, clidr_el1    /* read clidr_el1 */
  lsr x11, x10, #24
  and x11, x11, #0x7    /* x11 <- loc */
  cbz x11, finished   /* if loc is 0, exit */
  mov x15, lr
  mov x0, #0      /* start flush at cache level 0 */
  /* x0  <- cache level */
  /* x10 <- clidr_el1 */
  /* x11 <- loc */
  /* x15 <- return address */

loop_level:
  lsl x12, x0, #1
  add x12, x12, x0    /* x0 <- tripled cache level */
  lsr x12, x10, x12
  and x12, x12, #7    /* x12 <- cache type */
  cmp x12, #2
  b.lt  skip      /* skip if no cache or icache */
  bl  __asm_dcache_level  /* x1 = 0 flush, 1 invalidate */
skip:
  add x0, x0, #1    /* increment cache level */
  cmp x11, x0
  b.gt  loop_level

  mov x0, #0
  msr csselr_el1, x0    /* restore csselr_el1 */
  dsb sy
  isb
  mov lr, x15

finished:
  ret



