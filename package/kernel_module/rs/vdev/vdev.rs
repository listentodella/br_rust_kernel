//! Virtual Device Module
use kernel::prelude::*;

module! {
    type: VDev,
    name: "vdev",
    authors: ["Rust for Linux Contributors"],
    description: "Rust VDev sample",
    license: "GPL",
}

struct VDev;

impl kernel::Module for VDev {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        // Print a banner to make sure our module is working
        pr_info!("------------------------\n");
        pr_info!("Rust VDev (enter)\n");
        pr_info!("------------------------\n");
        Ok(VDev)
    }
}

impl Drop for VDev {
    fn drop(&mut self) {
        pr_info!("Rust VDev (exit)\n");
    }
}
