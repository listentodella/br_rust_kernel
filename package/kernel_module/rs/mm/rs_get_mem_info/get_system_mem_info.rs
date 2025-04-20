//! Virtual Device Module
use core::ffi::c_ulong;
use core::pin::Pin;
//use kernel::bindings::{_totalram_pages, si_meminfo as other_si_meminfo};
use core::sync::atomic::AtomicI64;
use core::sync::atomic::Ordering;
use kernel::page::Page;

use kernel::print;
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
    fn si_meminfo(info: *mut kernel::bindings::sysinfo);
    fn pfn_is_map_memory(pfn: u64) -> bool;
}

fn get_totalram_pages() -> i64 {
    unsafe { _totalram_pages.load(Ordering::Relaxed) }
}

fn get_meminfo() -> kernel::bindings::sysinfo {
    let mut info = core::mem::MaybeUninit::<kernel::bindings::sysinfo>::zeroed();
    unsafe { si_meminfo(info.as_mut_ptr()) };
    unsafe { info.assume_init() }
}

extern "C" {
    fn c_get_num_physpages() -> u64;
    //int pfn_valid(unsigned long pfn)
    fn c_pfn_valid(pfn: u64) -> bool;
    //static C_ARCH_PFN_OFFSET: u64;
    fn c_get_pfn_offset() -> u64;
    //struct page *c_pfn_to_page(u64 pfn)
    //fn c_pfn_to_page(pfn: u64) -> Page;
    fn c_pfn_to_page(pfn: u64) -> *const u8;

    fn c_page_count(pfn: u64) -> u64;
    // fn c_page_count(page: *const u8) -> u64;
    fn c_page_locked(pfn: u64) -> bool;
    fn c_page_reserved(pfn: u64) -> bool;
    fn c_page_swapcache(pfn: u64) -> bool;
    fn c_page_referenced(pfn: u64) -> bool;
    fn c_page_slab(pfn: u64) -> bool;
    fn c_page_private(pfn: u64) -> bool;
    fn c_page_uptodate(pfn: u64) -> bool;
    fn c_page_dirty(pfn: u64) -> bool;
    fn c_page_active(pfn: u64) -> bool;
    fn c_page_writeback(pfn: u64) -> bool;
    fn c_page_mappedtodisk(pfn: u64) -> bool;
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
        let page_nums = unsafe { c_get_num_physpages() };
        let mem_size = page_nums * kernel::page::PAGE_SIZE as u64 / 1024 / 1024;
        pr_info!("{} pages = {} MB\n", page_nums, mem_size);
        pr_info!("ARCH_PFN_OFFSET = {}\n", unsafe { c_get_pfn_offset() });

        let mut valid_pages = 0u64;
        let mut free_pages = 0u64;
        let mut locked_pages = 0u64;
        let mut reserved_pages = 0u64;
        let mut swapcache_pages = 0u64;
        let mut referenced_pages = 0u64;
        let mut slab_pages = 0u64;
        let mut private_pages = 0u64;
        let mut uptodate_pages = 0u64;
        let mut dirty_pages = 0u64;
        let mut active_pages = 0u64;
        let mut writeback_pages = 0u64;
        let mut maptodisk_pages = 0u64;

        for p in 0..page_nums {
            // Most ARM arch has ARCH_PFN_OFFSET
            let pfn = unsafe { p + c_get_pfn_offset() };
            // if !unsafe { c_pfn_valid(pfn) } {
            if !unsafe { pfn_is_map_memory(pfn) } {
                continue;
            }
            valid_pages += 1;
            //let page = unsafe { c_pfn_to_page(pfn) }.as_ptr();
            let page = unsafe { c_pfn_to_page(pfn) };
            if page.is_null() {
                continue;
            }
            pr_info!("get page = {:?}\n", page);

            // 0 means a free page
            if unsafe { c_page_count(pfn) } == 0 {
                // if unsafe { c_page_count(page) } == 0 {
                free_pages += 1;
                continue;
            }

            if unsafe { c_page_locked(pfn) } {
                locked_pages += 1;
            }
            if unsafe { c_page_reserved(pfn) } {
                reserved_pages += 1;
            }
            if unsafe { c_page_swapcache(pfn) } {
                swapcache_pages += 1;
            }
            if unsafe { c_page_referenced(pfn) } {
                referenced_pages += 1;
            }
            if unsafe { c_page_slab(pfn) } {
                slab_pages += 1;
            }
            if unsafe { c_page_private(pfn) } {
                private_pages += 1;
            }
            if unsafe { c_page_uptodate(pfn) } {
                uptodate_pages += 1;
            }
            if unsafe { c_page_dirty(pfn) } {
                dirty_pages += 1;
            }
            if unsafe { c_page_active(pfn) } {
                active_pages += 1;
            }
            if unsafe { c_page_writeback(pfn) } {
                writeback_pages += 1;
            }
            if unsafe { c_page_mappedtodisk(pfn) } {
                maptodisk_pages += 1;
            }
        }

        pr_info!(
            "Pages with valid PFN's = {}, {} MB\n",
            valid_pages,
            valid_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!("                  Pages       KB       MB\n");
        pr_info!(
            "free               {}          {}       {}\n",
            free_pages,
            free_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            free_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "locked             {}          {}       {}\n",
            locked_pages,
            locked_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            locked_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "reserved           {}          {}       {}\n",
            reserved_pages,
            reserved_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            reserved_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "swapcache          {}          {}       {}\n",
            swapcache_pages,
            swapcache_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            swapcache_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "referenced         {}          {}       {}\n",
            referenced_pages,
            referenced_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            referenced_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "slab               {}          {}       {}\n",
            slab_pages,
            slab_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            slab_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "private            {}          {}       {}\n",
            private_pages,
            private_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            private_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "uptodate           {}          {}       {}\n",
            uptodate_pages,
            uptodate_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            uptodate_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "dirty              {}          {}       {}\n",
            dirty_pages,
            dirty_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            dirty_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "active             {}          {}       {}\n",
            active_pages,
            active_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            active_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "writeback          {}          {}       {}\n",
            writeback_pages,
            writeback_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            writeback_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );
        pr_info!(
            "maptodisk          {}          {}       {}\n",
            maptodisk_pages,
            maptodisk_pages * kernel::page::PAGE_SIZE as u64 / 1024,
            maptodisk_pages * kernel::page::PAGE_SIZE as u64 / 1024 / 1024
        );

        Ok(RustMemInfo)
    }
}

impl Drop for RustMemInfo {
    fn drop(&mut self) {
        pr_info!("Rust System Mem Info (exit)\n");
    }
}
