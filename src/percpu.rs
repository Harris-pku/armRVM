// use crate::consts::{PER_CPU_ARRAY_PTR, PER_CPU_SIZE};
// use crate::error::HvResult;
use crate::memory::VirtAddr;

#[derive(Debug, Eq, PartialEq)]
pub enum CpuState {
    HvDisabled,
    HvEnabled,
}

#[repr(C, align(4096))]
pub struct PerCpu {
    /// Referenced by arch::cpu::thread_pointer() for x86_64.
    self_vaddr: VirtAddr,

    pub id: u32,
    // pub state: CpuState,
    // pub vcpu: Vcpu,
    // arch: ArchPerCpu,
    // linux: LinuxContext,
    // Stack will be placed here.
}