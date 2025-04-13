//! Virtual Device Module
use core::pin::Pin;
use kernel::{
    c_str,
    device::Device,
    fs::File,
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    new_mutex,
    prelude::*,
    sync::Mutex,
    types::ARef,
};

struct Inner {
    value: i32,
}

#[pin_data(PinnedDrop)]
struct VDev {
    #[pin]
    inner: Mutex<Inner>,
    dev: ARef<Device>,
}

#[vtable]
impl MiscDevice for VDev {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        let dev = ARef::from(misc.device());
        dev_info!(dev, "Opening Rust VDev --- called when User Open\n");
        KBox::try_pin_init(
            try_pin_init! {
                VDev {
                    inner <- new_mutex!( Inner {value: 0i32}),
                dev:dev
                }
            },
            GFP_KERNEL,
        )
    }
}

#[pinned_drop]
impl PinnedDrop for VDev {
    fn drop(self: Pin<&mut Self>) {
        dev_info!(self.dev, "Exiting Rust VDev --- called when User Close\n");
    }
}

module! {
    type: VDevModule,
    name: "vdev",
    authors: ["vdev"],
    description: "Rust VDev sample",
    license: "GPL",
}

//if `pin_data!` used only, we cannot impl `Drop` trait for it
//#[pin_data]
#[pin_data(PinnedDrop)]
struct VDevModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<VDev>,
}

// called when insmod
impl kernel::InPlaceModule for VDevModule {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        // Print a banner to make sure our module is working
        pr_info!("------------------------\n");
        pr_info!("Rust VDevModule (enter)\n");
        pr_info!("------------------------\n");

        // this will create /dev/vdev
        let options = MiscDeviceOptions {
            name: c_str!("vdev-miscdev"),
        };
        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options)
        })
    }
}

// called when rmmod
#[pinned_drop]
impl PinnedDrop for VDevModule {
    fn drop(self: Pin<&mut Self>) {
        pr_info!("------------------------\n");
        pr_info!("Rust VDevModule (exit)\n");
        pr_info!("------------------------\n");
    }
}
