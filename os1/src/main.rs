// 告诉 Rust 编译器不使用 Rust 标准库 std 转而使用核心库 core
#![no_std]
//告诉编译器我们没有一般意义上的 main 函数， 并将原来的 main 函数删除。这样编译器也就不需要考虑初始化工作了
#![no_main]

#![feature(panic_info_message)]

use log::*;

//为了能够用到 console.rs 提供的功能，需要添加对 console 的引用
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}
