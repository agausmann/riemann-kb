use core::cell::Cell;
use cortex_m::{
    interrupt::{CriticalSection, Mutex},
    peripheral::NVIC,
};

use rp2040_hal::{
    clocks::UsbClock,
    pac::{interrupt, Interrupt, RESETS, USBCTRL_DPRAM, USBCTRL_REGS},
    usb::UsbBus,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
};
use usbd_hid::{descriptor::SerializedDescriptor, hid_class::HIDClass};

use crate::nkro::NkroKeyboardReport;

pub struct UsbContext {
    device: UsbDevice<'static, UsbBus>,
    hid: HIDClass<'static, UsbBus>,
}

impl UsbContext {
    pub fn hid_mut(&mut self) -> &mut HIDClass<'static, UsbBus> {
        &mut self.hid
    }

    fn poll(&mut self, flags: &UsbFlags) {
        self.device.poll(&mut [&mut self.hid]);
        let mut leds = [0u8; 1];
        if self.hid.pull_raw_output(&mut leds).is_ok() {
            flags.output.set(Some(Leds {
                raw: leds[0],
                num: leds[0] & (1 << 0) != 0,
                caps: leds[0] & (1 << 1) != 0,
                scroll: leds[0] & (1 << 2) != 0,
            }));
        }
    }
}

pub struct UsbFlags {
    pub output: Cell<Option<Leds>>,
}

#[allow(dead_code)]
pub struct Leds {
    pub raw: u8,
    pub caps: bool,
    pub num: bool,
    pub scroll: bool,
}

// Resources sent to the USB interrupt contexts.
static mut USB_CTX: Option<UsbContext> = None;

// TODO more granular mutex based on which interrupts access this
static USB_FLAGS: Mutex<UsbFlags> = Mutex::new(UsbFlags {
    output: Cell::new(None),
});

pub unsafe fn init(
    regs: USBCTRL_REGS,
    dpram: USBCTRL_DPRAM,
    clocks: UsbClock,
    resets: &mut RESETS,
) {
    let usb_bus = {
        static mut USB_BUS: Option<UsbBusAllocator<UsbBus>> = None;
        unsafe {
            USB_BUS.insert(UsbBusAllocator::new(UsbBus::new(
                regs, dpram, clocks, true, resets,
            )))
        }
    };
    let usb_hid = HIDClass::new(usb_bus, NkroKeyboardReport::desc(), 1);
    // TODO allocate a PID code https://pid.codes
    let usb_device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
        .manufacturer("Gaussian")
        .product("Riemann")
        .device_release(0x0001)
        .build();
    unsafe {
        USB_CTX = Some(UsbContext {
            device: usb_device,
            hid: usb_hid,
        });
        NVIC::unmask(Interrupt::USBCTRL_IRQ);
    }
}

pub unsafe fn state(cs: &CriticalSection) -> UsbDeviceState {
    let _ = cs;
    unsafe { USB_CTX.as_mut() }.unwrap().device.state()
}

pub unsafe fn borrow(cs: &CriticalSection) -> (&mut UsbContext, &UsbFlags) {
    (unsafe { USB_CTX.as_mut() }.unwrap(), USB_FLAGS.borrow(cs))
}

#[interrupt]
fn USBCTRL_IRQ() {
    cortex_m::interrupt::free(|cs| {
        let (ctx, flags) = unsafe { borrow(cs) };
        ctx.poll(flags);
    })
}
