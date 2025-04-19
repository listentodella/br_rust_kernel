//! Virtual Device Module
use core::ffi::c_ulong;
use core::pin::Pin;
use kernel::{c_str, device::Device, fs::File, new_mutex, prelude::*, sync::Mutex, types::ARef};

// currently cannot pass compiler, maybe we locally create a c function, and ffi ?
// extern "C" {
//     fn get_num_physpages() -> c_ulong;
// }

// fn get_total_phys_pages() -> usize {
//     unsafe { get_num_physpages() as usize }
// }

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
        pr_info!("Am I built-in? {}\n", !cfg!(MODULE));
        //let page_nums = get_total_phys_pages();
        //pr_info!("get system mem pages = {}\n", page_nums);

        Ok(RustMemInfo)
    }
}

impl Drop for RustMemInfo {
    fn drop(&mut self) {
        pr_info!("Rust System Mem Info (exit)\n");
    }
}
