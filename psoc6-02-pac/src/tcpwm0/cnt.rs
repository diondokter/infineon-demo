#[repr(C)]
#[doc = "Timer/Counter/PWM Counter Module"]
#[doc(alias = "CNT")]
pub struct Cnt {
    ctrl: Ctrl,
    status: Status,
    counter: Counter,
    cc: Cc,
    cc_buff: CcBuff,
    period: Period,
    period_buff: PeriodBuff,
    _reserved7: [u8; 0x04],
    tr_ctrl0: TrCtrl0,
    tr_ctrl1: TrCtrl1,
    tr_ctrl2: TrCtrl2,
    _reserved10: [u8; 0x04],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
}
impl Cnt {
    #[doc = "0x00 - Counter control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Counter status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Counter count register"]
    #[inline(always)]
    pub const fn counter(&self) -> &Counter {
        &self.counter
    }
    #[doc = "0x0c - Counter compare/capture register"]
    #[inline(always)]
    pub const fn cc(&self) -> &Cc {
        &self.cc
    }
    #[doc = "0x10 - Counter buffered compare/capture register"]
    #[inline(always)]
    pub const fn cc_buff(&self) -> &CcBuff {
        &self.cc_buff
    }
    #[doc = "0x14 - Counter period register"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x18 - Counter buffered period register"]
    #[inline(always)]
    pub const fn period_buff(&self) -> &PeriodBuff {
        &self.period_buff
    }
    #[doc = "0x20 - Counter trigger control register 0"]
    #[inline(always)]
    pub const fn tr_ctrl0(&self) -> &TrCtrl0 {
        &self.tr_ctrl0
    }
    #[doc = "0x24 - Counter trigger control register 1"]
    #[inline(always)]
    pub const fn tr_ctrl1(&self) -> &TrCtrl1 {
        &self.tr_ctrl1
    }
    #[doc = "0x28 - Counter trigger control register 2"]
    #[inline(always)]
    pub const fn tr_ctrl2(&self) -> &TrCtrl2 {
        &self.tr_ctrl2
    }
    #[doc = "0x30 - Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x34 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x38 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x3c - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
}
#[doc = "CTRL (rw) register accessor: Counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Counter control register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Counter status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Counter status register"]
pub mod status;
#[doc = "COUNTER (rw) register accessor: Counter count register\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@counter`]
module"]
#[doc(alias = "COUNTER")]
pub type Counter = crate::Reg<counter::CounterSpec>;
#[doc = "Counter count register"]
pub mod counter;
#[doc = "CC (rw) register accessor: Counter compare/capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc`]
module"]
#[doc(alias = "CC")]
pub type Cc = crate::Reg<cc::CcSpec>;
#[doc = "Counter compare/capture register"]
pub mod cc;
#[doc = "CC_BUFF (rw) register accessor: Counter buffered compare/capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_buff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_buff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc_buff`]
module"]
#[doc(alias = "CC_BUFF")]
pub type CcBuff = crate::Reg<cc_buff::CcBuffSpec>;
#[doc = "Counter buffered compare/capture register"]
pub mod cc_buff;
#[doc = "PERIOD (rw) register accessor: Counter period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`]
module"]
#[doc(alias = "PERIOD")]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "Counter period register"]
pub mod period;
#[doc = "PERIOD_BUFF (rw) register accessor: Counter buffered period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period_buff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period_buff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period_buff`]
module"]
#[doc(alias = "PERIOD_BUFF")]
pub type PeriodBuff = crate::Reg<period_buff::PeriodBuffSpec>;
#[doc = "Counter buffered period register"]
pub mod period_buff;
#[doc = "TR_CTRL0 (rw) register accessor: Counter trigger control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl0`]
module"]
#[doc(alias = "TR_CTRL0")]
pub type TrCtrl0 = crate::Reg<tr_ctrl0::TrCtrl0Spec>;
#[doc = "Counter trigger control register 0"]
pub mod tr_ctrl0;
#[doc = "TR_CTRL1 (rw) register accessor: Counter trigger control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl1`]
module"]
#[doc(alias = "TR_CTRL1")]
pub type TrCtrl1 = crate::Reg<tr_ctrl1::TrCtrl1Spec>;
#[doc = "Counter trigger control register 1"]
pub mod tr_ctrl1;
#[doc = "TR_CTRL2 (rw) register accessor: Counter trigger control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctrl2`]
module"]
#[doc(alias = "TR_CTRL2")]
pub type TrCtrl2 = crate::Reg<tr_ctrl2::TrCtrl2Spec>;
#[doc = "Counter trigger control register 2"]
pub mod tr_ctrl2;
#[doc = "INTR (rw) register accessor: Interrupt request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt request register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
