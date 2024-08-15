#[repr(C)]
#[doc = "MMIO at SDHC wrapper level"]
#[doc(alias = "WRAP")]
pub struct Wrap {
    ctl: Ctl,
}
impl Wrap {
    #[doc = "0x00 - Top level wrapper control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
}
#[doc = "CTL (rw) register accessor: Top level wrapper control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Top level wrapper control"]
pub mod ctl;
