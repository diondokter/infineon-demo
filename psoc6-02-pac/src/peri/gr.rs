#[repr(C)]
#[doc = "Peripheral group structure"]
#[doc(alias = "GR")]
pub struct Gr {
    clock_ctl: ClockCtl,
    _reserved1: [u8; 0x0c],
    sl_ctl: SlCtl,
}
impl Gr {
    #[doc = "0x00 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self) -> &ClockCtl {
        &self.clock_ctl
    }
    #[doc = "0x10 - Slave control"]
    #[inline(always)]
    pub const fn sl_ctl(&self) -> &SlCtl {
        &self.sl_ctl
    }
}
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
#[doc(alias = "CLOCK_CTL")]
pub type ClockCtl = crate::Reg<clock_ctl::ClockCtlSpec>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "SL_CTL (rw) register accessor: Slave control\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_ctl`]
module"]
#[doc(alias = "SL_CTL")]
pub type SlCtl = crate::Reg<sl_ctl::SlCtlSpec>;
#[doc = "Slave control"]
pub mod sl_ctl;
