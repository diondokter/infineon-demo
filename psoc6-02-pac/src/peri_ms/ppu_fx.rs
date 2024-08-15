#[repr(C)]
#[doc = "Fixed protection structure pair"]
#[doc(alias = "PPU_FX")]
pub struct PpuFx {
    sl_addr: SlAddr,
    sl_size: SlSize,
    _reserved2: [u8; 0x08],
    sl_att0: SlAtt0,
    sl_att1: SlAtt1,
    sl_att2: SlAtt2,
    sl_att3: SlAtt3,
    ms_addr: MsAddr,
    ms_size: MsSize,
    _reserved8: [u8; 0x08],
    ms_att0: MsAtt0,
    ms_att1: MsAtt1,
    ms_att2: MsAtt2,
    ms_att3: MsAtt3,
}
impl PpuFx {
    #[doc = "0x00 - Slave region, base address"]
    #[inline(always)]
    pub const fn sl_addr(&self) -> &SlAddr {
        &self.sl_addr
    }
    #[doc = "0x04 - Slave region, size"]
    #[inline(always)]
    pub const fn sl_size(&self) -> &SlSize {
        &self.sl_size
    }
    #[doc = "0x10 - Slave attributes 0"]
    #[inline(always)]
    pub const fn sl_att0(&self) -> &SlAtt0 {
        &self.sl_att0
    }
    #[doc = "0x14 - Slave attributes 1"]
    #[inline(always)]
    pub const fn sl_att1(&self) -> &SlAtt1 {
        &self.sl_att1
    }
    #[doc = "0x18 - Slave attributes 2"]
    #[inline(always)]
    pub const fn sl_att2(&self) -> &SlAtt2 {
        &self.sl_att2
    }
    #[doc = "0x1c - Slave attributes 3"]
    #[inline(always)]
    pub const fn sl_att3(&self) -> &SlAtt3 {
        &self.sl_att3
    }
    #[doc = "0x20 - Master region, base address"]
    #[inline(always)]
    pub const fn ms_addr(&self) -> &MsAddr {
        &self.ms_addr
    }
    #[doc = "0x24 - Master region, size"]
    #[inline(always)]
    pub const fn ms_size(&self) -> &MsSize {
        &self.ms_size
    }
    #[doc = "0x30 - Master attributes 0"]
    #[inline(always)]
    pub const fn ms_att0(&self) -> &MsAtt0 {
        &self.ms_att0
    }
    #[doc = "0x34 - Master attributes 1"]
    #[inline(always)]
    pub const fn ms_att1(&self) -> &MsAtt1 {
        &self.ms_att1
    }
    #[doc = "0x38 - Master attributes 2"]
    #[inline(always)]
    pub const fn ms_att2(&self) -> &MsAtt2 {
        &self.ms_att2
    }
    #[doc = "0x3c - Master attributes 3"]
    #[inline(always)]
    pub const fn ms_att3(&self) -> &MsAtt3 {
        &self.ms_att3
    }
}
#[doc = "SL_ADDR (r) register accessor: Slave region, base address\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_addr`]
module"]
#[doc(alias = "SL_ADDR")]
pub type SlAddr = crate::Reg<sl_addr::SlAddrSpec>;
#[doc = "Slave region, base address"]
pub mod sl_addr;
#[doc = "SL_SIZE (r) register accessor: Slave region, size\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_size`]
module"]
#[doc(alias = "SL_SIZE")]
pub type SlSize = crate::Reg<sl_size::SlSizeSpec>;
#[doc = "Slave region, size"]
pub mod sl_size;
#[doc = "SL_ATT0 (rw) register accessor: Slave attributes 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att0`]
module"]
#[doc(alias = "SL_ATT0")]
pub type SlAtt0 = crate::Reg<sl_att0::SlAtt0Spec>;
#[doc = "Slave attributes 0"]
pub mod sl_att0;
#[doc = "SL_ATT1 (rw) register accessor: Slave attributes 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att1`]
module"]
#[doc(alias = "SL_ATT1")]
pub type SlAtt1 = crate::Reg<sl_att1::SlAtt1Spec>;
#[doc = "Slave attributes 1"]
pub mod sl_att1;
#[doc = "SL_ATT2 (rw) register accessor: Slave attributes 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att2`]
module"]
#[doc(alias = "SL_ATT2")]
pub type SlAtt2 = crate::Reg<sl_att2::SlAtt2Spec>;
#[doc = "Slave attributes 2"]
pub mod sl_att2;
#[doc = "SL_ATT3 (rw) register accessor: Slave attributes 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_att3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_att3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sl_att3`]
module"]
#[doc(alias = "SL_ATT3")]
pub type SlAtt3 = crate::Reg<sl_att3::SlAtt3Spec>;
#[doc = "Slave attributes 3"]
pub mod sl_att3;
#[doc = "MS_ADDR (r) register accessor: Master region, base address\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_addr`]
module"]
#[doc(alias = "MS_ADDR")]
pub type MsAddr = crate::Reg<ms_addr::MsAddrSpec>;
#[doc = "Master region, base address"]
pub mod ms_addr;
#[doc = "MS_SIZE (r) register accessor: Master region, size\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_size`]
module"]
#[doc(alias = "MS_SIZE")]
pub type MsSize = crate::Reg<ms_size::MsSizeSpec>;
#[doc = "Master region, size"]
pub mod ms_size;
#[doc = "MS_ATT0 (rw) register accessor: Master attributes 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att0`]
module"]
#[doc(alias = "MS_ATT0")]
pub type MsAtt0 = crate::Reg<ms_att0::MsAtt0Spec>;
#[doc = "Master attributes 0"]
pub mod ms_att0;
#[doc = "MS_ATT1 (rw) register accessor: Master attributes 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att1`]
module"]
#[doc(alias = "MS_ATT1")]
pub type MsAtt1 = crate::Reg<ms_att1::MsAtt1Spec>;
#[doc = "Master attributes 1"]
pub mod ms_att1;
#[doc = "MS_ATT2 (rw) register accessor: Master attributes 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att2`]
module"]
#[doc(alias = "MS_ATT2")]
pub type MsAtt2 = crate::Reg<ms_att2::MsAtt2Spec>;
#[doc = "Master attributes 2"]
pub mod ms_att2;
#[doc = "MS_ATT3 (rw) register accessor: Master attributes 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_att3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_att3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_att3`]
module"]
#[doc(alias = "MS_ATT3")]
pub type MsAtt3 = crate::Reg<ms_att3::MsAtt3Spec>;
#[doc = "Master attributes 3"]
pub mod ms_att3;
