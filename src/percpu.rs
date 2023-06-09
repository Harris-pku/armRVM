#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use core::fmt::{Debug, Formatter, Result};
use core::sync::atomic::{AtomicU32, Ordering};

use aarch64_cpu::registers::*;

use crate::arch::vcpu::Vcpu;
use crate::arch::entry::{shutdown_el2, virt2phys_el2, vmreturn};
use crate::arch::context::LinuxContext;
use crate::cell::Cell;
use crate::consts::{PER_CPU_ARRAY_PTR, PER_CPU_SIZE};
use crate::error::HvResult;
use crate::header::HvHeader;
use crate::header::{HvHeaderStuff, HEADER_STUFF};
use crate::memory::addr::VirtAddr;

static ENTERED_CPUS: AtomicU32 = AtomicU32::new(0);
static ACTIVATED_CPUS: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Eq, PartialEq)]
pub enum CpuState {
    HvDisabled,
    HvEnabled,
}

#[repr(C)]
pub struct PerCpu {
    pub id: u64,
    /// Referenced by arch::cpu::thread_pointer() for x86_64.
    pub self_vaddr: VirtAddr,
    pub state: CpuState,
    pub vcpu: Vcpu,
    linux: LinuxContext,
    // Stack will be placed here.
}

#[allow(unused_must_use)]
impl PerCpu {
    pub fn new<'a>(cpu_id: u64) -> HvResult<&'a mut Self> {
        if Self::entered_cpus() >= HvHeader::get().max_cpus {
            return hv_result_err!(EINVAL);
        }

        let _cpu_rank = ENTERED_CPUS.fetch_add(1, Ordering::SeqCst);
        let vaddr = PER_CPU_ARRAY_PTR as VirtAddr + cpu_id as usize * PER_CPU_SIZE;
        let ret = unsafe { &mut *(vaddr as *mut Self) };
        ret.id = cpu_id;
        ret.self_vaddr = vaddr;
        Ok(ret)
    }

    pub fn stack_top(&self) -> VirtAddr {
        self as *const _ as VirtAddr + PER_CPU_SIZE - 8
    }

    pub fn guest_reg(&self) -> VirtAddr {
        self as *const _ as VirtAddr + PER_CPU_SIZE - 8 - 32 * 8
    }

    pub fn entered_cpus() -> u32 {
        ENTERED_CPUS.load(Ordering::Acquire)
    }

    pub fn activated_cpus() -> u32 {
        ACTIVATED_CPUS.load(Ordering::Acquire)
    }

    pub fn init(&mut self, linux_sp: usize) -> HvResult {
        info!("CPU {} init...", self.id);

        // Save CPU state used for linux
        self.state = CpuState::HvDisabled;
        self.linux = LinuxContext::load_from(linux_sp);

        // // Activate hypervisor page table on each cpu.
        // unsafe { crate::memory::hv_page_table().read().activate() };

        // Initialize vCPU. Use `ptr::write()` to avoid dropping
        unsafe { core::ptr::write(&mut self.vcpu, Vcpu::new(&self.linux, self.id)?) };

        self.state = CpuState::HvEnabled;

        Ok(())
    }

    pub fn activate_vmm(&mut self) -> HvResult {
        // println!("Activating hypervisor on CPU {}...", self.id);
        ACTIVATED_CPUS.fetch_add(1, Ordering::SeqCst);
        self.return_linux()?;
        unreachable!()
    }

    pub fn deactivate_vmm(&mut self, ret_code: usize) -> HvResult {
        // println!("Deactivating hypervisor on CPU {}...", self.id);
        ACTIVATED_CPUS.fetch_sub(1, Ordering::SeqCst);
        info!("Disabling cpu{}", self.id);
        self.arch_shutdown_self();
        Ok(())
    }

    pub fn return_linux(&mut self) -> HvResult {
        unsafe {
            vmreturn(self.guest_reg());
        }
        Ok(())
    }
    
    /*should be in vcpu*/
    pub fn arch_shutdown_self(&mut self) -> HvResult {
        /* Free the guest */

        /* Remove stage-2 mappings */

        /* TLB flush needs the cell's VMID */

        /* we will restore the root cell state with the MMU turned off,
         * so we need to make sure it has been committed to memory */

        /* hand over control of EL2 back to Linux */
        let linux_hyp_vec: u64 =
            unsafe { core::ptr::read_volatile(&HEADER_STUFF.arm_linux_hyp_vectors as *const _) };
        unsafe {
            core::arch::asm!(
                "
                msr vbar_el2,{linux_hyp_vec}
        ",
                linux_hyp_vec= in(reg) linux_hyp_vec,
            );
        }

        /* Return to EL1 */
        /* Disable mmu */

        unsafe {
            let page_offset: u64 = 0xffff_4060_0000;
            virt2phys_el2(self.guest_reg(), page_offset);
        }
        Ok(())
    }
}

pub fn this_cpu_data<'a>() -> &'a mut PerCpu {
    /*per cpu data should be handled after final el2 paging init
    now just only cpu 0*/
    let cpu_id = 0;
    let cpu_data: usize = PER_CPU_ARRAY_PTR as VirtAddr + cpu_id as usize * PER_CPU_SIZE;
    unsafe { &mut *(cpu_data as *mut PerCpu) }
}
