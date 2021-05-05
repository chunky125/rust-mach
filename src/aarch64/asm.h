/*
 * Mach Operating System
 * Copyright (c) 1991,1990,1989 Carnegie Mellon University
 * All Rights Reserved.
 *
 * Permission to use, copy, modify and distribute this software and its
 * documentation is hereby granted, provided that both the copyright
 * notice and this permission notice appear in all copies of the
 * software, derivative works or modified versions, and any portions
 * thereof, and that both notices appear in supporting documentation.
 *
 * CARNEGIE MELLON ALLOWS FREE USE OF THIS SOFTWARE IN ITS "AS IS"
 * CONDITION.  CARNEGIE MELLON DISCLAIMS ANY LIABILITY OF ANY KIND FOR
 * ANY DAMAGES WHATSOEVER RESULTING FROM THE USE OF THIS SOFTWARE.
 *
 * Carnegie Mellon requests users of this software to return to
 *
 *  Software Distribution Coordinator  or  Software.Distribution@CS.CMU.EDU
 *  School of Computer Science
 *  Carnegie Mellon University
 *  Pittsburgh PA 15213-3890
 *
 * any improvements or extensions that they make and grant Carnegie Mellon
 * the rights to redistribute these changes
 *
 */

/* Copyright (C) 1997-2020 Free Software Foundation, Inc.

   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

/*
 * Adapted from GNU Mach and GNU Libc code as per copyright notes above 
 * by Chris Plant, 2020. 
 */

#ifndef _MACH_AARCH64_ASM_H_
#define _MACH_AARCH64_ASM_H_

#ifdef __LP64__
# define AARCH64_R(NAME)	R_AARCH64_ ## NAME
# define PTR_REG(n)		x##n
# define PTR_LOG_SIZE		3
# define DELOUSE(n)
#else
# define AARCH64_R(NAME)	R_AARCH64_P32_ ## NAME
# define PTR_REG(n)		w##n
# define PTR_LOG_SIZE		2
# define DELOUSE(n)		mov     w##n, w##n
#endif

#define PTR_SIZE	(1<<PTR_LOG_SIZE)

/*
 * Output macros to simplify reading/writing to memory
 */
/* Should we do something different with bytes? */
#define outb(addr, val)		outw(addr, val)
#define inb(addr)		inw(addr)

#define outw(addr, val)		{ asm volatile("str %w0, [%x1]" \
					:: "r" (val) , "r" (addr));}
#define inw(addr)		({unsigned int _tmp__; \
				asm volatile("ldr %w0, [%x1]" \
					: "=r" (_tmp__) : "r" (addr)); \
				_tmp__;})

#ifdef	__ASSEMBLER__

/* Syntactic details of assembler.  */

#define ASM_SIZE_DIRECTIVE(name) .size name,.-name

#define C_SYMBOL_NAME(x) x

/* Define an entry point visible from C.  */
#define ENTRY(name)						\
  .globl C_SYMBOL_NAME(name);					\
  .type C_SYMBOL_NAME(name), %function;				\
  .align 4;							\
  C_SYMBOL_NAME(name):						\
  CALL_MCOUNT

/* Define an entry point visible from C.  */
#define ENTRY_ALIGN(name, align)				\
  .globl C_SYMBOL_NAME(name);					\
  .type C_SYMBOL_NAME(name), %function;				\
  .p2align align;						\
  C_SYMBOL_NAME(name):						\
  CALL_MCOUNT

/* Define an entry point visible from C with a specified alignment and
   pre-padding with NOPs.  This can be used to ensure that a critical
   loop within a function is cache line aligned.  Note this version
   does not adjust the padding if CALL_MCOUNT is defined. */

#define ENTRY_ALIGN_AND_PAD(name, align, padding)		\
  .globl C_SYMBOL_NAME(name);					\
  .type C_SYMBOL_NAME(name), %function;				\
  .p2align align;						\
  .rep padding;							\
  nop;								\
  .endr;							\
  C_SYMBOL_NAME(name):						\
  CALL_MCOUNT

#undef	END
#define END(name)						\
  ASM_SIZE_DIRECTIVE(name)

/* If compiled for profiling, call `mcount' at the start of each function.  */
#ifdef	GPROF
#else
# define CALL_MCOUNT		/* Do nothing.  */
#endif

/* Local label name for asm code.  */
#ifndef L
# define L(name)         .L##name
#endif

.macro adr_l, dst, sym
	adrp	\dst, \sym
	add \dst, \dst, :lo12:\sym
.endm 

.macro str_l, src, sym, tmp
	adrp \tmp, \sym
	str \src,[\tmp,:lo12:\sym]
.endm

/* Load or store to/from a pc-relative EXPR into/from R, using T.
   Note R and T are register numbers and not register names.  */
#define LDST_PCREL(OP, R, T, EXPR)			\
	adrp	x##T, EXPR;				\
	OP	PTR_REG (R), [x##T, #:lo12:EXPR];	\

/* Load or store to/from a got-relative EXPR into/from R, using T.
   Note R and T are register numbers and not register names.  */
#define LDST_GLOBAL(OP, R, T,  EXPR)			\
	adrp	x##T, :got:EXPR;			\
	ldr	PTR_REG (T), [x##T, #:got_lo12:EXPR];	\
	OP	PTR_REG (R), [x##T];

/* Load an immediate into R.
   Note R is a register number and not a register name.  */
#ifdef __LP64__
# define MOVL(R, NAME)					\
	movz	PTR_REG (R), #:abs_g3:NAME;		\
	movk	PTR_REG (R), #:abs_g2_nc:NAME;		\
	movk	PTR_REG (R), #:abs_g1_nc:NAME;		\
	movk	PTR_REG (R), #:abs_g0_nc:NAME;
#else
# define MOVL(R, NAME)					\
	movz	PTR_REG (R), #:abs_g1:NAME;		\
	movk	PTR_REG (R), #:abs_g0_nc:NAME;
#endif

/* Since C identifiers are not normally prefixed with an underscore
   on this system, the asm identifier `syscall_error' intrudes on the
   C name space.  Make sure we use an innocuous name.  */
#define syscall_error	__syscall_error
#define mcount		_mcount

#endif /* __ASSEMBLER__ */

#endif




