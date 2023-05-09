//! Hypervisor Memory Layout
//!
//!     +--------------------------------------+ - HV_BASE: 0xffff_ff00_0000_0000 (lower address)
//!     | HvHeader                             |
//!     +--------------------------------------+
//!     | Text Segment                         |
//!     |                                      |
//!     +--------------------------------------+
//!     | Read-only Data Segment               |
//!     |                                      |
//!     +--------------------------------------+
//!     | Data Segment                         |
//!     |                                      |
//!     +--------------------------------------+
//!     | BSS Segment                          |
//!     | (includes hypervisor heap)           |
//!     |                                      |
//!     +--------------------------------------+ - PER_CPU_ARRAY_PTR (core_end)
//!     |  +--------------------------------+  |
//!     |  | Per-CPU Data 0                 |  |
//!     |  +--------------------------------+  |
//!     |  | Per-CPU Stack 0                |  |
//!     |  +--------------------------------+  | - PER_CPU_ARRAY_PTR + PER_CPU_SIZE
//!     |  | Per-CPU Data 1                 |  |
//!     |  +--------------------------------+  |
//!     |  | Per-CPU Stack 1                |  |
//!     |  +--------------------------------+  |
//!     :  :                                :  :
//!     :  :                                :  :
//!     |  +--------------------------------+  |
//!     |  | Per-CPU Data n-1               |  |
//!     |  +--------------------------------+  |
//!     |  | Per-CPU Stack n-1              |  |
//!     |  +--------------------------------+  | - hv_config_ptr
//!     |  | HvSystemConfig                 |  |
//!     |  | +----------------------------+ |  |
//!     |  | | CellConfigLayout           | |  |
//!     |  | |                            | |  |
//!     |  | +----------------------------+ |  |
//!     |  +--------------------------------+  |
//!     +--------------------------------------| - free_memory_start
//!     |  Dynamic Page Pool                   |
//!     :                                      :
//!     :                                      :
//!     |                                      |
//!     +--------------------------------------+ - hv_end (higher address)
//!

pub mod addr;

use bitflags::bitflags;

pub use addr::{GuestPhysAddr, GuestVirtAddr, HostPhysAddr, HostVirtAddr, PhysAddr, VirtAddr};

bitflags! {
    pub struct MemFlags: u64 {
        const READ          = 1 << 0;
        const WRITE         = 1 << 1;
        const EXECUTE       = 1 << 2;
        const DMA           = 1 << 3;
        const IO            = 1 << 4;
        const NO_HUGEPAGES  = 1 << 8;
        const USER          = 1 << 9;
    }
}
