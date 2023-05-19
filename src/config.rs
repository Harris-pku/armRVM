use core::fmt::{Debug/*, Formatter, Result*/};
// use core::{mem::size_of, slice};

// use crate::error::HvResult;
use crate::memory::MemFlags;

#[warn(unused_variables)]
const CONFIG_SIGNATURE: [u8; 6] = *b"RVMSYS";
const CONFIG_REVISION: u16 = 10;

const HV_CELL_NAME_MAXLEN: usize = 31;
const HV_MAX_IOMMU_UNITS: usize = 8;

#[derive(Debug)]
#[repr(C, packed)]
struct HvConsole {
    address: u64,
    size: u32,
    console_type: u16,
    flags: u16,
    divider: u32,
    gate_nr: u32,
    clock_reg: u64,
}

// #[derive(Debug)]
#[repr(C, packed)]
pub struct HvCellDesc {
    signature: [u8; 6],
    revision: u16,

    name: [u8; HV_CELL_NAME_MAXLEN + 1],
    id: u32, // set by the driver
    flags: u32,

    pub cpu_set_size: u32,
    pub num_memory_regions: u32,
    pub num_cache_regions: u32,
    pub num_irqchips: u32,
    pub pio_bitmap_size: u32,
    pub num_pci_devices: u32,
    pub num_pci_caps: u32,

    vpci_irq_base: u32,

    cpu_reset_address: u64,
    msg_reply_timeout: u64,

    console: HvConsole,
}

// #[derive(Debug)]
#[repr(C, packed)]
pub struct HvMemoryRegion {
    pub phys_start: u64,
    pub virt_start: u64,
    pub size: u64,
    pub flags: MemFlags,
}

/// General descriptor of the system.
// #[derive(Debug, Copy)]
#[repr(C, packed)]
pub struct HvSystemConfig {
    pub signature: [u8; 6],
    pub revision: u16,
    flags: u32,

    // /// Jailhouse's location in memory
    pub hypervisor_memory: HvMemoryRegion,
    debug_console: HvConsole,
    // platform_info: PlatformInfo,
    pub root_cell: HvCellDesc,
    // CellConfigLayout placed here.
}

#[macro_use]
#[warn(dead_code)]
impl HvSystemConfig {
    pub fn get<'a>() -> &'a Self {
        unsafe { &*crate::consts::hv_config_ptr() }
    }

    // pub const fn size(&self) -> usize {
    //     size_of::<Self>() + self.root_cell.config_size()
    // }

    // pub fn check(&self) -> HvResult {
    //     if self.signature != CONFIG_SIGNATURE {
    //         return hv_result_err!(EINVAL, "HvSystemConfig signature not matched!");
    //     }
    //     if self.revision != CONFIG_REVISION {
    //         return hv_result_err!(EINVAL, "HvSystemConfig revision not matched!");
    //     }
    //     Ok(())
    // }
}