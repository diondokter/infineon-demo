#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    _reserved1: [u8; 0x04],
    rtc_rw: RtcRw,
    cal_ctl: CalCtl,
    status: Status,
    rtc_time: RtcTime,
    rtc_date: RtcDate,
    alm1_time: Alm1Time,
    alm1_date: Alm1Date,
    alm2_time: Alm2Time,
    alm2_date: Alm2Date,
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    osccnt: Osccnt,
    ticks: Ticks,
    pmic_ctl: PmicCtl,
    reset: Reset,
    _reserved18: [u8; 0x0fb4],
    breg: [Breg; 64],
    _reserved19: [u8; 0xee00],
    trim: Trim,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - RTC Read Write register"]
    #[inline(always)]
    pub const fn rtc_rw(&self) -> &RtcRw {
        &self.rtc_rw
    }
    #[doc = "0x0c - Oscillator calibration for absolute frequency"]
    #[inline(always)]
    pub const fn cal_ctl(&self) -> &CalCtl {
        &self.cal_ctl
    }
    #[doc = "0x10 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - Calendar Seconds, Minutes, Hours, Day of Week"]
    #[inline(always)]
    pub const fn rtc_time(&self) -> &RtcTime {
        &self.rtc_time
    }
    #[doc = "0x18 - Calendar Day of Month, Month, Year"]
    #[inline(always)]
    pub const fn rtc_date(&self) -> &RtcDate {
        &self.rtc_date
    }
    #[doc = "0x1c - Alarm 1 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm1_time(&self) -> &Alm1Time {
        &self.alm1_time
    }
    #[doc = "0x20 - Alarm 1 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm1_date(&self) -> &Alm1Date {
        &self.alm1_date
    }
    #[doc = "0x24 - Alarm 2 Seconds, Minute, Hours, Day of Week"]
    #[inline(always)]
    pub const fn alm2_time(&self) -> &Alm2Time {
        &self.alm2_time
    }
    #[doc = "0x28 - Alarm 2 Day of Month, Month"]
    #[inline(always)]
    pub const fn alm2_date(&self) -> &Alm2Date {
        &self.alm2_date
    }
    #[doc = "0x2c - Interrupt request register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x30 - Interrupt set request register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x34 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x38 - Interrupt masked request register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x3c - 32kHz oscillator counter"]
    #[inline(always)]
    pub const fn osccnt(&self) -> &Osccnt {
        &self.osccnt
    }
    #[doc = "0x40 - 128Hz tick counter"]
    #[inline(always)]
    pub const fn ticks(&self) -> &Ticks {
        &self.ticks
    }
    #[doc = "0x44 - PMIC control register"]
    #[inline(always)]
    pub const fn pmic_ctl(&self) -> &PmicCtl {
        &self.pmic_ctl
    }
    #[doc = "0x48 - Backup reset register"]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
    #[doc = "0x1000..0x1100 - Backup register region"]
    #[inline(always)]
    pub const fn breg(&self, n: usize) -> &Breg {
        &self.breg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1100 - Backup register region"]
    #[inline(always)]
    pub fn breg_iter(&self) -> impl Iterator<Item = &Breg> {
        self.breg.iter()
    }
    #[doc = "0xff00 - Trim Register"]
    #[inline(always)]
    pub const fn trim(&self) -> &Trim {
        &self.trim
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "RTC_RW (rw) register accessor: RTC Read Write register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_rw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_rw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_rw`]
module"]
#[doc(alias = "RTC_RW")]
pub type RtcRw = crate::Reg<rtc_rw::RtcRwSpec>;
#[doc = "RTC Read Write register"]
pub mod rtc_rw;
#[doc = "CAL_CTL (rw) register accessor: Oscillator calibration for absolute frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl`]
module"]
#[doc(alias = "CAL_CTL")]
pub type CalCtl = crate::Reg<cal_ctl::CalCtlSpec>;
#[doc = "Oscillator calibration for absolute frequency"]
pub mod cal_ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "RTC_TIME (rw) register accessor: Calendar Seconds, Minutes, Hours, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_time`]
module"]
#[doc(alias = "RTC_TIME")]
pub type RtcTime = crate::Reg<rtc_time::RtcTimeSpec>;
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week"]
pub mod rtc_time;
#[doc = "RTC_DATE (rw) register accessor: Calendar Day of Month, Month, Year\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_date`]
module"]
#[doc(alias = "RTC_DATE")]
pub type RtcDate = crate::Reg<rtc_date::RtcDateSpec>;
#[doc = "Calendar Day of Month, Month, Year"]
pub mod rtc_date;
#[doc = "ALM1_TIME (rw) register accessor: Alarm 1 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`alm1_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm1_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm1_time`]
module"]
#[doc(alias = "ALM1_TIME")]
pub type Alm1Time = crate::Reg<alm1_time::Alm1TimeSpec>;
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week"]
pub mod alm1_time;
#[doc = "ALM1_DATE (rw) register accessor: Alarm 1 Day of Month, Month\n\nYou can [`read`](crate::Reg::read) this register and get [`alm1_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm1_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm1_date`]
module"]
#[doc(alias = "ALM1_DATE")]
pub type Alm1Date = crate::Reg<alm1_date::Alm1DateSpec>;
#[doc = "Alarm 1 Day of Month, Month"]
pub mod alm1_date;
#[doc = "ALM2_TIME (rw) register accessor: Alarm 2 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`alm2_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm2_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm2_time`]
module"]
#[doc(alias = "ALM2_TIME")]
pub type Alm2Time = crate::Reg<alm2_time::Alm2TimeSpec>;
#[doc = "Alarm 2 Seconds, Minute, Hours, Day of Week"]
pub mod alm2_time;
#[doc = "ALM2_DATE (rw) register accessor: Alarm 2 Day of Month, Month\n\nYou can [`read`](crate::Reg::read) this register and get [`alm2_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm2_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alm2_date`]
module"]
#[doc(alias = "ALM2_DATE")]
pub type Alm2Date = crate::Reg<alm2_date::Alm2DateSpec>;
#[doc = "Alarm 2 Day of Month, Month"]
pub mod alm2_date;
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
#[doc = "OSCCNT (r) register accessor: 32kHz oscillator counter\n\nYou can [`read`](crate::Reg::read) this register and get [`osccnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccnt`]
module"]
#[doc(alias = "OSCCNT")]
pub type Osccnt = crate::Reg<osccnt::OsccntSpec>;
#[doc = "32kHz oscillator counter"]
pub mod osccnt;
#[doc = "TICKS (r) register accessor: 128Hz tick counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ticks::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ticks`]
module"]
#[doc(alias = "TICKS")]
pub type Ticks = crate::Reg<ticks::TicksSpec>;
#[doc = "128Hz tick counter"]
pub mod ticks;
#[doc = "PMIC_CTL (rw) register accessor: PMIC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmic_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmic_ctl`]
module"]
#[doc(alias = "PMIC_CTL")]
pub type PmicCtl = crate::Reg<pmic_ctl::PmicCtlSpec>;
#[doc = "PMIC control register"]
pub mod pmic_ctl;
#[doc = "RESET (rw) register accessor: Backup reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "RESET")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "Backup reset register"]
pub mod reset;
#[doc = "BREG (rw) register accessor: Backup register region\n\nYou can [`read`](crate::Reg::read) this register and get [`breg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`breg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@breg`]
module"]
#[doc(alias = "BREG")]
pub type Breg = crate::Reg<breg::BregSpec>;
#[doc = "Backup register region"]
pub mod breg;
#[doc = "TRIM (rw) register accessor: Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim`]
module"]
#[doc(alias = "TRIM")]
pub type Trim = crate::Reg<trim::TrimSpec>;
#[doc = "Trim Register"]
pub mod trim;
