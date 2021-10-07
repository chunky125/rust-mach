#ifndef	_MM_H
#define	_MM_H

/*
 * FIXME: This shouldn't be a required dependancy
 */
#ifdef PLATFORM_raspi
#include <raspi/peripherals.h>
#endif

#define TABLE_SHIFT 		9
#define SECTION_SHIFT		(PAGE_SHIFT + TABLE_SHIFT)

#define PAGE_SIZE   		(1 << PAGE_SHIFT)	
#define SECTION_SIZE		(1 << SECTION_SHIFT)	
#define LOW_MEMORY          (2 * SECTION_SIZE) 

#ifndef __ASSEMBLER__

unsigned long get_free_page();
void free_page(unsigned long p);
void memzero(unsigned long src, unsigned long n);

#endif

#endif  /*_MM_H */
