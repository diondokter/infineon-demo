#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    usbdev: Usbdev,
    _reserved1: [u8; 0x0c1c],
    usblpm: Usblpm,
    _reserved2: [u8; 0x1f8c],
    usbhost: Usbhost,
}
impl RegisterBlock {
    #[doc = "0x00..0x13e4 - USB Device"]
    #[inline(always)]
    pub const fn usbdev(&self) -> &Usbdev {
        &self.usbdev
    }
    #[doc = "0x2000..0x2074 - USB Device LPM and PHY Test"]
    #[inline(always)]
    pub const fn usblpm(&self) -> &Usblpm {
        &self.usblpm
    }
    #[doc = "0x4000..0x4b34 - USB Host Controller"]
    #[inline(always)]
    pub const fn usbhost(&self) -> &Usbhost {
        &self.usbhost
    }
}
#[doc = "USB Device"]
pub use self::usbdev::Usbdev;
#[doc = r"Cluster"]
#[doc = "USB Device"]
pub mod usbdev;
#[doc = "USB Device LPM and PHY Test"]
pub use self::usblpm::Usblpm;
#[doc = r"Cluster"]
#[doc = "USB Device LPM and PHY Test"]
pub mod usblpm;
#[doc = "USB Host Controller"]
pub use self::usbhost::Usbhost;
#[doc = r"Cluster"]
#[doc = "USB Host Controller"]
pub mod usbhost;
