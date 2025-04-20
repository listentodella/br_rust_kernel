//! Virtual Device Module
use core::pin::Pin;
//use kernel::bindings::{_totalram_pages, si_meminfo as other_si_meminfo};
use core::alloc::Layout;
use core::sync::atomic::AtomicI64;
use core::sync::atomic::Ordering;
use kernel::alloc::allocator::Kmalloc;
use kernel::alloc::kbox;
use kernel::alloc::Allocator;
use kernel::bindings::KMALLOC_MAX_ORDER;
use kernel::page::{Page, PAGE_SIZE};
use kernel::print;
use kernel::{c_str, device::Device, fs::File, new_mutex, prelude::*, sync::Mutex, types::ARef};

extern "C" {
    static _totalram_pages: AtomicI64;
    fn si_meminfo(info: *mut kernel::bindings::sysinfo);
}

fn get_totalram_pages() -> i64 {
    unsafe { _totalram_pages.load(Ordering::Relaxed) }
}

fn get_meminfo() -> kernel::bindings::sysinfo {
    let mut info = core::mem::MaybeUninit::<kernel::bindings::sysinfo>::zeroed();
    unsafe { si_meminfo(info.as_mut_ptr()) };
    unsafe { info.assume_init() }
}

extern "C" {}

module! {
    type: RustAllocMem,
    name: "alloc_mem",
    authors: ["alloc_mem"],
    description: "Rust Alloc Mem",
    license: "GPL",
}

struct RustAllocMem;

impl kernel::Module for RustAllocMem {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust Alloc Mem Info (init)\n");
        let max_order = KMALLOC_MAX_ORDER;
        let mut size = PAGE_SIZE;
        // let mut order = 0usize;
        for order in 0..=max_order {
            pr_info!(
                "order = {}, pages = {}, size = {}\n",
                order,
                size / PAGE_SIZE,
                size
            );
            let layout = Layout::from_size_align(size, PAGE_SIZE).unwrap();
            let ret = Kmalloc::alloc(layout, GFP_ATOMIC);
            if !ret.is_ok() {
                pr_err!("Failed to kmalloc for size = {}\n", size);
                break;
            }
            ret.iter().for_each(|x| unsafe { x.write(order as u8) });
            // do we need free since we use rust?
            // unsafe {
            //     Kmalloc::free(ret.unwrap(), layout);
            // }
            // // struct Huge([u8; size as usize]);
            // let ret = kbox::KBox::<[u8; size as usize]>::new_uninit(GFP_ATOMIC).is_ok();

            // this api can only alloc one page
            // let page = Page::alloc_page(GFP_ATOMIC).unwrap();
            size *= 2;
        }

        Ok(RustAllocMem)
    }
}

impl Drop for RustAllocMem {
    fn drop(&mut self) {
        pr_info!("Rust Alloc Mem Info (exit)\n");
    }
}
