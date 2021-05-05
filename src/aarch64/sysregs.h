#ifndef _SYSREGS_H
#define _SYSREGS_H

// ***************************************
// SCTLR_EL1, System Control Register (EL1), Page 2654 of AArch64-Reference-Manual.
// ***************************************

#define SCTLR_RESERVED                  (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11)
#define SCTLR_EE_LITTLE_ENDIAN          (0 << 25)
#define SCTLR_EOE_LITTLE_ENDIAN         (0 << 24)
#define SCTLR_I_CACHE_DISABLED          (0 << 12)
#define SCTLR_D_CACHE_DISABLED          (0 << 2)
#define SCTLR_MMU_DISABLED              (0 << 0)
#define SCTLR_MMU_ENABLED               (1 << 0)

#define SCTLR_VALUE_MMU_DISABLED	(SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED )

// ***************************************
// HCR_EL2, Hypervisor Configuration Register (EL2), Page 2487 of AArch64-Reference-Manual.
// ***************************************

#define HCR_RW	    			(1 << 31)
#define HCR_VALUE			HCR_RW

/* 
 * SCR_EL3, Secure Configuration Register (EL3), Page 2648 of AArch64-Reference-Manual.
 */
#define SCR_RESERVED	    		(3 << 4)
#define SCR_RW				(1 << 10)
#define SCR_NS				(1 << 0)
#define SCR_VALUE	    	    	(SCR_RESERVED | SCR_RW | SCR_NS)

/*
 * SPSR_EL2, Saved Program Status Register (EL3) Page 389 of AArch64-Reference-Manual.
 */
#define SPSR_MASK_ALL 			(7 << 6)
#define SPSR_EL1h			(5 << 0)
#define SPSR_VALUE			(SPSR_MASK_ALL | SPSR_EL1h)

/*
 * TCR - Translation Control Register 
 */
#define	TCR_T0SZ		(64 - 48)
#define TCR_T1SZ		((64 - 48) << 16)
#define TCR_TG0_4K	(0 << 14)
#define TCR_TG1_4K	(2 << 30)



/*
 * Floating Point Control Register (FPCR)
 */
#define AARCH64_FPCR_AHP	(1 << 26) /* Alternative half precision */
#define AARCH64_FPCR_DN		(1 << 25) /* Default NaN enable */	
#define AARCH64_FPCR_FZ		(1 << 24) /* Flush to zero enable */	
#define AARCH64_FPCR_RMODE_RP	(1 << 22) /* Round to plus inifinity */
#define AARCH64_FPCR_RMODE_RM 	(2 << 22) /* Round to Minus Infinity */
#define AARCH64_FPCR_RMODE_RZ	(3 << 22) /* Round to Zero */
#define AARCH64_FPCR_FZ16	(1 << 19) /* Flush to zero for half precision */	

#endif /* _AARCH64_SYSREG_H_ */
