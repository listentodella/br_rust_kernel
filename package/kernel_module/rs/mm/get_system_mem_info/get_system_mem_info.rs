//! Virtual Device Module
use core::ffi::c_ulong;
use core::pin::Pin;
//use kernel::bindings::{_totalram_pages, si_meminfo as other_si_meminfo};
use core::sync::atomic::AtomicI64;
use core::sync::atomic::Ordering;
use kernel::bindings::si_meminfo as other_si_meminfo;

use kernel::{c_str, device::Device, fs::File, new_mutex, prelude::*, sync::Mutex, types::ARef};

// currently cannot pass compiler, maybe we locally create a c function, and ffi ?
// extern "C" {
//     fn get_num_physpages() -> c_ulong;
// }

// fn get_total_phys_pages() -> usize {
//     unsafe { get_num_physpages() as usize }
// }

extern "C" {
    static _totalram_pages: AtomicI64;
}

fn get_totalram_pages() -> i64 {
    unsafe { _totalram_pages.load(Ordering::Relaxed) }
}

extern "C" {
    fn si_meminfo(info: *mut kernel::bindings::sysinfo);
}

fn get_meminfo() -> kernel::bindings::sysinfo {
    let mut info = core::mem::MaybeUninit::<kernel::bindings::sysinfo>::zeroed();
    unsafe { si_meminfo(info.as_mut_ptr()) };
    unsafe { info.assume_init() }
}

module! {
    type: RustMemInfo,
    name: "mem_info",
    authors: ["mem_info"],
    description: "Rust Mem Info",
    license: "GPL",
}

struct RustMemInfo;

impl kernel::Module for RustMemInfo {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust System Mem Info (init)\n");
        //let page_nums = get_total_phys_pages();
        //pr_info!("get system mem pages = {}\n", page_nums);

        let sysinfo = get_meminfo();
        //pr_info!("get sysinfo = {:?}", sysinfo);
        //let totalram_pages = unsafe { _totalram_pages } as AtomicI64;
        //let totalram_pages = totalram_pages.load(Ordering::Relaxed);
        let totalram_pages = get_totalram_pages();
        pr_info!("get totalram_pages = {}\n", totalram_pages);

        Ok(RustMemInfo)
    }
}

impl Drop for RustMemInfo {
    fn drop(&mut self) {
        pr_info!("Rust System Mem Info (exit)\n");
    }
}
