// use crate::memory::addr::VirtAddr;
pub const PAGE_SIZE: usize = 4 * 1024;

// /// Size of the hypervisor heap.
// pub const HV_HEAP_SIZE: usize = 32 * 1024 * 1024; // 32 MB

// /// Size of the per-CPU data (stack and other CPU-local data).
// pub const PER_CPU_SIZE: usize = 512 * 1024; // 512 KB

// /// Start virtual address of the hypervisor memory.
// pub const HV_BASE: usize = 0xffff_ff00_0000_0000;

extern "C" {
    fn __header_start();
    fn __core_end();
}
