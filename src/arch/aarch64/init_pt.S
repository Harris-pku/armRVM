/*
 * macros used by init_bootstrap_pt
 */

/* clobbers x8,x9 */
.macro	set_pte table, xidx, xval, flags
	add	x8, \xval, #(\flags)
	adr	x9, \table
	add	x9, x9, \xidx, lsl #3
	str	x8, [x9]
.endm

/* clobbers x8,x9 */
.macro	set_block table, index, addr, lvl
	and	x8, \addr, \
		#(((1 << ((\lvl + 1) * 9)) - 1) << (12 + (3 - \lvl) * 9))
	set_pte \table, \index, x8, PAGE_DEFAULT_FLAGS
.endm

/* clobbers x8,x9 */
.macro	set_block_dev table, index, addr, lvl
	and	x8, \addr, \
		#(((1 << ((\lvl + 1) * 9)) - 1) << (12 + (3 - \lvl) * 9))
	set_pte \table, \index, x8, (PAGE_DEFAULT_FLAGS|PAGE_FLAG_DEVICE)
.endm

/* clobbers x8,x9 */
.macro	set_table parent, index, child
	adr	x8, \child
	set_pte \parent, \index, x8, PTE_TABLE_FLAGS
.endm

.macro	get_index idx, addr, lvl
	ubfx	\idx, \addr, #(12 + (3 - \lvl) * 9), 9
.endm

/*
 * These are the default vectors. They are used on early startup and if no
 * Spectre v2 mitigation is available.
 */
	.align 11
hyp_vectors:
	ventry	.
	ventry	.
	ventry	.
	ventry	.

	handle_vmexit arch_el2_abt
	ventry	.
	ventry	.
	ventry	.

	handle_vmexit arch_handle_trap
	handle_vmexit irqchip_handle_irq
	ventry	.
	ventry	.

	handle_vmexit arch_handle_trap
	handle_vmexit irqchip_handle_irq
	ventry	.
	ventry	.


	.align 11
	.globl hyp_vectors_hardened
hyp_vectors_hardened:
	ventry	.
	ventry	.
	ventry	.
	ventry	.

	handle_vmexit arch_el2_abt /* no mitigation, we're doomed anyway... */
	ventry	.
	ventry	.
	ventry	.

	handle_abort_fastpath
	handle_vmexit_hardened irqchip_handle_irq
	ventry	.
	ventry	.

	handle_abort_fastpath
	handle_vmexit irqchip_handle_irq
	ventry	.
	ventry	.
