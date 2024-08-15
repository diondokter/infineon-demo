#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    spare: Spare,
    _reserved2: [u8; 0x78],
    status: Status,
    stat_seq: StatSeq,
    stat_cnts: StatCnts,
    stat_hcnt: StatHcnt,
    _reserved6: [u8; 0x40],
    result_val1: ResultVal1,
    result_val2: ResultVal2,
    _reserved8: [u8; 0x08],
    adc_res: AdcRes,
    _reserved9: [u8; 0x0c],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    _reserved13: [u8; 0x80],
    hscmp: Hscmp,
    ambuf: Ambuf,
    refgen: Refgen,
    csdcmp: Csdcmp,
    _reserved17: [u8; 0x60],
    sw_res: SwRes,
    _reserved18: [u8; 0x0c],
    sense_period: SensePeriod,
    sense_duty: SenseDuty,
    _reserved20: [u8; 0x78],
    sw_hs_p_sel: SwHsPSel,
    sw_hs_n_sel: SwHsNSel,
    sw_shield_sel: SwShieldSel,
    _reserved23: [u8; 0x04],
    sw_amuxbuf_sel: SwAmuxbufSel,
    sw_byp_sel: SwBypSel,
    _reserved25: [u8; 0x08],
    sw_cmp_p_sel: SwCmpPSel,
    sw_cmp_n_sel: SwCmpNSel,
    sw_refgen_sel: SwRefgenSel,
    _reserved28: [u8; 0x04],
    sw_fw_mod_sel: SwFwModSel,
    sw_fw_tank_sel: SwFwTankSel,
    _reserved30: [u8; 0x08],
    sw_dsi_sel: SwDsiSel,
    _reserved31: [u8; 0x0c],
    io_sel: IoSel,
    _reserved32: [u8; 0x2c],
    seq_time: SeqTime,
    _reserved33: [u8; 0x0c],
    seq_init_cnt: SeqInitCnt,
    seq_norm_cnt: SeqNormCnt,
    _reserved35: [u8; 0x08],
    adc_ctl: AdcCtl,
    _reserved36: [u8; 0x1c],
    seq_start: SeqStart,
    _reserved37: [u8; 0xbc],
    idaca: Idaca,
    _reserved38: [u8; 0xfc],
    idacb: Idacb,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration and Control"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Spare MMIO"]
    #[inline(always)]
    pub const fn spare(&self) -> &Spare {
        &self.spare
    }
    #[doc = "0x80 - Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x84 - Current Sequencer status"]
    #[inline(always)]
    pub const fn stat_seq(&self) -> &StatSeq {
        &self.stat_seq
    }
    #[doc = "0x88 - Current status counts"]
    #[inline(always)]
    pub const fn stat_cnts(&self) -> &StatCnts {
        &self.stat_cnts
    }
    #[doc = "0x8c - Current count of the HSCMP counter"]
    #[inline(always)]
    pub const fn stat_hcnt(&self) -> &StatHcnt {
        &self.stat_hcnt
    }
    #[doc = "0xd0 - Result CSD/CSX accumulation counter value 1"]
    #[inline(always)]
    pub const fn result_val1(&self) -> &ResultVal1 {
        &self.result_val1
    }
    #[doc = "0xd4 - Result CSX accumulation counter value 2"]
    #[inline(always)]
    pub const fn result_val2(&self) -> &ResultVal2 {
        &self.result_val2
    }
    #[doc = "0xe0 - ADC measurement"]
    #[inline(always)]
    pub const fn adc_res(&self) -> &AdcRes {
        &self.adc_res
    }
    #[doc = "0xf0 - CSD Interrupt Request Register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0xf4 - CSD Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0xf8 - CSD Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0xfc - CSD Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x180 - High Speed Comparator configuration"]
    #[inline(always)]
    pub const fn hscmp(&self) -> &Hscmp {
        &self.hscmp
    }
    #[doc = "0x184 - Reference Generator configuration"]
    #[inline(always)]
    pub const fn ambuf(&self) -> &Ambuf {
        &self.ambuf
    }
    #[doc = "0x188 - Reference Generator configuration"]
    #[inline(always)]
    pub const fn refgen(&self) -> &Refgen {
        &self.refgen
    }
    #[doc = "0x18c - CSD Comparator configuration"]
    #[inline(always)]
    pub const fn csdcmp(&self) -> &Csdcmp {
        &self.csdcmp
    }
    #[doc = "0x1f0 - Switch Resistance configuration"]
    #[inline(always)]
    pub const fn sw_res(&self) -> &SwRes {
        &self.sw_res
    }
    #[doc = "0x200 - Sense clock period"]
    #[inline(always)]
    pub const fn sense_period(&self) -> &SensePeriod {
        &self.sense_period
    }
    #[doc = "0x204 - Sense clock duty cycle"]
    #[inline(always)]
    pub const fn sense_duty(&self) -> &SenseDuty {
        &self.sense_duty
    }
    #[doc = "0x280 - HSCMP Pos input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_p_sel(&self) -> &SwHsPSel {
        &self.sw_hs_p_sel
    }
    #[doc = "0x284 - HSCMP Neg input switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_hs_n_sel(&self) -> &SwHsNSel {
        &self.sw_hs_n_sel
    }
    #[doc = "0x288 - Shielding switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_shield_sel(&self) -> &SwShieldSel {
        &self.sw_shield_sel
    }
    #[doc = "0x290 - Amuxbuffer switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_amuxbuf_sel(&self) -> &SwAmuxbufSel {
        &self.sw_amuxbuf_sel
    }
    #[doc = "0x294 - AMUXBUS bypass switches Waveform selection"]
    #[inline(always)]
    pub const fn sw_byp_sel(&self) -> &SwBypSel {
        &self.sw_byp_sel
    }
    #[doc = "0x2a0 - CSDCMP Pos Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_p_sel(&self) -> &SwCmpPSel {
        &self.sw_cmp_p_sel
    }
    #[doc = "0x2a4 - CSDCMP Neg Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_cmp_n_sel(&self) -> &SwCmpNSel {
        &self.sw_cmp_n_sel
    }
    #[doc = "0x2a8 - Reference Generator Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_refgen_sel(&self) -> &SwRefgenSel {
        &self.sw_refgen_sel
    }
    #[doc = "0x2b0 - Full Wave Cmod Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_mod_sel(&self) -> &SwFwModSel {
        &self.sw_fw_mod_sel
    }
    #[doc = "0x2b4 - Full Wave Csh_tank Switch Waveform selection"]
    #[inline(always)]
    pub const fn sw_fw_tank_sel(&self) -> &SwFwTankSel {
        &self.sw_fw_tank_sel
    }
    #[doc = "0x2c0 - DSI output switch control Waveform selection"]
    #[inline(always)]
    pub const fn sw_dsi_sel(&self) -> &SwDsiSel {
        &self.sw_dsi_sel
    }
    #[doc = "0x2d0 - IO output control Waveform selection"]
    #[inline(always)]
    pub const fn io_sel(&self) -> &IoSel {
        &self.io_sel
    }
    #[doc = "0x300 - Sequencer Timing"]
    #[inline(always)]
    pub const fn seq_time(&self) -> &SeqTime {
        &self.seq_time
    }
    #[doc = "0x310 - Sequencer Initial conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_init_cnt(&self) -> &SeqInitCnt {
        &self.seq_init_cnt
    }
    #[doc = "0x314 - Sequencer Normal conversion and sample counts"]
    #[inline(always)]
    pub const fn seq_norm_cnt(&self) -> &SeqNormCnt {
        &self.seq_norm_cnt
    }
    #[doc = "0x320 - ADC Control"]
    #[inline(always)]
    pub const fn adc_ctl(&self) -> &AdcCtl {
        &self.adc_ctl
    }
    #[doc = "0x340 - Sequencer start"]
    #[inline(always)]
    pub const fn seq_start(&self) -> &SeqStart {
        &self.seq_start
    }
    #[doc = "0x400 - IDACA Configuration"]
    #[inline(always)]
    pub const fn idaca(&self) -> &Idaca {
        &self.idaca
    }
    #[doc = "0x500 - IDACB Configuration"]
    #[inline(always)]
    pub const fn idacb(&self) -> &Idacb {
        &self.idacb
    }
}
#[doc = "CONFIG (rw) register accessor: Configuration and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Configuration and Control"]
pub mod config;
#[doc = "SPARE (rw) register accessor: Spare MMIO\n\nYou can [`read`](crate::Reg::read) this register and get [`spare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spare`]
module"]
#[doc(alias = "SPARE")]
pub type Spare = crate::Reg<spare::SpareSpec>;
#[doc = "Spare MMIO"]
pub mod spare;
#[doc = "STATUS (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status Register"]
pub mod status;
#[doc = "STAT_SEQ (r) register accessor: Current Sequencer status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_seq::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_seq`]
module"]
#[doc(alias = "STAT_SEQ")]
pub type StatSeq = crate::Reg<stat_seq::StatSeqSpec>;
#[doc = "Current Sequencer status"]
pub mod stat_seq;
#[doc = "STAT_CNTS (r) register accessor: Current status counts\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cnts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_cnts`]
module"]
#[doc(alias = "STAT_CNTS")]
pub type StatCnts = crate::Reg<stat_cnts::StatCntsSpec>;
#[doc = "Current status counts"]
pub mod stat_cnts;
#[doc = "STAT_HCNT (r) register accessor: Current count of the HSCMP counter\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_hcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat_hcnt`]
module"]
#[doc(alias = "STAT_HCNT")]
pub type StatHcnt = crate::Reg<stat_hcnt::StatHcntSpec>;
#[doc = "Current count of the HSCMP counter"]
pub mod stat_hcnt;
#[doc = "RESULT_VAL1 (r) register accessor: Result CSD/CSX accumulation counter value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`result_val1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_val1`]
module"]
#[doc(alias = "RESULT_VAL1")]
pub type ResultVal1 = crate::Reg<result_val1::ResultVal1Spec>;
#[doc = "Result CSD/CSX accumulation counter value 1"]
pub mod result_val1;
#[doc = "RESULT_VAL2 (r) register accessor: Result CSX accumulation counter value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`result_val2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result_val2`]
module"]
#[doc(alias = "RESULT_VAL2")]
pub type ResultVal2 = crate::Reg<result_val2::ResultVal2Spec>;
#[doc = "Result CSX accumulation counter value 2"]
pub mod result_val2;
#[doc = "ADC_RES (r) register accessor: ADC measurement\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_res::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_res`]
module"]
#[doc(alias = "ADC_RES")]
pub type AdcRes = crate::Reg<adc_res::AdcResSpec>;
#[doc = "ADC measurement"]
pub mod adc_res;
#[doc = "INTR (rw) register accessor: CSD Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "CSD Interrupt Request Register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: CSD Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "CSD Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: CSD Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "CSD Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: CSD Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "CSD Interrupt masked register"]
pub mod intr_masked;
#[doc = "HSCMP (rw) register accessor: High Speed Comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hscmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hscmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hscmp`]
module"]
#[doc(alias = "HSCMP")]
pub type Hscmp = crate::Reg<hscmp::HscmpSpec>;
#[doc = "High Speed Comparator configuration"]
pub mod hscmp;
#[doc = "AMBUF (rw) register accessor: Reference Generator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ambuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ambuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ambuf`]
module"]
#[doc(alias = "AMBUF")]
pub type Ambuf = crate::Reg<ambuf::AmbufSpec>;
#[doc = "Reference Generator configuration"]
pub mod ambuf;
#[doc = "REFGEN (rw) register accessor: Reference Generator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`refgen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refgen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refgen`]
module"]
#[doc(alias = "REFGEN")]
pub type Refgen = crate::Reg<refgen::RefgenSpec>;
#[doc = "Reference Generator configuration"]
pub mod refgen;
#[doc = "CSDCMP (rw) register accessor: CSD Comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csdcmp`]
module"]
#[doc(alias = "CSDCMP")]
pub type Csdcmp = crate::Reg<csdcmp::CsdcmpSpec>;
#[doc = "CSD Comparator configuration"]
pub mod csdcmp;
#[doc = "SW_RES (rw) register accessor: Switch Resistance configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_res`]
module"]
#[doc(alias = "SW_RES")]
pub type SwRes = crate::Reg<sw_res::SwResSpec>;
#[doc = "Switch Resistance configuration"]
pub mod sw_res;
#[doc = "SENSE_PERIOD (rw) register accessor: Sense clock period\n\nYou can [`read`](crate::Reg::read) this register and get [`sense_period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sense_period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sense_period`]
module"]
#[doc(alias = "SENSE_PERIOD")]
pub type SensePeriod = crate::Reg<sense_period::SensePeriodSpec>;
#[doc = "Sense clock period"]
pub mod sense_period;
#[doc = "SENSE_DUTY (rw) register accessor: Sense clock duty cycle\n\nYou can [`read`](crate::Reg::read) this register and get [`sense_duty::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sense_duty::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sense_duty`]
module"]
#[doc(alias = "SENSE_DUTY")]
pub type SenseDuty = crate::Reg<sense_duty::SenseDutySpec>;
#[doc = "Sense clock duty cycle"]
pub mod sense_duty;
#[doc = "SW_HS_P_SEL (rw) register accessor: HSCMP Pos input switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_hs_p_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_hs_p_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_hs_p_sel`]
module"]
#[doc(alias = "SW_HS_P_SEL")]
pub type SwHsPSel = crate::Reg<sw_hs_p_sel::SwHsPSelSpec>;
#[doc = "HSCMP Pos input switch Waveform selection"]
pub mod sw_hs_p_sel;
#[doc = "SW_HS_N_SEL (rw) register accessor: HSCMP Neg input switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_hs_n_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_hs_n_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_hs_n_sel`]
module"]
#[doc(alias = "SW_HS_N_SEL")]
pub type SwHsNSel = crate::Reg<sw_hs_n_sel::SwHsNSelSpec>;
#[doc = "HSCMP Neg input switch Waveform selection"]
pub mod sw_hs_n_sel;
#[doc = "SW_SHIELD_SEL (rw) register accessor: Shielding switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_shield_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_shield_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_shield_sel`]
module"]
#[doc(alias = "SW_SHIELD_SEL")]
pub type SwShieldSel = crate::Reg<sw_shield_sel::SwShieldSelSpec>;
#[doc = "Shielding switches Waveform selection"]
pub mod sw_shield_sel;
#[doc = "SW_AMUXBUF_SEL (rw) register accessor: Amuxbuffer switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_amuxbuf_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_amuxbuf_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_amuxbuf_sel`]
module"]
#[doc(alias = "SW_AMUXBUF_SEL")]
pub type SwAmuxbufSel = crate::Reg<sw_amuxbuf_sel::SwAmuxbufSelSpec>;
#[doc = "Amuxbuffer switches Waveform selection"]
pub mod sw_amuxbuf_sel;
#[doc = "SW_BYP_SEL (rw) register accessor: AMUXBUS bypass switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_byp_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_byp_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_byp_sel`]
module"]
#[doc(alias = "SW_BYP_SEL")]
pub type SwBypSel = crate::Reg<sw_byp_sel::SwBypSelSpec>;
#[doc = "AMUXBUS bypass switches Waveform selection"]
pub mod sw_byp_sel;
#[doc = "SW_CMP_P_SEL (rw) register accessor: CSDCMP Pos Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_cmp_p_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_cmp_p_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cmp_p_sel`]
module"]
#[doc(alias = "SW_CMP_P_SEL")]
pub type SwCmpPSel = crate::Reg<sw_cmp_p_sel::SwCmpPSelSpec>;
#[doc = "CSDCMP Pos Switch Waveform selection"]
pub mod sw_cmp_p_sel;
#[doc = "SW_CMP_N_SEL (rw) register accessor: CSDCMP Neg Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_cmp_n_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_cmp_n_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_cmp_n_sel`]
module"]
#[doc(alias = "SW_CMP_N_SEL")]
pub type SwCmpNSel = crate::Reg<sw_cmp_n_sel::SwCmpNSelSpec>;
#[doc = "CSDCMP Neg Switch Waveform selection"]
pub mod sw_cmp_n_sel;
#[doc = "SW_REFGEN_SEL (rw) register accessor: Reference Generator Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_refgen_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_refgen_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_refgen_sel`]
module"]
#[doc(alias = "SW_REFGEN_SEL")]
pub type SwRefgenSel = crate::Reg<sw_refgen_sel::SwRefgenSelSpec>;
#[doc = "Reference Generator Switch Waveform selection"]
pub mod sw_refgen_sel;
#[doc = "SW_FW_MOD_SEL (rw) register accessor: Full Wave Cmod Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_fw_mod_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_fw_mod_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_fw_mod_sel`]
module"]
#[doc(alias = "SW_FW_MOD_SEL")]
pub type SwFwModSel = crate::Reg<sw_fw_mod_sel::SwFwModSelSpec>;
#[doc = "Full Wave Cmod Switch Waveform selection"]
pub mod sw_fw_mod_sel;
#[doc = "SW_FW_TANK_SEL (rw) register accessor: Full Wave Csh_tank Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_fw_tank_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_fw_tank_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_fw_tank_sel`]
module"]
#[doc(alias = "SW_FW_TANK_SEL")]
pub type SwFwTankSel = crate::Reg<sw_fw_tank_sel::SwFwTankSelSpec>;
#[doc = "Full Wave Csh_tank Switch Waveform selection"]
pub mod sw_fw_tank_sel;
#[doc = "SW_DSI_SEL (rw) register accessor: DSI output switch control Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_dsi_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_dsi_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_dsi_sel`]
module"]
#[doc(alias = "SW_DSI_SEL")]
pub type SwDsiSel = crate::Reg<sw_dsi_sel::SwDsiSelSpec>;
#[doc = "DSI output switch control Waveform selection"]
pub mod sw_dsi_sel;
#[doc = "IO_SEL (rw) register accessor: IO output control Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`io_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_sel`]
module"]
#[doc(alias = "IO_SEL")]
pub type IoSel = crate::Reg<io_sel::IoSelSpec>;
#[doc = "IO output control Waveform selection"]
pub mod io_sel;
#[doc = "SEQ_TIME (rw) register accessor: Sequencer Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_time`]
module"]
#[doc(alias = "SEQ_TIME")]
pub type SeqTime = crate::Reg<seq_time::SeqTimeSpec>;
#[doc = "Sequencer Timing"]
pub mod seq_time;
#[doc = "SEQ_INIT_CNT (rw) register accessor: Sequencer Initial conversion and sample counts\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_init_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_init_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_init_cnt`]
module"]
#[doc(alias = "SEQ_INIT_CNT")]
pub type SeqInitCnt = crate::Reg<seq_init_cnt::SeqInitCntSpec>;
#[doc = "Sequencer Initial conversion and sample counts"]
pub mod seq_init_cnt;
#[doc = "SEQ_NORM_CNT (rw) register accessor: Sequencer Normal conversion and sample counts\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_norm_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_norm_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_norm_cnt`]
module"]
#[doc(alias = "SEQ_NORM_CNT")]
pub type SeqNormCnt = crate::Reg<seq_norm_cnt::SeqNormCntSpec>;
#[doc = "Sequencer Normal conversion and sample counts"]
pub mod seq_norm_cnt;
#[doc = "ADC_CTL (rw) register accessor: ADC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc_ctl`]
module"]
#[doc(alias = "ADC_CTL")]
pub type AdcCtl = crate::Reg<adc_ctl::AdcCtlSpec>;
#[doc = "ADC Control"]
pub mod adc_ctl;
#[doc = "SEQ_START (rw) register accessor: Sequencer start\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_start`]
module"]
#[doc(alias = "SEQ_START")]
pub type SeqStart = crate::Reg<seq_start::SeqStartSpec>;
#[doc = "Sequencer start"]
pub mod seq_start;
#[doc = "IDACA (rw) register accessor: IDACA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idaca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idaca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idaca`]
module"]
#[doc(alias = "IDACA")]
pub type Idaca = crate::Reg<idaca::IdacaSpec>;
#[doc = "IDACA Configuration"]
pub mod idaca;
#[doc = "IDACB (rw) register accessor: IDACB Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idacb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idacb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idacb`]
module"]
#[doc(alias = "IDACB")]
pub type Idacb = crate::Reg<idacb::IdacbSpec>;
#[doc = "IDACB Configuration"]
pub mod idacb;
