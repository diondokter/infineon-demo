#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    status: Status,
    _reserved2: [u8; 0x08],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    _reserved6: [u8; 0x20],
    cmp0_ctrl: Cmp0Ctrl,
    _reserved7: [u8; 0x0c],
    cmp0_sw: Cmp0Sw,
    cmp0_sw_clear: Cmp0SwClear,
    _reserved9: [u8; 0x28],
    cmp1_ctrl: Cmp1Ctrl,
    _reserved10: [u8; 0x0c],
    cmp1_sw: Cmp1Sw,
    cmp1_sw_clear: Cmp1SwClear,
}
impl RegisterBlock {
    #[doc = "0x00 - LPCOMP Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - LPCOMP Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - LPCOMP Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x14 - LPCOMP Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x18 - LPCOMP Interrupt request mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x1c - LPCOMP Interrupt request masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x40 - Comparator 0 control Register"]
    #[inline(always)]
    pub const fn cmp0_ctrl(&self) -> &Cmp0Ctrl {
        &self.cmp0_ctrl
    }
    #[doc = "0x50 - Comparator 0 switch control"]
    #[inline(always)]
    pub const fn cmp0_sw(&self) -> &Cmp0Sw {
        &self.cmp0_sw
    }
    #[doc = "0x54 - Comparator 0 switch control clear"]
    #[inline(always)]
    pub const fn cmp0_sw_clear(&self) -> &Cmp0SwClear {
        &self.cmp0_sw_clear
    }
    #[doc = "0x80 - Comparator 1 control Register"]
    #[inline(always)]
    pub const fn cmp1_ctrl(&self) -> &Cmp1Ctrl {
        &self.cmp1_ctrl
    }
    #[doc = "0x90 - Comparator 1 switch control"]
    #[inline(always)]
    pub const fn cmp1_sw(&self) -> &Cmp1Sw {
        &self.cmp1_sw
    }
    #[doc = "0x94 - Comparator 1 switch control clear"]
    #[inline(always)]
    pub const fn cmp1_sw_clear(&self) -> &Cmp1SwClear {
        &self.cmp1_sw_clear
    }
}
#[doc = "CONFIG (rw) register accessor: LPCOMP Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "LPCOMP Configuration Register"]
pub mod config;
#[doc = "STATUS (r) register accessor: LPCOMP Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "LPCOMP Status Register"]
pub mod status;
#[doc = "INTR (rw) register accessor: LPCOMP Interrupt request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "LPCOMP Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: LPCOMP Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "LPCOMP Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: LPCOMP Interrupt request mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "LPCOMP Interrupt request mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: LPCOMP Interrupt request masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "LPCOMP Interrupt request masked"]
pub mod intr_masked;
#[doc = "CMP0_CTRL (rw) register accessor: Comparator 0 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_ctrl`]
module"]
#[doc(alias = "CMP0_CTRL")]
pub type Cmp0Ctrl = crate::Reg<cmp0_ctrl::Cmp0CtrlSpec>;
#[doc = "Comparator 0 control Register"]
pub mod cmp0_ctrl;
#[doc = "CMP0_SW (rw) register accessor: Comparator 0 switch control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_sw`]
module"]
#[doc(alias = "CMP0_SW")]
pub type Cmp0Sw = crate::Reg<cmp0_sw::Cmp0SwSpec>;
#[doc = "Comparator 0 switch control"]
pub mod cmp0_sw;
#[doc = "CMP0_SW_CLEAR (rw) register accessor: Comparator 0 switch control clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_sw_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_sw_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp0_sw_clear`]
module"]
#[doc(alias = "CMP0_SW_CLEAR")]
pub type Cmp0SwClear = crate::Reg<cmp0_sw_clear::Cmp0SwClearSpec>;
#[doc = "Comparator 0 switch control clear"]
pub mod cmp0_sw_clear;
#[doc = "CMP1_CTRL (rw) register accessor: Comparator 1 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_ctrl`]
module"]
#[doc(alias = "CMP1_CTRL")]
pub type Cmp1Ctrl = crate::Reg<cmp1_ctrl::Cmp1CtrlSpec>;
#[doc = "Comparator 1 control Register"]
pub mod cmp1_ctrl;
#[doc = "CMP1_SW (rw) register accessor: Comparator 1 switch control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_sw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_sw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_sw`]
module"]
#[doc(alias = "CMP1_SW")]
pub type Cmp1Sw = crate::Reg<cmp1_sw::Cmp1SwSpec>;
#[doc = "Comparator 1 switch control"]
pub mod cmp1_sw;
#[doc = "CMP1_SW_CLEAR (rw) register accessor: Comparator 1 switch control clear\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_sw_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_sw_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmp1_sw_clear`]
module"]
#[doc(alias = "CMP1_SW_CLEAR")]
pub type Cmp1SwClear = crate::Reg<cmp1_sw_clear::Cmp1SwClearSpec>;
#[doc = "Comparator 1 switch control clear"]
pub mod cmp1_sw_clear;
