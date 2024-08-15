#[repr(C)]
#[doc = "USB Device LPM and PHY Test"]
#[doc(alias = "USBLPM")]
pub struct Usblpm {
    power_ctl: PowerCtl,
    _reserved1: [u8; 0x04],
    usbio_ctl: UsbioCtl,
    flow_ctl: FlowCtl,
    lpm_ctl: LpmCtl,
    lpm_stat: LpmStat,
    _reserved5: [u8; 0x08],
    intr_sie: IntrSie,
    intr_sie_set: IntrSieSet,
    intr_sie_mask: IntrSieMask,
    intr_sie_masked: IntrSieMasked,
    intr_lvl_sel: IntrLvlSel,
    intr_cause_hi: IntrCauseHi,
    intr_cause_med: IntrCauseMed,
    intr_cause_lo: IntrCauseLo,
    _reserved13: [u8; 0x30],
    dft_ctl: DftCtl,
}
impl Usblpm {
    #[doc = "0x00 - Power Control Register"]
    #[inline(always)]
    pub const fn power_ctl(&self) -> &PowerCtl {
        &self.power_ctl
    }
    #[doc = "0x08 - USB IO Control Register"]
    #[inline(always)]
    pub const fn usbio_ctl(&self) -> &UsbioCtl {
        &self.usbio_ctl
    }
    #[doc = "0x0c - Flow Control Register"]
    #[inline(always)]
    pub const fn flow_ctl(&self) -> &FlowCtl {
        &self.flow_ctl
    }
    #[doc = "0x10 - LPM Control Register"]
    #[inline(always)]
    pub const fn lpm_ctl(&self) -> &LpmCtl {
        &self.lpm_ctl
    }
    #[doc = "0x14 - LPM Status register"]
    #[inline(always)]
    pub const fn lpm_stat(&self) -> &LpmStat {
        &self.lpm_stat
    }
    #[doc = "0x20 - USB SOF, BUS RESET and EP0 Interrupt Status"]
    #[inline(always)]
    pub const fn intr_sie(&self) -> &IntrSie {
        &self.intr_sie
    }
    #[doc = "0x24 - USB SOF, BUS RESET and EP0 Interrupt Set"]
    #[inline(always)]
    pub const fn intr_sie_set(&self) -> &IntrSieSet {
        &self.intr_sie_set
    }
    #[doc = "0x28 - USB SOF, BUS RESET and EP0 Interrupt Mask"]
    #[inline(always)]
    pub const fn intr_sie_mask(&self) -> &IntrSieMask {
        &self.intr_sie_mask
    }
    #[doc = "0x2c - USB SOF, BUS RESET and EP0 Interrupt Masked"]
    #[inline(always)]
    pub const fn intr_sie_masked(&self) -> &IntrSieMasked {
        &self.intr_sie_masked
    }
    #[doc = "0x30 - Select interrupt level for each interrupt source"]
    #[inline(always)]
    pub const fn intr_lvl_sel(&self) -> &IntrLvlSel {
        &self.intr_lvl_sel
    }
    #[doc = "0x34 - High priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_hi(&self) -> &IntrCauseHi {
        &self.intr_cause_hi
    }
    #[doc = "0x38 - Medium priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_med(&self) -> &IntrCauseMed {
        &self.intr_cause_med
    }
    #[doc = "0x3c - Low priority interrupt Cause register"]
    #[inline(always)]
    pub const fn intr_cause_lo(&self) -> &IntrCauseLo {
        &self.intr_cause_lo
    }
    #[doc = "0x70 - DFT control"]
    #[inline(always)]
    pub const fn dft_ctl(&self) -> &DftCtl {
        &self.dft_ctl
    }
}
#[doc = "POWER_CTL (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`power_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ctl`]
module"]
#[doc(alias = "POWER_CTL")]
pub type PowerCtl = crate::Reg<power_ctl::PowerCtlSpec>;
#[doc = "Power Control Register"]
pub mod power_ctl;
#[doc = "USBIO_CTL (rw) register accessor: USB IO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_ctl`]
module"]
#[doc(alias = "USBIO_CTL")]
pub type UsbioCtl = crate::Reg<usbio_ctl::UsbioCtlSpec>;
#[doc = "USB IO Control Register"]
pub mod usbio_ctl;
#[doc = "FLOW_CTL (rw) register accessor: Flow Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow_ctl`]
module"]
#[doc(alias = "FLOW_CTL")]
pub type FlowCtl = crate::Reg<flow_ctl::FlowCtlSpec>;
#[doc = "Flow Control Register"]
pub mod flow_ctl;
#[doc = "LPM_CTL (rw) register accessor: LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm_ctl`]
module"]
#[doc(alias = "LPM_CTL")]
pub type LpmCtl = crate::Reg<lpm_ctl::LpmCtlSpec>;
#[doc = "LPM Control Register"]
pub mod lpm_ctl;
#[doc = "LPM_STAT (r) register accessor: LPM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpm_stat`]
module"]
#[doc(alias = "LPM_STAT")]
pub type LpmStat = crate::Reg<lpm_stat::LpmStatSpec>;
#[doc = "LPM Status register"]
pub mod lpm_stat;
#[doc = "INTR_SIE (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie`]
module"]
#[doc(alias = "INTR_SIE")]
pub type IntrSie = crate::Reg<intr_sie::IntrSieSpec>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Status"]
pub mod intr_sie;
#[doc = "INTR_SIE_SET (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_set`]
module"]
#[doc(alias = "INTR_SIE_SET")]
pub type IntrSieSet = crate::Reg<intr_sie_set::IntrSieSetSpec>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Set"]
pub mod intr_sie_set;
#[doc = "INTR_SIE_MASK (rw) register accessor: USB SOF, BUS RESET and EP0 Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_sie_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_mask`]
module"]
#[doc(alias = "INTR_SIE_MASK")]
pub type IntrSieMask = crate::Reg<intr_sie_mask::IntrSieMaskSpec>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Mask"]
pub mod intr_sie_mask;
#[doc = "INTR_SIE_MASKED (r) register accessor: USB SOF, BUS RESET and EP0 Interrupt Masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_sie_masked`]
module"]
#[doc(alias = "INTR_SIE_MASKED")]
pub type IntrSieMasked = crate::Reg<intr_sie_masked::IntrSieMaskedSpec>;
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked"]
pub mod intr_sie_masked;
#[doc = "INTR_LVL_SEL (rw) register accessor: Select interrupt level for each interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_lvl_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_lvl_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_lvl_sel`]
module"]
#[doc(alias = "INTR_LVL_SEL")]
pub type IntrLvlSel = crate::Reg<intr_lvl_sel::IntrLvlSelSpec>;
#[doc = "Select interrupt level for each interrupt source"]
pub mod intr_lvl_sel;
#[doc = "INTR_CAUSE_HI (r) register accessor: High priority interrupt Cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_hi`]
module"]
#[doc(alias = "INTR_CAUSE_HI")]
pub type IntrCauseHi = crate::Reg<intr_cause_hi::IntrCauseHiSpec>;
#[doc = "High priority interrupt Cause register"]
pub mod intr_cause_hi;
#[doc = "INTR_CAUSE_MED (r) register accessor: Medium priority interrupt Cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause_med::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_med`]
module"]
#[doc(alias = "INTR_CAUSE_MED")]
pub type IntrCauseMed = crate::Reg<intr_cause_med::IntrCauseMedSpec>;
#[doc = "Medium priority interrupt Cause register"]
pub mod intr_cause_med;
#[doc = "INTR_CAUSE_LO (r) register accessor: Low priority interrupt Cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause_lo`]
module"]
#[doc(alias = "INTR_CAUSE_LO")]
pub type IntrCauseLo = crate::Reg<intr_cause_lo::IntrCauseLoSpec>;
#[doc = "Low priority interrupt Cause register"]
pub mod intr_cause_lo;
#[doc = "DFT_CTL (rw) register accessor: DFT control\n\nYou can [`read`](crate::Reg::read) this register and get [`dft_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dft_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dft_ctl`]
module"]
#[doc(alias = "DFT_CTL")]
pub type DftCtl = crate::Reg<dft_ctl::DftCtlSpec>;
#[doc = "DFT control"]
pub mod dft_ctl;
