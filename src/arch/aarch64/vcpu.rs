use crate::error::HvResult;
use crate::percpu::GeneralRegisters;

use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::ESR_EL2::EC::Value;
use aarch64_cpu::registers::*;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

#[repr(C)]
pub struct Vcpu {
    /// RSP will be loaded from here when handle VM exits.
    regs: GeneralRegisters,
}
impl Vcpu {
    pub fn new() -> HvResult<Self> {
        let mut ret = Self {
            regs: GeneralRegisters,
        };
        Ok(ret)
    }
    pub fn enter(&mut self) -> HvResult {}
}
