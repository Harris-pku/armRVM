#![allow(unused_imports)]
#![allow(unused_variables)]
use core::{
    arch::asm,
    marker::PhantomData,
    mem::size_of,
};

use aarch64_cpu::asm::barrier;
use aarch64_cpu::registers::ESR_EL2::EC::Value;
use aarch64_cpu::registers::*;
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use crate::cell::Cell;
use crate::error::HvResult;
use super::context::{GeneralRegisters, LinuxContext};
use crate::memory::addr::{GuestPhysAddr, HostPhysAddr};

#[repr(C)]
pub struct Vcpu {
    /// RSP will be loaded from here when handle VM exits.
    guest_regs: GeneralRegisters,
    guest_sp: u64,
    pub elr: u64,
    spsr: u64,
    host_stack_top: u64,
    pub cpu_id: u64,
}
#[allow(unused_mut)]
impl Vcpu {
    pub fn new(
        linux: &LinuxContext,
        cpu_id: u64,
        // cell: &Cell,
    ) -> HvResult<Self> {
        let mut ret = Self {
            guest_regs: GeneralRegisters::default(),
            guest_sp: 0,
            elr: 0,
            spsr: (SPSR_EL2::M::EL1h
                + SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked)
                .into(),
            cpu_id,
            host_stack_top: 0,
        };
        // HCR_EL2.write(
        //     HCR_EL2::VM::Enable + 
        //     HCR_EL2::IMO::EnableVirtualIRQ +
        //     HCR_EL2::FMO::EnableVirtualFIQ +
        //     HCR_EL2::RW::EL1IsAarch64
        // );
        Ok(ret)
    }

    pub fn cpu_id(&self) -> u64 {
        self.cpu_id
    }

    pub fn regs(&self) -> &GeneralRegisters {
        &self.guest_regs
    }

    pub fn regs_mut(&mut self) -> &mut GeneralRegisters {
        &mut self.guest_regs
    }
    
    // pub fn enter(&mut self) -> HvResult {}
}
