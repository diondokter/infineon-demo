#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wrap: Wrap,
    _reserved1: [u8; 0x0ffc],
    core: Core,
}
impl RegisterBlock {
    #[doc = "0x00 - MMIO at SDHC wrapper level"]
    #[inline(always)]
    pub const fn wrap(&self) -> &Wrap {
        &self.wrap
    }
    #[doc = "0x1000..0x1538 - MMIO for Synopsys Mobile Storage Host Controller IP"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
}
#[doc = "MMIO at SDHC wrapper level"]
pub use self::wrap::Wrap;
#[doc = r"Cluster"]
#[doc = "MMIO at SDHC wrapper level"]
pub mod wrap;
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub use self::core::Core;
#[doc = r"Cluster"]
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
pub mod core;
