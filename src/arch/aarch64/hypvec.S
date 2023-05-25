/*
 * These are the default vectors. They are used on early startup and if no
 * Spectre v2 mitigation is available.
 */ /*
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

/*
	.align 11
	.globl hyp_vectors_hardened
hyp_vectors_hardened:
	ventry	.
	ventry	.
	ventry	.
	ventry	.

	handle_vmexit arch_el2_abt 
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
*/