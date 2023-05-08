use core::arch::global_asm; // 支持内联汇编

use crate::consts::{DEFAULT_MAIR_EL2, T0SZ, TCR_IRGN0_SHIFT, TCR_ORGN0_SHIFT};
use crate::consts::{TCR_INNER_SHAREABLE, TCR_SH0_SHIFT, TCR_EL2_RES1};
use crate::consts::{ID_AA64MMFR0_PARANGE_SHIFT, SCTLR_I_BIT, SCTLR_C_BIT};
use crate::consts::{SCTLR_M_BIT, SCTLR_EL2_RES1};

pub unsafe extern "C" fn el2_entry() -> i32 {
    core::arch::asm!(
        "
        mrs	x1, esr_el2
    	lsr	x1, x1, #26
	    cmp	x1, #0x16
	    b.ne	.

        /* init bootstrap page tables */
        bl	{init_bootstrap_pt}
        
        adr	x0, bootstrap_pt_l0 // todo
    	adr	x30, 1f
	    phys2virt x30
    	b	{enable_mmu_el2}

        /* install the final vectors */
        adr	x1, hyp_vectors
    	msr	vbar_el2, x1

	    mov	x0, x16
    	adrp	x1, __page_pool // todo
	    mov	x2, #PERCPU_SIZE_ASM // todo

        madd	x1, x2, x0, x1

        add	sp, x1, #PERCPU_STACK_END // todo
    	stp	x29, x17, [sp, #-16]!
	    stp	x27, x28, [sp, #-16]!
    	stp	x25, x26, [sp, #-16]!
	    stp	x23, x24, [sp, #-16]!
    	stp	x21, x22, [sp, #-16]!
	    stp	x19, x20, [sp, #-16]!

        sub	sp, sp, 20 * 8

    	mrs	x29, id_aa64mmfr0_el1
	    str	x29, [x1, #PERCPU_ID_AA64MMFR0] // todo

    	mov	x29, xzr	/* reset fp,lr */
	    mov	x30, xzr

        
        bl {entry}     
        eret
        ",
        init_bootstrap_pt = sym self::init_bootstrap_pt,
        enable_mmu_el2 = sym self::enable_mmu_el2,
        entry = sym crate::entry,
        options(noreturn),
    );
}

global_asm!(
    include_str!("./bootvec.S"),
    sym el2_entry
);

pub unsafe extern "C" fn init_bootstrap_pt() -> i32 {
    core::arch::asm!(
        "
        adrp	x0, __trampoline_start

    	get_index x2, x13, 0
	    set_table bootstrap_pt_l0, x2, bootstrap_pt_l1_hyp_uart

    	get_index x3, x0, 0
	    set_table bootstrap_pt_l0, x3, bootstrap_pt_l1_trampoline

    	get_index x2, x13, 1
	    set_table bootstrap_pt_l1_hyp_uart, x2, bootstrap_pt_l2_hyp_uart
    	get_index x4, x0, 1
	    set_block bootstrap_pt_l1_trampoline, x4, x0, 1

    	get_index x2, x13, 2
	    set_block bootstrap_pt_l2_hyp_uart, x2, x12, 2
    	get_index x3, x15, 2
	    set_block_dev bootstrap_pt_l2_hyp_uart, x3, x14, 2

    	adrp	x0, bootstrap_pt_l0
	    mov	x1, PAGE_SIZE * 4
    	mov	x2, DCACHE_INVALIDATE_ASM

        /* will return to our caller */
	    b	arm_dcaches_flush // todo
        ",
        options(noreturn),
    );
}

global_asm!(include_str!("./init_pt.S"));

pub unsafe extern "C" fn enable_mmu_el2() -> i32 {
    core::arch::asm!(
        "
        /* setup the MMU for EL2 hypervisor mappings */
    	ldr	x1, ={DEFAULT_MAIR_EL2}
	    msr	mair_el2, x1

        ldr	x1, =({T0SZ}(48) | ({TCR_RGN_WB_WA} << {TCR_IRGN0_SHIFT})	\
    			       | ({TCR_RGN_WB_WA} << {TCR_ORGN0_SHIFT})	\
	    		       | ({TCR_INNER_SHAREABLE} << {TCR_SH0_SHIFT})	\
		    	       | {TCR_EL2_RES1})

        mrs     x9, id_aa64mmfr0_el1
    	/* Narrow PARange to fit the PS field in TCR_ELx */
	    ubfx    x9, x9, #{ID_AA64MMFR0_PARANGE_SHIFT}, #3
	    bfi     x1, x9, #{TCR_PS_SHIFT}, #3

    	msr	tcr_el2, x1

    	msr	ttbr0_el2, x0

    	isb
	    tlbi	alle2
	    dsb	nsh

    	/* Enable MMU, allow cacheability for instructions and data */
	    ldr	x1, =({SCTLR_I_BIT} | {SCTLR_C_BIT} | {SCTLR_M_BIT} | {SCTLR_EL2_RES1})
	    msr	sctlr_el2, x1

    	isb
	    tlbi	alle2
	    dsb	nsh

    	ret    
        ",
        DEFAULT_MAIR_EL2 = sym crate::consts::DEFAULT_MAIR_EL2,
        T0SZ = sym crate::consts::T0SZ,
        TCR_RGN_WB_WA = sym crate::consts::TCR_RGN_WB_WA,
        TCR_IRGN0_SHIFT = sym crate::consts::TCR_IRGN0_SHIFT,
        TCR_ORGN0_SHIFT = sym crate::consts::TCR_ORGN0_SHIFT,
        TCR_INNER_SHAREABLE = sym crate::consts::TCR_INNER_SHAREABLE,
        TCR_SH0_SHIFT = sym crate::consts::TCR_SH0_SHIFT,
        TCR_EL2_RES1 = sym crate::consts::TCR_EL2_RES1,
        ID_AA64MMFR0_PARANGE_SHIFT = sym crate::consts::ID_AA64MMFR0_PARANGE_SHIFT,
        TCR_PS_SHIFT = sym crate::consts::TCR_PS_SHIFT,
        SCTLR_I_BIT = sym crate::consts::SCTLR_I_BIT,
        SCTLR_C_BIT = sym crate::consts::SCTLR_C_BIT,
        SCTLR_M_BIT = sym crate::consts::SCTLR_M_BIT,
        SCTLR_EL2_RES1 = sym crate::consts::SCTLR_M_BIT,
        options(noreturn),
    );
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn arch_entry() -> i32 {
    core::arch::asm!(
        "

        mov	x16, x0
	    mov	x17, x30
        ldr	x13, =BASE_ADDRESS 
        ldr	x12, =0x7fc00000    //config file hv.phy_mem
        sub	x11, x12, x13
        ldr	x1, =bootstrap_vectors
        virt2phys x1       

        /* choose opcode */
        mov	x0, 0
        hvc	#0   //install bootstrap vec

        hvc	#0	/* bootstrap vectors enter EL2 at el2_entry */
        mov	x30, x17 /* we go back to linux */
        //mov x0, -22 //return EINVAL ?driver
        ret

    ",
        options(noreturn),
    );
}
