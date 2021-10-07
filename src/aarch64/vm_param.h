/* 
 * Mach Operating System
 * Copyright (c) 1991,1990,1989,1988 Carnegie Mellon University
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
 * the rights to redistribute these changes.
 */
/*
 *	File:	vm_param.h
 *	Author:	Avadis Tevanian, Jr. / Chris Plant
 *	Date:	1985 / 2020
 *
 *	AARCH64 machine dependent virtual memory parameters.
 *	Most of the declarations are preceded by AARCH64
 *	which is OK because only AARCH64 specific code will be using
 *	them.
 */

#ifndef	_MACH_AARCH64_VM_PARAM_H_
#define _MACH_AARCH64_VM_PARAM_H_

#include "vm_types.h"

#define BYTE_SIZE		8	/* byte size in bits */

#define AARCH64_PGBYTES		4096	/* bytes per 80386 page */

#define AARCH64_PAGE_SHIFT	12
#define PAGE_SHIFT 		AARCH64_PAGE_SHIFT
#define AARCH64_TABLE_SHIFT	9
#define AARCH64_SECTION_SHIFT	(AARCH64_PAGE_SHIFT + AARCH64_TABLE_SHIFT)

#define AARCH64_PAGE_SIZE	(1 << AARCH64_PAGE_SHIFT)
#define AARCH64_PAGE_MASK 	(~(AARCH64_PAGE_SIZE - 1))
#define AARCH64_SECTION_SIZE	(1 << AARCH64_SECTION_SHIFT)

#define AARCH64_LOW_MEMORY	(2 * AARCH64_SECTION_SIZE)
#define AARCH64_HIGH_MEMORY	DEVICE_BASE

#define AARCH64_PAGING_MEMORY	(AARCH64_HIGH_MEMORY - AARCH64_LOW_MEMORY)
#define AARCH64_PAGING_PAGES	(AARCH64_PAGING_MEMORY / AARCH64_PAGE_SIZE)

#define AARCH64_PTRS_PER_TABLE	(1 << AARCH64_TABLE_SHIFT)

#define AARCH64_PGD_SHIFT	AARCH64_PAGE_SHIFT + 3 * AARCH64_TABLE_SHIFT
#define AARCH64_PUD_SHIFT	AARCH64_PAGE_SHIFT + 2 * AARCH64_TABLE_SHIFT
#define AARCH64_PMD_SHIFT 	AARCH64_PAGE_SHIFT + 1 * AARCH64_TABLE_SHIFT

#define AARCH64_PG_DIR_SIZE	(3 * AARCH64_PAGE_SIZE)

/*
 * Limited to 3GB by Mach servers and glibc
 */
#define VM_MIN_ADDRESS		(0)
#define VM_MAX_ADDRESS		(0xc0000000UL)


/*
 * Useful sizes
 */
#define SZ_1M				0x00100000

#endif /* _MACH_AARCH64_VM_PARAM_H_ */
