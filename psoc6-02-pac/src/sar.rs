#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    sample_ctrl: SampleCtrl,
    _reserved2: [u8; 0x08],
    sample_time01: SampleTime01,
    sample_time23: SampleTime23,
    range_thres: RangeThres,
    range_cond: RangeCond,
    chan_en: ChanEn,
    start_ctrl: StartCtrl,
    _reserved8: [u8; 0x58],
    chan_config: [ChanConfig; 16],
    _reserved9: [u8; 0x40],
    chan_work: [ChanWork; 16],
    _reserved10: [u8; 0x40],
    chan_result: [ChanResult; 16],
    _reserved11: [u8; 0x40],
    chan_work_updated: ChanWorkUpdated,
    chan_result_updated: ChanResultUpdated,
    chan_work_newvalue: ChanWorkNewvalue,
    chan_result_newvalue: ChanResultNewvalue,
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    saturate_intr: SaturateIntr,
    saturate_intr_set: SaturateIntrSet,
    saturate_intr_mask: SaturateIntrMask,
    saturate_intr_masked: SaturateIntrMasked,
    range_intr: RangeIntr,
    range_intr_set: RangeIntrSet,
    range_intr_mask: RangeIntrMask,
    range_intr_masked: RangeIntrMasked,
    intr_cause: IntrCause,
    _reserved28: [u8; 0x3c],
    inj_chan_config: InjChanConfig,
    _reserved29: [u8; 0x0c],
    inj_result: InjResult,
    _reserved30: [u8; 0x0c],
    status: Status,
    avg_stat: AvgStat,
    _reserved32: [u8; 0x58],
    mux_switch0: MuxSwitch0,
    mux_switch_clear0: MuxSwitchClear0,
    _reserved34: [u8; 0x38],
    mux_switch_ds_ctrl: MuxSwitchDsCtrl,
    mux_switch_sq_ctrl: MuxSwitchSqCtrl,
    mux_switch_status: MuxSwitchStatus,
    _reserved37: [u8; 0x0bb4],
    ana_trim0: AnaTrim0,
    ana_trim1: AnaTrim1,
}
impl RegisterBlock {
    #[doc = "0x00 - Analog control register."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Sample control register."]
    #[inline(always)]
    pub const fn sample_ctrl(&self) -> &SampleCtrl {
        &self.sample_ctrl
    }
    #[doc = "0x10 - Sample time specification ST0 and ST1"]
    #[inline(always)]
    pub const fn sample_time01(&self) -> &SampleTime01 {
        &self.sample_time01
    }
    #[doc = "0x14 - Sample time specification ST2 and ST3"]
    #[inline(always)]
    pub const fn sample_time23(&self) -> &SampleTime23 {
        &self.sample_time23
    }
    #[doc = "0x18 - Global range detect threshold register."]
    #[inline(always)]
    pub const fn range_thres(&self) -> &RangeThres {
        &self.range_thres
    }
    #[doc = "0x1c - Global range detect mode register."]
    #[inline(always)]
    pub const fn range_cond(&self) -> &RangeCond {
        &self.range_cond
    }
    #[doc = "0x20 - Enable bits for the channels"]
    #[inline(always)]
    pub const fn chan_en(&self) -> &ChanEn {
        &self.chan_en
    }
    #[doc = "0x24 - Start control register (firmware trigger)."]
    #[inline(always)]
    pub const fn start_ctrl(&self) -> &StartCtrl {
        &self.start_ctrl
    }
    #[doc = "0x80..0xc0 - Channel configuration register."]
    #[inline(always)]
    pub const fn chan_config(&self, n: usize) -> &ChanConfig {
        &self.chan_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Channel configuration register."]
    #[inline(always)]
    pub fn chan_config_iter(&self) -> impl Iterator<Item = &ChanConfig> {
        self.chan_config.iter()
    }
    #[doc = "0x100..0x140 - Channel working data register"]
    #[inline(always)]
    pub const fn chan_work(&self, n: usize) -> &ChanWork {
        &self.chan_work[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - Channel working data register"]
    #[inline(always)]
    pub fn chan_work_iter(&self) -> impl Iterator<Item = &ChanWork> {
        self.chan_work.iter()
    }
    #[doc = "0x180..0x1c0 - Channel result data register"]
    #[inline(always)]
    pub const fn chan_result(&self, n: usize) -> &ChanResult {
        &self.chan_result[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1c0 - Channel result data register"]
    #[inline(always)]
    pub fn chan_result_iter(&self) -> impl Iterator<Item = &ChanResult> {
        self.chan_result.iter()
    }
    #[doc = "0x200 - Channel working data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_work_updated(&self) -> &ChanWorkUpdated {
        &self.chan_work_updated
    }
    #[doc = "0x204 - Channel result data register 'updated' bits"]
    #[inline(always)]
    pub const fn chan_result_updated(&self) -> &ChanResultUpdated {
        &self.chan_result_updated
    }
    #[doc = "0x208 - Channel working data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_work_newvalue(&self) -> &ChanWorkNewvalue {
        &self.chan_work_newvalue
    }
    #[doc = "0x20c - Channel result data register 'new value' bits"]
    #[inline(always)]
    pub const fn chan_result_newvalue(&self) -> &ChanResultNewvalue {
        &self.chan_result_newvalue
    }
    #[doc = "0x210 - Interrupt request register."]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x214 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x218 - Interrupt mask register."]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x21c - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x220 - Saturate interrupt request register."]
    #[inline(always)]
    pub const fn saturate_intr(&self) -> &SaturateIntr {
        &self.saturate_intr
    }
    #[doc = "0x224 - Saturate interrupt set request register"]
    #[inline(always)]
    pub const fn saturate_intr_set(&self) -> &SaturateIntrSet {
        &self.saturate_intr_set
    }
    #[doc = "0x228 - Saturate interrupt mask register."]
    #[inline(always)]
    pub const fn saturate_intr_mask(&self) -> &SaturateIntrMask {
        &self.saturate_intr_mask
    }
    #[doc = "0x22c - Saturate interrupt masked request register"]
    #[inline(always)]
    pub const fn saturate_intr_masked(&self) -> &SaturateIntrMasked {
        &self.saturate_intr_masked
    }
    #[doc = "0x230 - Range detect interrupt request register."]
    #[inline(always)]
    pub const fn range_intr(&self) -> &RangeIntr {
        &self.range_intr
    }
    #[doc = "0x234 - Range detect interrupt set request register"]
    #[inline(always)]
    pub const fn range_intr_set(&self) -> &RangeIntrSet {
        &self.range_intr_set
    }
    #[doc = "0x238 - Range detect interrupt mask register."]
    #[inline(always)]
    pub const fn range_intr_mask(&self) -> &RangeIntrMask {
        &self.range_intr_mask
    }
    #[doc = "0x23c - Range interrupt masked request register"]
    #[inline(always)]
    pub const fn range_intr_masked(&self) -> &RangeIntrMasked {
        &self.range_intr_masked
    }
    #[doc = "0x240 - Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &IntrCause {
        &self.intr_cause
    }
    #[doc = "0x280 - Injection channel configuration register."]
    #[inline(always)]
    pub const fn inj_chan_config(&self) -> &InjChanConfig {
        &self.inj_chan_config
    }
    #[doc = "0x290 - Injection channel result register"]
    #[inline(always)]
    pub const fn inj_result(&self) -> &InjResult {
        &self.inj_result
    }
    #[doc = "0x2a0 - Current status of internal SAR registers (mostly for debug)"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x2a4 - Current averaging status (for debug)"]
    #[inline(always)]
    pub const fn avg_stat(&self) -> &AvgStat {
        &self.avg_stat
    }
    #[doc = "0x300 - SARMUX Firmware switch controls"]
    #[inline(always)]
    pub const fn mux_switch0(&self) -> &MuxSwitch0 {
        &self.mux_switch0
    }
    #[doc = "0x304 - SARMUX Firmware switch control clear"]
    #[inline(always)]
    pub const fn mux_switch_clear0(&self) -> &MuxSwitchClear0 {
        &self.mux_switch_clear0
    }
    #[doc = "0x340 - SARMUX switch DSI control"]
    #[inline(always)]
    pub const fn mux_switch_ds_ctrl(&self) -> &MuxSwitchDsCtrl {
        &self.mux_switch_ds_ctrl
    }
    #[doc = "0x344 - SARMUX switch Sar Sequencer control"]
    #[inline(always)]
    pub const fn mux_switch_sq_ctrl(&self) -> &MuxSwitchSqCtrl {
        &self.mux_switch_sq_ctrl
    }
    #[doc = "0x348 - SARMUX switch status"]
    #[inline(always)]
    pub const fn mux_switch_status(&self) -> &MuxSwitchStatus {
        &self.mux_switch_status
    }
    #[doc = "0xf00 - Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim0(&self) -> &AnaTrim0 {
        &self.ana_trim0
    }
    #[doc = "0xf04 - Analog trim register."]
    #[inline(always)]
    pub const fn ana_trim1(&self) -> &AnaTrim1 {
        &self.ana_trim1
    }
}
#[doc = "CTRL (rw) register accessor: Analog control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Analog control register."]
pub mod ctrl;
#[doc = "SAMPLE_CTRL (rw) register accessor: Sample control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_ctrl`]
module"]
#[doc(alias = "SAMPLE_CTRL")]
pub type SampleCtrl = crate::Reg<sample_ctrl::SampleCtrlSpec>;
#[doc = "Sample control register."]
pub mod sample_ctrl;
#[doc = "SAMPLE_TIME01 (rw) register accessor: Sample time specification ST0 and ST1\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_time01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_time01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_time01`]
module"]
#[doc(alias = "SAMPLE_TIME01")]
pub type SampleTime01 = crate::Reg<sample_time01::SampleTime01Spec>;
#[doc = "Sample time specification ST0 and ST1"]
pub mod sample_time01;
#[doc = "SAMPLE_TIME23 (rw) register accessor: Sample time specification ST2 and ST3\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_time23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_time23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sample_time23`]
module"]
#[doc(alias = "SAMPLE_TIME23")]
pub type SampleTime23 = crate::Reg<sample_time23::SampleTime23Spec>;
#[doc = "Sample time specification ST2 and ST3"]
pub mod sample_time23;
#[doc = "RANGE_THRES (rw) register accessor: Global range detect threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_thres`]
module"]
#[doc(alias = "RANGE_THRES")]
pub type RangeThres = crate::Reg<range_thres::RangeThresSpec>;
#[doc = "Global range detect threshold register."]
pub mod range_thres;
#[doc = "RANGE_COND (rw) register accessor: Global range detect mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_cond::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_cond::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_cond`]
module"]
#[doc(alias = "RANGE_COND")]
pub type RangeCond = crate::Reg<range_cond::RangeCondSpec>;
#[doc = "Global range detect mode register."]
pub mod range_cond;
#[doc = "CHAN_EN (rw) register accessor: Enable bits for the channels\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_en`]
module"]
#[doc(alias = "CHAN_EN")]
pub type ChanEn = crate::Reg<chan_en::ChanEnSpec>;
#[doc = "Enable bits for the channels"]
pub mod chan_en;
#[doc = "START_CTRL (rw) register accessor: Start control register (firmware trigger).\n\nYou can [`read`](crate::Reg::read) this register and get [`start_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start_ctrl`]
module"]
#[doc(alias = "START_CTRL")]
pub type StartCtrl = crate::Reg<start_ctrl::StartCtrlSpec>;
#[doc = "Start control register (firmware trigger)."]
pub mod start_ctrl;
#[doc = "CHAN_CONFIG (rw) register accessor: Channel configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_config`]
module"]
#[doc(alias = "CHAN_CONFIG")]
pub type ChanConfig = crate::Reg<chan_config::ChanConfigSpec>;
#[doc = "Channel configuration register."]
pub mod chan_config;
#[doc = "CHAN_WORK (r) register accessor: Channel working data register\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_work::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work`]
module"]
#[doc(alias = "CHAN_WORK")]
pub type ChanWork = crate::Reg<chan_work::ChanWorkSpec>;
#[doc = "Channel working data register"]
pub mod chan_work;
#[doc = "CHAN_RESULT (r) register accessor: Channel result data register\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result`]
module"]
#[doc(alias = "CHAN_RESULT")]
pub type ChanResult = crate::Reg<chan_result::ChanResultSpec>;
#[doc = "Channel result data register"]
pub mod chan_result;
#[doc = "CHAN_WORK_UPDATED (r) register accessor: Channel working data register 'updated' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_work_updated::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work_updated`]
module"]
#[doc(alias = "CHAN_WORK_UPDATED")]
pub type ChanWorkUpdated = crate::Reg<chan_work_updated::ChanWorkUpdatedSpec>;
#[doc = "Channel working data register 'updated' bits"]
pub mod chan_work_updated;
#[doc = "CHAN_RESULT_UPDATED (r) register accessor: Channel result data register 'updated' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result_updated::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result_updated`]
module"]
#[doc(alias = "CHAN_RESULT_UPDATED")]
pub type ChanResultUpdated = crate::Reg<chan_result_updated::ChanResultUpdatedSpec>;
#[doc = "Channel result data register 'updated' bits"]
pub mod chan_result_updated;
#[doc = "CHAN_WORK_NEWVALUE (r) register accessor: Channel working data register 'new value' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_work_newvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_work_newvalue`]
module"]
#[doc(alias = "CHAN_WORK_NEWVALUE")]
pub type ChanWorkNewvalue = crate::Reg<chan_work_newvalue::ChanWorkNewvalueSpec>;
#[doc = "Channel working data register 'new value' bits"]
pub mod chan_work_newvalue;
#[doc = "CHAN_RESULT_NEWVALUE (r) register accessor: Channel result data register 'new value' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result_newvalue::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chan_result_newvalue`]
module"]
#[doc(alias = "CHAN_RESULT_NEWVALUE")]
pub type ChanResultNewvalue = crate::Reg<chan_result_newvalue::ChanResultNewvalueSpec>;
#[doc = "Channel result data register 'new value' bits"]
pub mod chan_result_newvalue;
#[doc = "INTR (rw) register accessor: Interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt request register."]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set request register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask register."]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked request register"]
pub mod intr_masked;
#[doc = "SATURATE_INTR (rw) register accessor: Saturate interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr`]
module"]
#[doc(alias = "SATURATE_INTR")]
pub type SaturateIntr = crate::Reg<saturate_intr::SaturateIntrSpec>;
#[doc = "Saturate interrupt request register."]
pub mod saturate_intr;
#[doc = "SATURATE_INTR_SET (rw) register accessor: Saturate interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_set`]
module"]
#[doc(alias = "SATURATE_INTR_SET")]
pub type SaturateIntrSet = crate::Reg<saturate_intr_set::SaturateIntrSetSpec>;
#[doc = "Saturate interrupt set request register"]
pub mod saturate_intr_set;
#[doc = "SATURATE_INTR_MASK (rw) register accessor: Saturate interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_mask`]
module"]
#[doc(alias = "SATURATE_INTR_MASK")]
pub type SaturateIntrMask = crate::Reg<saturate_intr_mask::SaturateIntrMaskSpec>;
#[doc = "Saturate interrupt mask register."]
pub mod saturate_intr_mask;
#[doc = "SATURATE_INTR_MASKED (r) register accessor: Saturate interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saturate_intr_masked`]
module"]
#[doc(alias = "SATURATE_INTR_MASKED")]
pub type SaturateIntrMasked = crate::Reg<saturate_intr_masked::SaturateIntrMaskedSpec>;
#[doc = "Saturate interrupt masked request register"]
pub mod saturate_intr_masked;
#[doc = "RANGE_INTR (rw) register accessor: Range detect interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr`]
module"]
#[doc(alias = "RANGE_INTR")]
pub type RangeIntr = crate::Reg<range_intr::RangeIntrSpec>;
#[doc = "Range detect interrupt request register."]
pub mod range_intr;
#[doc = "RANGE_INTR_SET (rw) register accessor: Range detect interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_set`]
module"]
#[doc(alias = "RANGE_INTR_SET")]
pub type RangeIntrSet = crate::Reg<range_intr_set::RangeIntrSetSpec>;
#[doc = "Range detect interrupt set request register"]
pub mod range_intr_set;
#[doc = "RANGE_INTR_MASK (rw) register accessor: Range detect interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_mask`]
module"]
#[doc(alias = "RANGE_INTR_MASK")]
pub type RangeIntrMask = crate::Reg<range_intr_mask::RangeIntrMaskSpec>;
#[doc = "Range detect interrupt mask register."]
pub mod range_intr_mask;
#[doc = "RANGE_INTR_MASKED (r) register accessor: Range interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@range_intr_masked`]
module"]
#[doc(alias = "RANGE_INTR_MASKED")]
pub type RangeIntrMasked = crate::Reg<range_intr_masked::RangeIntrMaskedSpec>;
#[doc = "Range interrupt masked request register"]
pub mod range_intr_masked;
#[doc = "INTR_CAUSE (r) register accessor: Interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
#[doc(alias = "INTR_CAUSE")]
pub type IntrCause = crate::Reg<intr_cause::IntrCauseSpec>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "INJ_CHAN_CONFIG (rw) register accessor: Injection channel configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`inj_chan_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inj_chan_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inj_chan_config`]
module"]
#[doc(alias = "INJ_CHAN_CONFIG")]
pub type InjChanConfig = crate::Reg<inj_chan_config::InjChanConfigSpec>;
#[doc = "Injection channel configuration register."]
pub mod inj_chan_config;
#[doc = "INJ_RESULT (r) register accessor: Injection channel result register\n\nYou can [`read`](crate::Reg::read) this register and get [`inj_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inj_result`]
module"]
#[doc(alias = "INJ_RESULT")]
pub type InjResult = crate::Reg<inj_result::InjResultSpec>;
#[doc = "Injection channel result register"]
pub mod inj_result;
#[doc = "STATUS (r) register accessor: Current status of internal SAR registers (mostly for debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Current status of internal SAR registers (mostly for debug)"]
pub mod status;
#[doc = "AVG_STAT (r) register accessor: Current averaging status (for debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`avg_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@avg_stat`]
module"]
#[doc(alias = "AVG_STAT")]
pub type AvgStat = crate::Reg<avg_stat::AvgStatSpec>;
#[doc = "Current averaging status (for debug)"]
pub mod avg_stat;
#[doc = "MUX_SWITCH0 (rw) register accessor: SARMUX Firmware switch controls\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch0`]
module"]
#[doc(alias = "MUX_SWITCH0")]
pub type MuxSwitch0 = crate::Reg<mux_switch0::MuxSwitch0Spec>;
#[doc = "SARMUX Firmware switch controls"]
pub mod mux_switch0;
#[doc = "MUX_SWITCH_CLEAR0 (rw) register accessor: SARMUX Firmware switch control clear\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_clear0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch_clear0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_clear0`]
module"]
#[doc(alias = "MUX_SWITCH_CLEAR0")]
pub type MuxSwitchClear0 = crate::Reg<mux_switch_clear0::MuxSwitchClear0Spec>;
#[doc = "SARMUX Firmware switch control clear"]
pub mod mux_switch_clear0;
#[doc = "MUX_SWITCH_DS_CTRL (rw) register accessor: SARMUX switch DSI control\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_ds_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch_ds_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_ds_ctrl`]
module"]
#[doc(alias = "MUX_SWITCH_DS_CTRL")]
pub type MuxSwitchDsCtrl = crate::Reg<mux_switch_ds_ctrl::MuxSwitchDsCtrlSpec>;
#[doc = "SARMUX switch DSI control"]
pub mod mux_switch_ds_ctrl;
#[doc = "MUX_SWITCH_SQ_CTRL (rw) register accessor: SARMUX switch Sar Sequencer control\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_sq_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mux_switch_sq_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_sq_ctrl`]
module"]
#[doc(alias = "MUX_SWITCH_SQ_CTRL")]
pub type MuxSwitchSqCtrl = crate::Reg<mux_switch_sq_ctrl::MuxSwitchSqCtrlSpec>;
#[doc = "SARMUX switch Sar Sequencer control"]
pub mod mux_switch_sq_ctrl;
#[doc = "MUX_SWITCH_STATUS (r) register accessor: SARMUX switch status\n\nYou can [`read`](crate::Reg::read) this register and get [`mux_switch_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux_switch_status`]
module"]
#[doc(alias = "MUX_SWITCH_STATUS")]
pub type MuxSwitchStatus = crate::Reg<mux_switch_status::MuxSwitchStatusSpec>;
#[doc = "SARMUX switch status"]
pub mod mux_switch_status;
#[doc = "ANA_TRIM0 (rw) register accessor: Analog trim register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_trim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_trim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_trim0`]
module"]
#[doc(alias = "ANA_TRIM0")]
pub type AnaTrim0 = crate::Reg<ana_trim0::AnaTrim0Spec>;
#[doc = "Analog trim register."]
pub mod ana_trim0;
#[doc = "ANA_TRIM1 (rw) register accessor: Analog trim register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_trim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_trim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_trim1`]
module"]
#[doc(alias = "ANA_TRIM1")]
pub type AnaTrim1 = crate::Reg<ana_trim1::AnaTrim1Spec>;
#[doc = "Analog trim register."]
pub mod ana_trim1;
