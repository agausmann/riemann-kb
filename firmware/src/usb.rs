use core::cell::Cell;
use cortex_m::{
    interrupt::{CriticalSection, Mutex},
    peripheral::NVIC,
};

use frunk::HList;
use rp2040_hal::{
    clocks::UsbClock,
    pac::{interrupt, Interrupt, RESETS, USBCTRL_DPRAM, USBCTRL_REGS},
    usb::UsbBus,
};
use usb_device::{
    class_prelude::UsbBusAllocator,
    prelude::{UsbDevice, UsbDeviceBuilder, UsbDeviceState, UsbVidPid},
    UsbError,
};
use usbd_human_interface_device::{
    device::{
        consumer::{ConsumerControlFixed, ConsumerControlFixedConfig},
        keyboard::{KeyboardLedsReport, NKROBootKeyboard, NKROBootKeyboardConfig},
    },
    usb_class::{UsbHidClass, UsbHidClassBuilder},
};

pub type HidClass = UsbHidClass<
    'static,
    UsbBus,
    HList!(
        ConsumerControlFixed<'static, UsbBus>,
        NKROBootKeyboard<'static, UsbBus>,
    ),
>;

pub struct UsbContext {
    device: UsbDevice<'static, UsbBus>,
    hid: HidClass,
}

impl UsbContext {
    pub fn keyboard(&mut self) -> &mut NKROBootKeyboard<'static, UsbBus> {
        self.hid.device::<_, _>()
    }

    pub fn consumer(&mut self) -> &mut ConsumerControlFixed<'static, UsbBus> {
        self.hid.device::<_, _>()
    }

    fn poll(&mut self, flags: &UsbFlags) {
        if self.device.poll(&mut [&mut self.hid]) {
            match self.keyboard().read_report() {
                Err(UsbError::WouldBlock) => {}
                Err(e) => {
                    panic!("Failed to read LED report: {:?}", e);
                }
                Ok(leds) => {
                    flags.leds.set(Some(leds));
                }
            }
        }
    }
}

pub struct UsbFlags {
    pub leds: Cell<Option<KeyboardLedsReport>>,
}

// Resources sent to the USB interrupt contexts.
static mut USB_CTX: Option<UsbContext> = None;

// TODO more granular mutex based on which interrupts access this
static USB_FLAGS: Mutex<UsbFlags> = Mutex::new(UsbFlags {
    leds: Cell::new(None),
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

    let usb_hid = UsbHidClassBuilder::new()
        .add_device(NKROBootKeyboardConfig::default())
        .add_device(ConsumerControlFixedConfig::default())
        .build(usb_bus);
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
