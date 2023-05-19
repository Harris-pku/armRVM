#![no_std] // 禁用标准库链接
#![no_main] // 不使用main入口，使用自己定义实际入口_start，因为我们还没有初始化堆栈指针 
#![feature(naked_functions)] //  surpport naked function
#![feature(default_alloc_error_handler)]
// use core::arch::global_asm; // 支持内联汇编
#[macro_use]
extern crate alloc;
extern crate buddy_system_allocator;

mod config;
mod header;
mod error;
mod panic;
mod percpu;
mod consts;
mod memory;

#[cfg(target_arch = "aarch64")]
#[path = "arch/aarch64/mod.rs"]
mod arch;

use percpu::PerCpu;
use error::HvResult;

#[warn(dead_code)]
fn primary_init_early() -> HvResult{
    memory::init_heap();
    Ok(())
}

#[warn(unused_variables)]
fn main(cpuid:u32,cpu_data: &mut PerCpu) -> HvResult{
    let is_primary = cpuid == 0;
    if is_primary {
        primary_init_early()?;
    }
    Ok(())
}

#[warn(dead_code)]
fn arch_handle_exit()-> Result<(), ()> {
    Ok(())
}

extern "C" fn entry(cpuid:u32,cpu_data: &mut PerCpu) -> () {
    if let Err(_e) = main(cpuid,cpu_data) {}
}