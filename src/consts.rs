#[warn(unused_variables)]
// use crate::memory::addr::VirtAddr;
use crate::config::HvSystemConfig;
use crate::header::HvHeader;
// use crate::memory::addr::{align_up, VirtAddr};
use crate::percpu::PerCpu;

pub use crate::memory::PAGE_SIZE;

/// Size of the hypervisor heap.
pub const HV_HEAP_SIZE: usize = 32 * 1024 * 1024; // 32 MB

/// Size of the per-CPU data (stack and other CPU-local data).
pub const PER_CPU_SIZE: usize = 512 * 1024; // 512 KB

/// Start virtual address of the hypervisor memory.
pub const HV_BASE: usize = 0xffff_ff00_0000_0000;

/// Pointer of the `HvHeader` structure.
pub const HV_HEADER_PTR: *const HvHeader = __header_start as _;

/// Pointer of the per-CPU data array.
pub const PER_CPU_ARRAY_PTR: *mut PerCpu = __core_end as _;

// pub const DEFAULT_MAIR_EL2: usize = 0x00000000004404ff;

// pub const ID_AA64MMFR0_PARANGE_SHIFT: usize = 0;

// pub const T0SZ: usize = 0;

// pub const TCR_RGN_WB_WA: usize = 0x1;

// pub const TCR_IRGN0_SHIFT: usize = 8;

// pub const TCR_ORGN0_SHIFT: usize = 10;

// pub const TCR_INNER_SHAREABLE: usize = 0x3;

// pub const TCR_SH0_SHIFT: usize = 12;

// pub const TCR_EL2_RES1: usize = ((1 << 31) | (1 << 23));

// pub const TCR_PS_SHIFT: usize = 16;

// pub const SCTLR_I_BIT: usize = (1 << 12);

// pub const SCTLR_C_BIT: usize = (1 << 2);

// pub const SCTLR_M_BIT: usize = (1 << 0);

// pub const SCTLR_EL2_RES1: usize = ((3 << 4) | (1 << 11) | (1 << 16) | (1 << 18)	| (3 << 22) | (3 << 28));

/// Pointer of the `HvSystemConfig` structure.
pub fn hv_config_ptr() -> *const HvSystemConfig {
    (PER_CPU_ARRAY_PTR as usize + HvHeader::get().max_cpus as usize * PER_CPU_SIZE) as _
}

// /// Pointer of the free memory pool.
// pub fn free_memory_start() -> VirtAddr {
//     align_up(hv_config_ptr() as usize + HvSystemConfig::get().size())
// }

// /// End virtual address of the hypervisor memory.
// pub fn hv_end() -> VirtAddr {
//     HV_BASE + HvSystemConfig::get().hypervisor_memory.size as usize
// }

extern "C" {
    fn __header_start();
    fn __core_end();
}
