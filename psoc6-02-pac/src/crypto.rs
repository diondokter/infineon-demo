#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    _reserved1: [u8; 0x04],
    ram_pwr_ctl: RamPwrCtl,
    ram_pwr_delay_ctl: RamPwrDelayCtl,
    ecc_ctl: EccCtl,
    _reserved4: [u8; 0x0c],
    error_status0: ErrorStatus0,
    error_status1: ErrorStatus1,
    _reserved6: [u8; 0xd8],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    _reserved10: [u8; 0xf0],
    pr_lfsr_ctl0: PrLfsrCtl0,
    pr_lfsr_ctl1: PrLfsrCtl1,
    pr_lfsr_ctl2: PrLfsrCtl2,
    pr_max_ctl: PrMaxCtl,
    pr_cmd: PrCmd,
    _reserved15: [u8; 0x04],
    pr_result: PrResult,
    _reserved16: [u8; 0x64],
    tr_ctl0: TrCtl0,
    tr_ctl1: TrCtl1,
    tr_ctl2: TrCtl2,
    tr_status: TrStatus,
    tr_cmd: TrCmd,
    _reserved21: [u8; 0x04],
    tr_result: TrResult,
    _reserved22: [u8; 0x04],
    tr_garo_ctl: TrGaroCtl,
    tr_firo_ctl: TrFiroCtl,
    _reserved24: [u8; 0x18],
    tr_mon_ctl: TrMonCtl,
    _reserved25: [u8; 0x04],
    tr_mon_cmd: TrMonCmd,
    _reserved26: [u8; 0x04],
    tr_mon_rc_ctl: TrMonRcCtl,
    _reserved27: [u8; 0x04],
    tr_mon_rc_status0: TrMonRcStatus0,
    tr_mon_rc_status1: TrMonRcStatus1,
    tr_mon_ap_ctl: TrMonApCtl,
    _reserved30: [u8; 0x04],
    tr_mon_ap_status0: TrMonApStatus0,
    tr_mon_ap_status1: TrMonApStatus1,
    _reserved32: [u8; 0x0d14],
    status: Status,
    _reserved33: [u8; 0x38],
    instr_ff_ctl: InstrFfCtl,
    instr_ff_status: InstrFfStatus,
    instr_ff_wr: InstrFfWr,
    _reserved36: [u8; 0x74],
    load0_ff_status: Load0FfStatus,
    _reserved37: [u8; 0x0c],
    load1_ff_status: Load1FfStatus,
    _reserved38: [u8; 0x1c],
    store_ff_status: StoreFfStatus,
    _reserved39: [u8; 0x0c],
    aes_ctl: AesCtl,
    _reserved40: [u8; 0x7c],
    result: Result,
    _reserved41: [u8; 0x027c],
    crc_ctl: CrcCtl,
    _reserved42: [u8; 0x0c],
    crc_data_ctl: CrcDataCtl,
    _reserved43: [u8; 0x0c],
    crc_pol_ctl: CrcPolCtl,
    _reserved44: [u8; 0x1c],
    crc_rem_ctl: CrcRemCtl,
    _reserved45: [u8; 0x04],
    crc_rem_result: CrcRemResult,
    _reserved46: [u8; 0x34],
    vu_ctl0: VuCtl0,
    vu_ctl1: VuCtl1,
    vu_ctl2: VuCtl2,
    _reserved49: [u8; 0x04],
    vu_status: VuStatus,
    _reserved50: [u8; 0x2c],
    vu_rf_data: [VuRfData; 16],
    _reserved51: [u8; 0x0b00],
    dev_key_addr0_ctl: DevKeyAddr0Ctl,
    dev_key_addr0: DevKeyAddr0,
    _reserved53: [u8; 0x18],
    dev_key_addr1_ctl: DevKeyAddr1Ctl,
    dev_key_addr1: DevKeyAddr1,
    _reserved55: [u8; 0x58],
    dev_key_status: DevKeyStatus,
    _reserved56: [u8; 0x7c],
    dev_key_ctl0: DevKeyCtl0,
    _reserved57: [u8; 0x1c],
    dev_key_ctl1: DevKeyCtl1,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - SRAM power control"]
    #[inline(always)]
    pub const fn ram_pwr_ctl(&self) -> &RamPwrCtl {
        &self.ram_pwr_ctl
    }
    #[doc = "0x0c - SRAM power delay control"]
    #[inline(always)]
    pub const fn ram_pwr_delay_ctl(&self) -> &RamPwrDelayCtl {
        &self.ram_pwr_delay_ctl
    }
    #[doc = "0x10 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &EccCtl {
        &self.ecc_ctl
    }
    #[doc = "0x20 - Error status 0"]
    #[inline(always)]
    pub const fn error_status0(&self) -> &ErrorStatus0 {
        &self.error_status0
    }
    #[doc = "0x24 - Error status 1"]
    #[inline(always)]
    pub const fn error_status1(&self) -> &ErrorStatus1 {
        &self.error_status1
    }
    #[doc = "0x100 - Interrupt register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x104 - Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x108 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x10c - Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x200 - Pseudo random LFSR control 0"]
    #[inline(always)]
    pub const fn pr_lfsr_ctl0(&self) -> &PrLfsrCtl0 {
        &self.pr_lfsr_ctl0
    }
    #[doc = "0x204 - Pseudo random LFSR control 1"]
    #[inline(always)]
    pub const fn pr_lfsr_ctl1(&self) -> &PrLfsrCtl1 {
        &self.pr_lfsr_ctl1
    }
    #[doc = "0x208 - Pseudo random LFSR control 2"]
    #[inline(always)]
    pub const fn pr_lfsr_ctl2(&self) -> &PrLfsrCtl2 {
        &self.pr_lfsr_ctl2
    }
    #[doc = "0x20c - Pseudo random maximum control"]
    #[inline(always)]
    pub const fn pr_max_ctl(&self) -> &PrMaxCtl {
        &self.pr_max_ctl
    }
    #[doc = "0x210 - Pseudo random command"]
    #[inline(always)]
    pub const fn pr_cmd(&self) -> &PrCmd {
        &self.pr_cmd
    }
    #[doc = "0x218 - Pseudo random result"]
    #[inline(always)]
    pub const fn pr_result(&self) -> &PrResult {
        &self.pr_result
    }
    #[doc = "0x280 - True random control 0"]
    #[inline(always)]
    pub const fn tr_ctl0(&self) -> &TrCtl0 {
        &self.tr_ctl0
    }
    #[doc = "0x284 - True random control 1"]
    #[inline(always)]
    pub const fn tr_ctl1(&self) -> &TrCtl1 {
        &self.tr_ctl1
    }
    #[doc = "0x288 - True random control 2"]
    #[inline(always)]
    pub const fn tr_ctl2(&self) -> &TrCtl2 {
        &self.tr_ctl2
    }
    #[doc = "0x28c - True random status"]
    #[inline(always)]
    pub const fn tr_status(&self) -> &TrStatus {
        &self.tr_status
    }
    #[doc = "0x290 - True random command"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TrCmd {
        &self.tr_cmd
    }
    #[doc = "0x298 - True random result"]
    #[inline(always)]
    pub const fn tr_result(&self) -> &TrResult {
        &self.tr_result
    }
    #[doc = "0x2a0 - True random GARO control"]
    #[inline(always)]
    pub const fn tr_garo_ctl(&self) -> &TrGaroCtl {
        &self.tr_garo_ctl
    }
    #[doc = "0x2a4 - True random FIRO control"]
    #[inline(always)]
    pub const fn tr_firo_ctl(&self) -> &TrFiroCtl {
        &self.tr_firo_ctl
    }
    #[doc = "0x2c0 - True random monitor control"]
    #[inline(always)]
    pub const fn tr_mon_ctl(&self) -> &TrMonCtl {
        &self.tr_mon_ctl
    }
    #[doc = "0x2c8 - True random monitor command"]
    #[inline(always)]
    pub const fn tr_mon_cmd(&self) -> &TrMonCmd {
        &self.tr_mon_cmd
    }
    #[doc = "0x2d0 - True random monitor RC control"]
    #[inline(always)]
    pub const fn tr_mon_rc_ctl(&self) -> &TrMonRcCtl {
        &self.tr_mon_rc_ctl
    }
    #[doc = "0x2d8 - True random monitor RC status 0"]
    #[inline(always)]
    pub const fn tr_mon_rc_status0(&self) -> &TrMonRcStatus0 {
        &self.tr_mon_rc_status0
    }
    #[doc = "0x2dc - True random monitor RC status 1"]
    #[inline(always)]
    pub const fn tr_mon_rc_status1(&self) -> &TrMonRcStatus1 {
        &self.tr_mon_rc_status1
    }
    #[doc = "0x2e0 - True random monitor AP control"]
    #[inline(always)]
    pub const fn tr_mon_ap_ctl(&self) -> &TrMonApCtl {
        &self.tr_mon_ap_ctl
    }
    #[doc = "0x2e8 - True random monitor AP status 0"]
    #[inline(always)]
    pub const fn tr_mon_ap_status0(&self) -> &TrMonApStatus0 {
        &self.tr_mon_ap_status0
    }
    #[doc = "0x2ec - True random monitor AP status 1"]
    #[inline(always)]
    pub const fn tr_mon_ap_status1(&self) -> &TrMonApStatus1 {
        &self.tr_mon_ap_status1
    }
    #[doc = "0x1004 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1040 - Instruction FIFO control"]
    #[inline(always)]
    pub const fn instr_ff_ctl(&self) -> &InstrFfCtl {
        &self.instr_ff_ctl
    }
    #[doc = "0x1044 - Instruction FIFO status"]
    #[inline(always)]
    pub const fn instr_ff_status(&self) -> &InstrFfStatus {
        &self.instr_ff_status
    }
    #[doc = "0x1048 - Instruction FIFO write"]
    #[inline(always)]
    pub const fn instr_ff_wr(&self) -> &InstrFfWr {
        &self.instr_ff_wr
    }
    #[doc = "0x10c0 - Load 0 FIFO status"]
    #[inline(always)]
    pub const fn load0_ff_status(&self) -> &Load0FfStatus {
        &self.load0_ff_status
    }
    #[doc = "0x10d0 - Load 1 FIFO status"]
    #[inline(always)]
    pub const fn load1_ff_status(&self) -> &Load1FfStatus {
        &self.load1_ff_status
    }
    #[doc = "0x10f0 - Store FIFO status"]
    #[inline(always)]
    pub const fn store_ff_status(&self) -> &StoreFfStatus {
        &self.store_ff_status
    }
    #[doc = "0x1100 - AES control"]
    #[inline(always)]
    pub const fn aes_ctl(&self) -> &AesCtl {
        &self.aes_ctl
    }
    #[doc = "0x1180 - Result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x1400 - CRC control"]
    #[inline(always)]
    pub const fn crc_ctl(&self) -> &CrcCtl {
        &self.crc_ctl
    }
    #[doc = "0x1410 - CRC data control"]
    #[inline(always)]
    pub const fn crc_data_ctl(&self) -> &CrcDataCtl {
        &self.crc_data_ctl
    }
    #[doc = "0x1420 - CRC polynomial control"]
    #[inline(always)]
    pub const fn crc_pol_ctl(&self) -> &CrcPolCtl {
        &self.crc_pol_ctl
    }
    #[doc = "0x1440 - CRC remainder control"]
    #[inline(always)]
    pub const fn crc_rem_ctl(&self) -> &CrcRemCtl {
        &self.crc_rem_ctl
    }
    #[doc = "0x1448 - CRC remainder result"]
    #[inline(always)]
    pub const fn crc_rem_result(&self) -> &CrcRemResult {
        &self.crc_rem_result
    }
    #[doc = "0x1480 - Vector unit control 0"]
    #[inline(always)]
    pub const fn vu_ctl0(&self) -> &VuCtl0 {
        &self.vu_ctl0
    }
    #[doc = "0x1484 - Vector unit control 1"]
    #[inline(always)]
    pub const fn vu_ctl1(&self) -> &VuCtl1 {
        &self.vu_ctl1
    }
    #[doc = "0x1488 - Vector unit control 2"]
    #[inline(always)]
    pub const fn vu_ctl2(&self) -> &VuCtl2 {
        &self.vu_ctl2
    }
    #[doc = "0x1490 - Vector unit status"]
    #[inline(always)]
    pub const fn vu_status(&self) -> &VuStatus {
        &self.vu_status
    }
    #[doc = "0x14c0..0x1500 - Vector unit register-file"]
    #[inline(always)]
    pub const fn vu_rf_data(&self, n: usize) -> &VuRfData {
        &self.vu_rf_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14c0..0x1500 - Vector unit register-file"]
    #[inline(always)]
    pub fn vu_rf_data_iter(&self) -> impl Iterator<Item = &VuRfData> {
        self.vu_rf_data.iter()
    }
    #[doc = "0x2000 - Device key address 0 control"]
    #[inline(always)]
    pub const fn dev_key_addr0_ctl(&self) -> &DevKeyAddr0Ctl {
        &self.dev_key_addr0_ctl
    }
    #[doc = "0x2004 - Device key address 0"]
    #[inline(always)]
    pub const fn dev_key_addr0(&self) -> &DevKeyAddr0 {
        &self.dev_key_addr0
    }
    #[doc = "0x2020 - Device key address 1 control"]
    #[inline(always)]
    pub const fn dev_key_addr1_ctl(&self) -> &DevKeyAddr1Ctl {
        &self.dev_key_addr1_ctl
    }
    #[doc = "0x2024 - Device key address 1 control"]
    #[inline(always)]
    pub const fn dev_key_addr1(&self) -> &DevKeyAddr1 {
        &self.dev_key_addr1
    }
    #[doc = "0x2080 - Device key status"]
    #[inline(always)]
    pub const fn dev_key_status(&self) -> &DevKeyStatus {
        &self.dev_key_status
    }
    #[doc = "0x2100 - Device key control 0"]
    #[inline(always)]
    pub const fn dev_key_ctl0(&self) -> &DevKeyCtl0 {
        &self.dev_key_ctl0
    }
    #[doc = "0x2120 - Device key control 1"]
    #[inline(always)]
    pub const fn dev_key_ctl1(&self) -> &DevKeyCtl1 {
        &self.dev_key_ctl1
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "RAM_PWR_CTL (rw) register accessor: SRAM power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pwr_ctl`]
module"]
#[doc(alias = "RAM_PWR_CTL")]
pub type RamPwrCtl = crate::Reg<ram_pwr_ctl::RamPwrCtlSpec>;
#[doc = "SRAM power control"]
pub mod ram_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: SRAM power delay control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pwr_delay_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pwr_delay_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pwr_delay_ctl`]
module"]
#[doc(alias = "RAM_PWR_DELAY_CTL")]
pub type RamPwrDelayCtl = crate::Reg<ram_pwr_delay_ctl::RamPwrDelayCtlSpec>;
#[doc = "SRAM power delay control"]
pub mod ram_pwr_delay_ctl;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
#[doc(alias = "ECC_CTL")]
pub type EccCtl = crate::Reg<ecc_ctl::EccCtlSpec>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "ERROR_STATUS0 (r) register accessor: Error status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status0`]
module"]
#[doc(alias = "ERROR_STATUS0")]
pub type ErrorStatus0 = crate::Reg<error_status0::ErrorStatus0Spec>;
#[doc = "Error status 0"]
pub mod error_status0;
#[doc = "ERROR_STATUS1 (rw) register accessor: Error status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`error_status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_status1`]
module"]
#[doc(alias = "ERROR_STATUS1")]
pub type ErrorStatus1 = crate::Reg<error_status1::ErrorStatus1Spec>;
#[doc = "Error status 1"]
pub mod error_status1;
#[doc = "INTR (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
#[doc = "PR_LFSR_CTL0 (rw) register accessor: Pseudo random LFSR control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_lfsr_ctl0`]
module"]
#[doc(alias = "PR_LFSR_CTL0")]
pub type PrLfsrCtl0 = crate::Reg<pr_lfsr_ctl0::PrLfsrCtl0Spec>;
#[doc = "Pseudo random LFSR control 0"]
pub mod pr_lfsr_ctl0;
#[doc = "PR_LFSR_CTL1 (rw) register accessor: Pseudo random LFSR control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_lfsr_ctl1`]
module"]
#[doc(alias = "PR_LFSR_CTL1")]
pub type PrLfsrCtl1 = crate::Reg<pr_lfsr_ctl1::PrLfsrCtl1Spec>;
#[doc = "Pseudo random LFSR control 1"]
pub mod pr_lfsr_ctl1;
#[doc = "PR_LFSR_CTL2 (rw) register accessor: Pseudo random LFSR control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_lfsr_ctl2`]
module"]
#[doc(alias = "PR_LFSR_CTL2")]
pub type PrLfsrCtl2 = crate::Reg<pr_lfsr_ctl2::PrLfsrCtl2Spec>;
#[doc = "Pseudo random LFSR control 2"]
pub mod pr_lfsr_ctl2;
#[doc = "PR_MAX_CTL (rw) register accessor: Pseudo random maximum control\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_max_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_max_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_max_ctl`]
module"]
#[doc(alias = "PR_MAX_CTL")]
pub type PrMaxCtl = crate::Reg<pr_max_ctl::PrMaxCtlSpec>;
#[doc = "Pseudo random maximum control"]
pub mod pr_max_ctl;
#[doc = "PR_CMD (rw) register accessor: Pseudo random command\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_cmd`]
module"]
#[doc(alias = "PR_CMD")]
pub type PrCmd = crate::Reg<pr_cmd::PrCmdSpec>;
#[doc = "Pseudo random command"]
pub mod pr_cmd;
#[doc = "PR_RESULT (rw) register accessor: Pseudo random result\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr_result`]
module"]
#[doc(alias = "PR_RESULT")]
pub type PrResult = crate::Reg<pr_result::PrResultSpec>;
#[doc = "Pseudo random result"]
pub mod pr_result;
#[doc = "TR_CTL0 (rw) register accessor: True random control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl0`]
module"]
#[doc(alias = "TR_CTL0")]
pub type TrCtl0 = crate::Reg<tr_ctl0::TrCtl0Spec>;
#[doc = "True random control 0"]
pub mod tr_ctl0;
#[doc = "TR_CTL1 (rw) register accessor: True random control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl1`]
module"]
#[doc(alias = "TR_CTL1")]
pub type TrCtl1 = crate::Reg<tr_ctl1::TrCtl1Spec>;
#[doc = "True random control 1"]
pub mod tr_ctl1;
#[doc = "TR_CTL2 (rw) register accessor: True random control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl2`]
module"]
#[doc(alias = "TR_CTL2")]
pub type TrCtl2 = crate::Reg<tr_ctl2::TrCtl2Spec>;
#[doc = "True random control 2"]
pub mod tr_ctl2;
#[doc = "TR_STATUS (r) register accessor: True random status\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_status`]
module"]
#[doc(alias = "TR_STATUS")]
pub type TrStatus = crate::Reg<tr_status::TrStatusSpec>;
#[doc = "True random status"]
pub mod tr_status;
#[doc = "TR_CMD (rw) register accessor: True random command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
#[doc(alias = "TR_CMD")]
pub type TrCmd = crate::Reg<tr_cmd::TrCmdSpec>;
#[doc = "True random command"]
pub mod tr_cmd;
#[doc = "TR_RESULT (rw) register accessor: True random result\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_result`]
module"]
#[doc(alias = "TR_RESULT")]
pub type TrResult = crate::Reg<tr_result::TrResultSpec>;
#[doc = "True random result"]
pub mod tr_result;
#[doc = "TR_GARO_CTL (rw) register accessor: True random GARO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_garo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_garo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_garo_ctl`]
module"]
#[doc(alias = "TR_GARO_CTL")]
pub type TrGaroCtl = crate::Reg<tr_garo_ctl::TrGaroCtlSpec>;
#[doc = "True random GARO control"]
pub mod tr_garo_ctl;
#[doc = "TR_FIRO_CTL (rw) register accessor: True random FIRO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_firo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_firo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_firo_ctl`]
module"]
#[doc(alias = "TR_FIRO_CTL")]
pub type TrFiroCtl = crate::Reg<tr_firo_ctl::TrFiroCtlSpec>;
#[doc = "True random FIRO control"]
pub mod tr_firo_ctl;
#[doc = "TR_MON_CTL (rw) register accessor: True random monitor control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_ctl`]
module"]
#[doc(alias = "TR_MON_CTL")]
pub type TrMonCtl = crate::Reg<tr_mon_ctl::TrMonCtlSpec>;
#[doc = "True random monitor control"]
pub mod tr_mon_ctl;
#[doc = "TR_MON_CMD (rw) register accessor: True random monitor command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_cmd`]
module"]
#[doc(alias = "TR_MON_CMD")]
pub type TrMonCmd = crate::Reg<tr_mon_cmd::TrMonCmdSpec>;
#[doc = "True random monitor command"]
pub mod tr_mon_cmd;
#[doc = "TR_MON_RC_CTL (rw) register accessor: True random monitor RC control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_rc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_rc_ctl`]
module"]
#[doc(alias = "TR_MON_RC_CTL")]
pub type TrMonRcCtl = crate::Reg<tr_mon_rc_ctl::TrMonRcCtlSpec>;
#[doc = "True random monitor RC control"]
pub mod tr_mon_rc_ctl;
#[doc = "TR_MON_RC_STATUS0 (r) register accessor: True random monitor RC status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_rc_status0`]
module"]
#[doc(alias = "TR_MON_RC_STATUS0")]
pub type TrMonRcStatus0 = crate::Reg<tr_mon_rc_status0::TrMonRcStatus0Spec>;
#[doc = "True random monitor RC status 0"]
pub mod tr_mon_rc_status0;
#[doc = "TR_MON_RC_STATUS1 (r) register accessor: True random monitor RC status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_rc_status1`]
module"]
#[doc(alias = "TR_MON_RC_STATUS1")]
pub type TrMonRcStatus1 = crate::Reg<tr_mon_rc_status1::TrMonRcStatus1Spec>;
#[doc = "True random monitor RC status 1"]
pub mod tr_mon_rc_status1;
#[doc = "TR_MON_AP_CTL (rw) register accessor: True random monitor AP control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_ap_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_ap_ctl`]
module"]
#[doc(alias = "TR_MON_AP_CTL")]
pub type TrMonApCtl = crate::Reg<tr_mon_ap_ctl::TrMonApCtlSpec>;
#[doc = "True random monitor AP control"]
pub mod tr_mon_ap_ctl;
#[doc = "TR_MON_AP_STATUS0 (r) register accessor: True random monitor AP status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_ap_status0`]
module"]
#[doc(alias = "TR_MON_AP_STATUS0")]
pub type TrMonApStatus0 = crate::Reg<tr_mon_ap_status0::TrMonApStatus0Spec>;
#[doc = "True random monitor AP status 0"]
pub mod tr_mon_ap_status0;
#[doc = "TR_MON_AP_STATUS1 (r) register accessor: True random monitor AP status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_mon_ap_status1`]
module"]
#[doc(alias = "TR_MON_AP_STATUS1")]
pub type TrMonApStatus1 = crate::Reg<tr_mon_ap_status1::TrMonApStatus1Spec>;
#[doc = "True random monitor AP status 1"]
pub mod tr_mon_ap_status1;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "INSTR_FF_CTL (rw) register accessor: Instruction FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`instr_ff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr_ff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_ff_ctl`]
module"]
#[doc(alias = "INSTR_FF_CTL")]
pub type InstrFfCtl = crate::Reg<instr_ff_ctl::InstrFfCtlSpec>;
#[doc = "Instruction FIFO control"]
pub mod instr_ff_ctl;
#[doc = "INSTR_FF_STATUS (r) register accessor: Instruction FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`instr_ff_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_ff_status`]
module"]
#[doc(alias = "INSTR_FF_STATUS")]
pub type InstrFfStatus = crate::Reg<instr_ff_status::InstrFfStatusSpec>;
#[doc = "Instruction FIFO status"]
pub mod instr_ff_status;
#[doc = "INSTR_FF_WR (w) register accessor: Instruction FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr_ff_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@instr_ff_wr`]
module"]
#[doc(alias = "INSTR_FF_WR")]
pub type InstrFfWr = crate::Reg<instr_ff_wr::InstrFfWrSpec>;
#[doc = "Instruction FIFO write"]
pub mod instr_ff_wr;
#[doc = "LOAD0_FF_STATUS (r) register accessor: Load 0 FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`load0_ff_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load0_ff_status`]
module"]
#[doc(alias = "LOAD0_FF_STATUS")]
pub type Load0FfStatus = crate::Reg<load0_ff_status::Load0FfStatusSpec>;
#[doc = "Load 0 FIFO status"]
pub mod load0_ff_status;
#[doc = "LOAD1_FF_STATUS (r) register accessor: Load 1 FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`load1_ff_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@load1_ff_status`]
module"]
#[doc(alias = "LOAD1_FF_STATUS")]
pub type Load1FfStatus = crate::Reg<load1_ff_status::Load1FfStatusSpec>;
#[doc = "Load 1 FIFO status"]
pub mod load1_ff_status;
#[doc = "STORE_FF_STATUS (r) register accessor: Store FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`store_ff_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@store_ff_status`]
module"]
#[doc(alias = "STORE_FF_STATUS")]
pub type StoreFfStatus = crate::Reg<store_ff_status::StoreFfStatusSpec>;
#[doc = "Store FIFO status"]
pub mod store_ff_status;
#[doc = "AES_CTL (rw) register accessor: AES control\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aes_ctl`]
module"]
#[doc(alias = "AES_CTL")]
pub type AesCtl = crate::Reg<aes_ctl::AesCtlSpec>;
#[doc = "AES control"]
pub mod aes_ctl;
#[doc = "RESULT (rw) register accessor: Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`result::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result"]
pub mod result;
#[doc = "CRC_CTL (rw) register accessor: CRC control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctl`]
module"]
#[doc(alias = "CRC_CTL")]
pub type CrcCtl = crate::Reg<crc_ctl::CrcCtlSpec>;
#[doc = "CRC control"]
pub mod crc_ctl;
#[doc = "CRC_DATA_CTL (rw) register accessor: CRC data control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_data_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_data_ctl`]
module"]
#[doc(alias = "CRC_DATA_CTL")]
pub type CrcDataCtl = crate::Reg<crc_data_ctl::CrcDataCtlSpec>;
#[doc = "CRC data control"]
pub mod crc_data_ctl;
#[doc = "CRC_POL_CTL (rw) register accessor: CRC polynomial control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pol_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_pol_ctl`]
module"]
#[doc(alias = "CRC_POL_CTL")]
pub type CrcPolCtl = crate::Reg<crc_pol_ctl::CrcPolCtlSpec>;
#[doc = "CRC polynomial control"]
pub mod crc_pol_ctl;
#[doc = "CRC_REM_CTL (rw) register accessor: CRC remainder control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_rem_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_rem_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_rem_ctl`]
module"]
#[doc(alias = "CRC_REM_CTL")]
pub type CrcRemCtl = crate::Reg<crc_rem_ctl::CrcRemCtlSpec>;
#[doc = "CRC remainder control"]
pub mod crc_rem_ctl;
#[doc = "CRC_REM_RESULT (r) register accessor: CRC remainder result\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_rem_result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_rem_result`]
module"]
#[doc(alias = "CRC_REM_RESULT")]
pub type CrcRemResult = crate::Reg<crc_rem_result::CrcRemResultSpec>;
#[doc = "CRC remainder result"]
pub mod crc_rem_result;
#[doc = "VU_CTL0 (rw) register accessor: Vector unit control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vu_ctl0`]
module"]
#[doc(alias = "VU_CTL0")]
pub type VuCtl0 = crate::Reg<vu_ctl0::VuCtl0Spec>;
#[doc = "Vector unit control 0"]
pub mod vu_ctl0;
#[doc = "VU_CTL1 (rw) register accessor: Vector unit control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vu_ctl1`]
module"]
#[doc(alias = "VU_CTL1")]
pub type VuCtl1 = crate::Reg<vu_ctl1::VuCtl1Spec>;
#[doc = "Vector unit control 1"]
pub mod vu_ctl1;
#[doc = "VU_CTL2 (rw) register accessor: Vector unit control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vu_ctl2`]
module"]
#[doc(alias = "VU_CTL2")]
pub type VuCtl2 = crate::Reg<vu_ctl2::VuCtl2Spec>;
#[doc = "Vector unit control 2"]
pub mod vu_ctl2;
#[doc = "VU_STATUS (r) register accessor: Vector unit status\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vu_status`]
module"]
#[doc(alias = "VU_STATUS")]
pub type VuStatus = crate::Reg<vu_status::VuStatusSpec>;
#[doc = "Vector unit status"]
pub mod vu_status;
#[doc = "VU_RF_DATA (r) register accessor: Vector unit register-file\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_rf_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vu_rf_data`]
module"]
#[doc(alias = "VU_RF_DATA")]
pub type VuRfData = crate::Reg<vu_rf_data::VuRfDataSpec>;
#[doc = "Vector unit register-file"]
pub mod vu_rf_data;
#[doc = "DEV_KEY_ADDR0_CTL (rw) register accessor: Device key address 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_addr0_ctl`]
module"]
#[doc(alias = "DEV_KEY_ADDR0_CTL")]
pub type DevKeyAddr0Ctl = crate::Reg<dev_key_addr0_ctl::DevKeyAddr0CtlSpec>;
#[doc = "Device key address 0 control"]
pub mod dev_key_addr0_ctl;
#[doc = "DEV_KEY_ADDR0 (rw) register accessor: Device key address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_addr0`]
module"]
#[doc(alias = "DEV_KEY_ADDR0")]
pub type DevKeyAddr0 = crate::Reg<dev_key_addr0::DevKeyAddr0Spec>;
#[doc = "Device key address 0"]
pub mod dev_key_addr0;
#[doc = "DEV_KEY_ADDR1_CTL (rw) register accessor: Device key address 1 control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_addr1_ctl`]
module"]
#[doc(alias = "DEV_KEY_ADDR1_CTL")]
pub type DevKeyAddr1Ctl = crate::Reg<dev_key_addr1_ctl::DevKeyAddr1CtlSpec>;
#[doc = "Device key address 1 control"]
pub mod dev_key_addr1_ctl;
#[doc = "DEV_KEY_ADDR1 (rw) register accessor: Device key address 1 control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_addr1`]
module"]
#[doc(alias = "DEV_KEY_ADDR1")]
pub type DevKeyAddr1 = crate::Reg<dev_key_addr1::DevKeyAddr1Spec>;
#[doc = "Device key address 1 control"]
pub mod dev_key_addr1;
#[doc = "DEV_KEY_STATUS (r) register accessor: Device key status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_status`]
module"]
#[doc(alias = "DEV_KEY_STATUS")]
pub type DevKeyStatus = crate::Reg<dev_key_status::DevKeyStatusSpec>;
#[doc = "Device key status"]
pub mod dev_key_status;
#[doc = "DEV_KEY_CTL0 (rw) register accessor: Device key control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_ctl0`]
module"]
#[doc(alias = "DEV_KEY_CTL0")]
pub type DevKeyCtl0 = crate::Reg<dev_key_ctl0::DevKeyCtl0Spec>;
#[doc = "Device key control 0"]
pub mod dev_key_ctl0;
#[doc = "DEV_KEY_CTL1 (rw) register accessor: Device key control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dev_key_ctl1`]
module"]
#[doc(alias = "DEV_KEY_CTL1")]
pub type DevKeyCtl1 = crate::Reg<dev_key_ctl1::DevKeyCtl1Spec>;
#[doc = "Device key control 1"]
pub mod dev_key_ctl1;
