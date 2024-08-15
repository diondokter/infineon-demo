#[repr(C)]
#[doc = "Trigger 1-to-1 group"]
#[doc(alias = "TR_1TO1_GR")]
pub struct Tr1to1Gr {
    tr_ctl: [TrCtl; 256],
}
impl Tr1to1Gr {
    #[doc = "0x00..0x400 - Trigger control register"]
    #[inline(always)]
    pub const fn tr_ctl(&self, n: usize) -> &TrCtl {
        &self.tr_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x400 - Trigger control register"]
    #[inline(always)]
    pub fn tr_ctl_iter(&self) -> impl Iterator<Item = &TrCtl> {
        self.tr_ctl.iter()
    }
}
#[doc = "TR_CTL (rw) register accessor: Trigger control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl`]
module"]
#[doc(alias = "TR_CTL")]
pub type TrCtl = crate::Reg<tr_ctl::TrCtlSpec>;
#[doc = "Trigger control register"]
pub mod tr_ctl;
