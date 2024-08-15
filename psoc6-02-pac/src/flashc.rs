#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flash_ctl: FlashCtl,
    flash_pwr_ctl: FlashPwrCtl,
    flash_cmd: FlashCmd,
    _reserved3: [u8; 0x0294],
    ecc_ctl: EccCtl,
    _reserved4: [u8; 0x0c],
    fm_sram_ecc_ctl0: FmSramEccCtl0,
    fm_sram_ecc_ctl1: FmSramEccCtl1,
    fm_sram_ecc_ctl2: FmSramEccCtl2,
    fm_sram_ecc_ctl3: FmSramEccCtl3,
    _reserved8: [u8; 0x0140],
    cm0_ca_ctl0: Cm0CaCtl0,
    cm0_ca_ctl1: Cm0CaCtl1,
    cm0_ca_ctl2: Cm0CaCtl2,
    _reserved11: [u8; 0x34],
    cm0_ca_status0: Cm0CaStatus0,
    cm0_ca_status1: Cm0CaStatus1,
    cm0_ca_status2: Cm0CaStatus2,
    _reserved14: [u8; 0x14],
    cm0_status: Cm0Status,
    _reserved15: [u8; 0x1c],
    cm4_ca_ctl0: Cm4CaCtl0,
    cm4_ca_ctl1: Cm4CaCtl1,
    cm4_ca_ctl2: Cm4CaCtl2,
    _reserved18: [u8; 0x34],
    cm4_ca_status0: Cm4CaStatus0,
    cm4_ca_status1: Cm4CaStatus1,
    cm4_ca_status2: Cm4CaStatus2,
    _reserved21: [u8; 0x14],
    cm4_status: Cm4Status,
    _reserved22: [u8; 0x1c],
    crypto_buff_ctl: CryptoBuffCtl,
    _reserved23: [u8; 0x7c],
    dw0_buff_ctl: Dw0BuffCtl,
    _reserved24: [u8; 0x7c],
    dw1_buff_ctl: Dw1BuffCtl,
    _reserved25: [u8; 0x7c],
    dmac_buff_ctl: DmacBuffCtl,
    _reserved26: [u8; 0x7c],
    ext_ms0_buff_ctl: ExtMs0BuffCtl,
    _reserved27: [u8; 0x7c],
    ext_ms1_buff_ctl: ExtMs1BuffCtl,
    _reserved28: [u8; 0xe87c],
    fm_ctl: FmCtl,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn flash_ctl(&self) -> &FlashCtl {
        &self.flash_ctl
    }
    #[doc = "0x04 - Flash power control"]
    #[inline(always)]
    pub const fn flash_pwr_ctl(&self) -> &FlashPwrCtl {
        &self.flash_pwr_ctl
    }
    #[doc = "0x08 - Command"]
    #[inline(always)]
    pub const fn flash_cmd(&self) -> &FlashCmd {
        &self.flash_cmd
    }
    #[doc = "0x2a0 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &EccCtl {
        &self.ecc_ctl
    }
    #[doc = "0x2b0 - eCT Flash SRAM ECC control 0"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl0(&self) -> &FmSramEccCtl0 {
        &self.fm_sram_ecc_ctl0
    }
    #[doc = "0x2b4 - eCT Flash SRAM ECC control 1"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl1(&self) -> &FmSramEccCtl1 {
        &self.fm_sram_ecc_ctl1
    }
    #[doc = "0x2b8 - eCT Flash SRAM ECC control 2"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl2(&self) -> &FmSramEccCtl2 {
        &self.fm_sram_ecc_ctl2
    }
    #[doc = "0x2bc - eCT Flash SRAM ECC control 3"]
    #[inline(always)]
    pub const fn fm_sram_ecc_ctl3(&self) -> &FmSramEccCtl3 {
        &self.fm_sram_ecc_ctl3
    }
    #[doc = "0x400 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl0(&self) -> &Cm0CaCtl0 {
        &self.cm0_ca_ctl0
    }
    #[doc = "0x404 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl1(&self) -> &Cm0CaCtl1 {
        &self.cm0_ca_ctl1
    }
    #[doc = "0x408 - CM0+ cache control"]
    #[inline(always)]
    pub const fn cm0_ca_ctl2(&self) -> &Cm0CaCtl2 {
        &self.cm0_ca_ctl2
    }
    #[doc = "0x440 - CM0+ cache status 0"]
    #[inline(always)]
    pub const fn cm0_ca_status0(&self) -> &Cm0CaStatus0 {
        &self.cm0_ca_status0
    }
    #[doc = "0x444 - CM0+ cache status 1"]
    #[inline(always)]
    pub const fn cm0_ca_status1(&self) -> &Cm0CaStatus1 {
        &self.cm0_ca_status1
    }
    #[doc = "0x448 - CM0+ cache status 2"]
    #[inline(always)]
    pub const fn cm0_ca_status2(&self) -> &Cm0CaStatus2 {
        &self.cm0_ca_status2
    }
    #[doc = "0x460 - CM0+ interface status"]
    #[inline(always)]
    pub const fn cm0_status(&self) -> &Cm0Status {
        &self.cm0_status
    }
    #[doc = "0x480 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl0(&self) -> &Cm4CaCtl0 {
        &self.cm4_ca_ctl0
    }
    #[doc = "0x484 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl1(&self) -> &Cm4CaCtl1 {
        &self.cm4_ca_ctl1
    }
    #[doc = "0x488 - CM4 cache control"]
    #[inline(always)]
    pub const fn cm4_ca_ctl2(&self) -> &Cm4CaCtl2 {
        &self.cm4_ca_ctl2
    }
    #[doc = "0x4c0 - CM4 cache status 0"]
    #[inline(always)]
    pub const fn cm4_ca_status0(&self) -> &Cm4CaStatus0 {
        &self.cm4_ca_status0
    }
    #[doc = "0x4c4 - CM4 cache status 1"]
    #[inline(always)]
    pub const fn cm4_ca_status1(&self) -> &Cm4CaStatus1 {
        &self.cm4_ca_status1
    }
    #[doc = "0x4c8 - CM4 cache status 2"]
    #[inline(always)]
    pub const fn cm4_ca_status2(&self) -> &Cm4CaStatus2 {
        &self.cm4_ca_status2
    }
    #[doc = "0x4e0 - CM4 interface status"]
    #[inline(always)]
    pub const fn cm4_status(&self) -> &Cm4Status {
        &self.cm4_status
    }
    #[doc = "0x500 - Cryptography buffer control"]
    #[inline(always)]
    pub const fn crypto_buff_ctl(&self) -> &CryptoBuffCtl {
        &self.crypto_buff_ctl
    }
    #[doc = "0x580 - Datawire 0 buffer control"]
    #[inline(always)]
    pub const fn dw0_buff_ctl(&self) -> &Dw0BuffCtl {
        &self.dw0_buff_ctl
    }
    #[doc = "0x600 - Datawire 1 buffer control"]
    #[inline(always)]
    pub const fn dw1_buff_ctl(&self) -> &Dw1BuffCtl {
        &self.dw1_buff_ctl
    }
    #[doc = "0x680 - DMA controller buffer control"]
    #[inline(always)]
    pub const fn dmac_buff_ctl(&self) -> &DmacBuffCtl {
        &self.dmac_buff_ctl
    }
    #[doc = "0x700 - External master 0 buffer control"]
    #[inline(always)]
    pub const fn ext_ms0_buff_ctl(&self) -> &ExtMs0BuffCtl {
        &self.ext_ms0_buff_ctl
    }
    #[doc = "0x780 - External master 1 buffer control"]
    #[inline(always)]
    pub const fn ext_ms1_buff_ctl(&self) -> &ExtMs1BuffCtl {
        &self.ext_ms1_buff_ctl
    }
    #[doc = "0xf000..0x10000 - Flash Macro Registers"]
    #[inline(always)]
    pub const fn fm_ctl(&self) -> &FmCtl {
        &self.fm_ctl
    }
}
#[doc = "FLASH_CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_ctl`]
module"]
#[doc(alias = "FLASH_CTL")]
pub type FlashCtl = crate::Reg<flash_ctl::FlashCtlSpec>;
#[doc = "Control"]
pub mod flash_ctl;
#[doc = "FLASH_PWR_CTL (rw) register accessor: Flash power control\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_pwr_ctl`]
module"]
#[doc(alias = "FLASH_PWR_CTL")]
pub type FlashPwrCtl = crate::Reg<flash_pwr_ctl::FlashPwrCtlSpec>;
#[doc = "Flash power control"]
pub mod flash_pwr_ctl;
#[doc = "FLASH_CMD (rw) register accessor: Command\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_cmd`]
module"]
#[doc(alias = "FLASH_CMD")]
pub type FlashCmd = crate::Reg<flash_cmd::FlashCmdSpec>;
#[doc = "Command"]
pub mod flash_cmd;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
#[doc(alias = "ECC_CTL")]
pub type EccCtl = crate::Reg<ecc_ctl::EccCtlSpec>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "FM_SRAM_ECC_CTL0 (rw) register accessor: eCT Flash SRAM ECC control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl0`]
module"]
#[doc(alias = "FM_SRAM_ECC_CTL0")]
pub type FmSramEccCtl0 = crate::Reg<fm_sram_ecc_ctl0::FmSramEccCtl0Spec>;
#[doc = "eCT Flash SRAM ECC control 0"]
pub mod fm_sram_ecc_ctl0;
#[doc = "FM_SRAM_ECC_CTL1 (rw) register accessor: eCT Flash SRAM ECC control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl1`]
module"]
#[doc(alias = "FM_SRAM_ECC_CTL1")]
pub type FmSramEccCtl1 = crate::Reg<fm_sram_ecc_ctl1::FmSramEccCtl1Spec>;
#[doc = "eCT Flash SRAM ECC control 1"]
pub mod fm_sram_ecc_ctl1;
#[doc = "FM_SRAM_ECC_CTL2 (r) register accessor: eCT Flash SRAM ECC control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl2`]
module"]
#[doc(alias = "FM_SRAM_ECC_CTL2")]
pub type FmSramEccCtl2 = crate::Reg<fm_sram_ecc_ctl2::FmSramEccCtl2Spec>;
#[doc = "eCT Flash SRAM ECC control 2"]
pub mod fm_sram_ecc_ctl2;
#[doc = "FM_SRAM_ECC_CTL3 (rw) register accessor: eCT Flash SRAM ECC control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_sram_ecc_ctl3`]
module"]
#[doc(alias = "FM_SRAM_ECC_CTL3")]
pub type FmSramEccCtl3 = crate::Reg<fm_sram_ecc_ctl3::FmSramEccCtl3Spec>;
#[doc = "eCT Flash SRAM ECC control 3"]
pub mod fm_sram_ecc_ctl3;
#[doc = "CM0_CA_CTL0 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl0`]
module"]
#[doc(alias = "CM0_CA_CTL0")]
pub type Cm0CaCtl0 = crate::Reg<cm0_ca_ctl0::Cm0CaCtl0Spec>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl0;
#[doc = "CM0_CA_CTL1 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl1`]
module"]
#[doc(alias = "CM0_CA_CTL1")]
pub type Cm0CaCtl1 = crate::Reg<cm0_ca_ctl1::Cm0CaCtl1Spec>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl1;
#[doc = "CM0_CA_CTL2 (rw) register accessor: CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_ctl2`]
module"]
#[doc(alias = "CM0_CA_CTL2")]
pub type Cm0CaCtl2 = crate::Reg<cm0_ca_ctl2::Cm0CaCtl2Spec>;
#[doc = "CM0+ cache control"]
pub mod cm0_ca_ctl2;
#[doc = "CM0_CA_STATUS0 (r) register accessor: CM0+ cache status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status0`]
module"]
#[doc(alias = "CM0_CA_STATUS0")]
pub type Cm0CaStatus0 = crate::Reg<cm0_ca_status0::Cm0CaStatus0Spec>;
#[doc = "CM0+ cache status 0"]
pub mod cm0_ca_status0;
#[doc = "CM0_CA_STATUS1 (r) register accessor: CM0+ cache status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status1`]
module"]
#[doc(alias = "CM0_CA_STATUS1")]
pub type Cm0CaStatus1 = crate::Reg<cm0_ca_status1::Cm0CaStatus1Spec>;
#[doc = "CM0+ cache status 1"]
pub mod cm0_ca_status1;
#[doc = "CM0_CA_STATUS2 (r) register accessor: CM0+ cache status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ca_status2`]
module"]
#[doc(alias = "CM0_CA_STATUS2")]
pub type Cm0CaStatus2 = crate::Reg<cm0_ca_status2::Cm0CaStatus2Spec>;
#[doc = "CM0+ cache status 2"]
pub mod cm0_ca_status2;
#[doc = "CM0_STATUS (rw) register accessor: CM0+ interface status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_status`]
module"]
#[doc(alias = "CM0_STATUS")]
pub type Cm0Status = crate::Reg<cm0_status::Cm0StatusSpec>;
#[doc = "CM0+ interface status"]
pub mod cm0_status;
#[doc = "CM4_CA_CTL0 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ca_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl0`]
module"]
#[doc(alias = "CM4_CA_CTL0")]
pub type Cm4CaCtl0 = crate::Reg<cm4_ca_ctl0::Cm4CaCtl0Spec>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl0;
#[doc = "CM4_CA_CTL1 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ca_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl1`]
module"]
#[doc(alias = "CM4_CA_CTL1")]
pub type Cm4CaCtl1 = crate::Reg<cm4_ca_ctl1::Cm4CaCtl1Spec>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl1;
#[doc = "CM4_CA_CTL2 (rw) register accessor: CM4 cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ca_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_ctl2`]
module"]
#[doc(alias = "CM4_CA_CTL2")]
pub type Cm4CaCtl2 = crate::Reg<cm4_ca_ctl2::Cm4CaCtl2Spec>;
#[doc = "CM4 cache control"]
pub mod cm4_ca_ctl2;
#[doc = "CM4_CA_STATUS0 (r) register accessor: CM4 cache status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status0`]
module"]
#[doc(alias = "CM4_CA_STATUS0")]
pub type Cm4CaStatus0 = crate::Reg<cm4_ca_status0::Cm4CaStatus0Spec>;
#[doc = "CM4 cache status 0"]
pub mod cm4_ca_status0;
#[doc = "CM4_CA_STATUS1 (r) register accessor: CM4 cache status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status1`]
module"]
#[doc(alias = "CM4_CA_STATUS1")]
pub type Cm4CaStatus1 = crate::Reg<cm4_ca_status1::Cm4CaStatus1Spec>;
#[doc = "CM4 cache status 1"]
pub mod cm4_ca_status1;
#[doc = "CM4_CA_STATUS2 (r) register accessor: CM4 cache status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ca_status2`]
module"]
#[doc(alias = "CM4_CA_STATUS2")]
pub type Cm4CaStatus2 = crate::Reg<cm4_ca_status2::Cm4CaStatus2Spec>;
#[doc = "CM4 cache status 2"]
pub mod cm4_ca_status2;
#[doc = "CM4_STATUS (rw) register accessor: CM4 interface status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_status`]
module"]
#[doc(alias = "CM4_STATUS")]
pub type Cm4Status = crate::Reg<cm4_status::Cm4StatusSpec>;
#[doc = "CM4 interface status"]
pub mod cm4_status;
#[doc = "CRYPTO_BUFF_CTL (rw) register accessor: Cryptography buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_buff_ctl`]
module"]
#[doc(alias = "CRYPTO_BUFF_CTL")]
pub type CryptoBuffCtl = crate::Reg<crypto_buff_ctl::CryptoBuffCtlSpec>;
#[doc = "Cryptography buffer control"]
pub mod crypto_buff_ctl;
#[doc = "DW0_BUFF_CTL (rw) register accessor: Datawire 0 buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`dw0_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dw0_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dw0_buff_ctl`]
module"]
#[doc(alias = "DW0_BUFF_CTL")]
pub type Dw0BuffCtl = crate::Reg<dw0_buff_ctl::Dw0BuffCtlSpec>;
#[doc = "Datawire 0 buffer control"]
pub mod dw0_buff_ctl;
#[doc = "DW1_BUFF_CTL (rw) register accessor: Datawire 1 buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`dw1_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dw1_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dw1_buff_ctl`]
module"]
#[doc(alias = "DW1_BUFF_CTL")]
pub type Dw1BuffCtl = crate::Reg<dw1_buff_ctl::Dw1BuffCtlSpec>;
#[doc = "Datawire 1 buffer control"]
pub mod dw1_buff_ctl;
#[doc = "DMAC_BUFF_CTL (rw) register accessor: DMA controller buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmac_buff_ctl`]
module"]
#[doc(alias = "DMAC_BUFF_CTL")]
pub type DmacBuffCtl = crate::Reg<dmac_buff_ctl::DmacBuffCtlSpec>;
#[doc = "DMA controller buffer control"]
pub mod dmac_buff_ctl;
#[doc = "EXT_MS0_BUFF_CTL (rw) register accessor: External master 0 buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ms0_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ms0_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ms0_buff_ctl`]
module"]
#[doc(alias = "EXT_MS0_BUFF_CTL")]
pub type ExtMs0BuffCtl = crate::Reg<ext_ms0_buff_ctl::ExtMs0BuffCtlSpec>;
#[doc = "External master 0 buffer control"]
pub mod ext_ms0_buff_ctl;
#[doc = "EXT_MS1_BUFF_CTL (rw) register accessor: External master 1 buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ms1_buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ms1_buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ms1_buff_ctl`]
module"]
#[doc(alias = "EXT_MS1_BUFF_CTL")]
pub type ExtMs1BuffCtl = crate::Reg<ext_ms1_buff_ctl::ExtMs1BuffCtlSpec>;
#[doc = "External master 1 buffer control"]
pub mod ext_ms1_buff_ctl;
#[doc = "Flash Macro Registers"]
pub use self::fm_ctl::FmCtl;
#[doc = r"Cluster"]
#[doc = "Flash Macro Registers"]
pub mod fm_ctl;
