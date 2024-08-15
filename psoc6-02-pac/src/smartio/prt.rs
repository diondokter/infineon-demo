#[repr(C)]
#[doc = "Programmable IO port registers"]
#[doc(alias = "PRT")]
pub struct Prt {
    ctl: Ctl,
    _reserved1: [u8; 0x0c],
    sync_ctl: SyncCtl,
    _reserved2: [u8; 0x0c],
    lut_sel: [LutSel; 8],
    lut_ctl: [LutCtl; 8],
    _reserved4: [u8; 0x60],
    du_sel: DuSel,
    du_ctl: DuCtl,
    _reserved6: [u8; 0x28],
    data: Data,
}
impl Prt {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - Synchronization control register"]
    #[inline(always)]
    pub const fn sync_ctl(&self) -> &SyncCtl {
        &self.sync_ctl
    }
    #[doc = "0x20..0x40 - LUT component input selection"]
    #[inline(always)]
    pub const fn lut_sel(&self, n: usize) -> &LutSel {
        &self.lut_sel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - LUT component input selection"]
    #[inline(always)]
    pub fn lut_sel_iter(&self) -> impl Iterator<Item = &LutSel> {
        self.lut_sel.iter()
    }
    #[doc = "0x40..0x60 - LUT component control register"]
    #[inline(always)]
    pub const fn lut_ctl(&self, n: usize) -> &LutCtl {
        &self.lut_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - LUT component control register"]
    #[inline(always)]
    pub fn lut_ctl_iter(&self) -> impl Iterator<Item = &LutCtl> {
        self.lut_ctl.iter()
    }
    #[doc = "0xc0 - Data unit component input selection"]
    #[inline(always)]
    pub const fn du_sel(&self) -> &DuSel {
        &self.du_sel
    }
    #[doc = "0xc4 - Data unit component control register"]
    #[inline(always)]
    pub const fn du_ctl(&self) -> &DuCtl {
        &self.du_ctl
    }
    #[doc = "0xf0 - Data register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
}
#[doc = "CTL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control register"]
pub mod ctl;
#[doc = "SYNC_CTL (rw) register accessor: Synchronization control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync_ctl`]
module"]
#[doc(alias = "SYNC_CTL")]
pub type SyncCtl = crate::Reg<sync_ctl::SyncCtlSpec>;
#[doc = "Synchronization control register"]
pub mod sync_ctl;
#[doc = "LUT_SEL (rw) register accessor: LUT component input selection\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_sel`]
module"]
#[doc(alias = "LUT_SEL")]
pub type LutSel = crate::Reg<lut_sel::LutSelSpec>;
#[doc = "LUT component input selection"]
pub mod lut_sel;
#[doc = "LUT_CTL (rw) register accessor: LUT component control register\n\nYou can [`read`](crate::Reg::read) this register and get [`lut_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lut_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lut_ctl`]
module"]
#[doc(alias = "LUT_CTL")]
pub type LutCtl = crate::Reg<lut_ctl::LutCtlSpec>;
#[doc = "LUT component control register"]
pub mod lut_ctl;
#[doc = "DU_SEL (rw) register accessor: Data unit component input selection\n\nYou can [`read`](crate::Reg::read) this register and get [`du_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`du_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@du_sel`]
module"]
#[doc(alias = "DU_SEL")]
pub type DuSel = crate::Reg<du_sel::DuSelSpec>;
#[doc = "Data unit component input selection"]
pub mod du_sel;
#[doc = "DU_CTL (rw) register accessor: Data unit component control register\n\nYou can [`read`](crate::Reg::read) this register and get [`du_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`du_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@du_ctl`]
module"]
#[doc(alias = "DU_CTL")]
pub type DuCtl = crate::Reg<du_ctl::DuCtlSpec>;
#[doc = "Data unit component control register"]
pub mod du_ctl;
#[doc = "DATA (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Data register"]
pub mod data;
