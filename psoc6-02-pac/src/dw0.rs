#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    status: Status,
    _reserved2: [u8; 0x18],
    act_descr_ctl: ActDescrCtl,
    act_descr_src: ActDescrSrc,
    act_descr_dst: ActDescrDst,
    _reserved5: [u8; 0x04],
    act_descr_x_ctl: ActDescrXCtl,
    act_descr_y_ctl: ActDescrYCtl,
    act_descr_next_ptr: ActDescrNextPtr,
    _reserved8: [u8; 0x04],
    act_src: ActSrc,
    act_dst: ActDst,
    _reserved10: [u8; 0x38],
    ecc_ctl: EccCtl,
    _reserved11: [u8; 0x7c],
    crc_ctl: CrcCtl,
    _reserved12: [u8; 0x0c],
    crc_data_ctl: CrcDataCtl,
    _reserved13: [u8; 0x0c],
    crc_pol_ctl: CrcPolCtl,
    _reserved14: [u8; 0x0c],
    crc_lfsr_ctl: CrcLfsrCtl,
    _reserved15: [u8; 0x0c],
    crc_rem_ctl: CrcRemCtl,
    _reserved16: [u8; 0x04],
    crc_rem_result: CrcRemResult,
    _reserved17: [u8; 0x7eb4],
    ch_struct: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x20 - Active descriptor control"]
    #[inline(always)]
    pub const fn act_descr_ctl(&self) -> &ActDescrCtl {
        &self.act_descr_ctl
    }
    #[doc = "0x24 - Active descriptor source"]
    #[inline(always)]
    pub const fn act_descr_src(&self) -> &ActDescrSrc {
        &self.act_descr_src
    }
    #[doc = "0x28 - Active descriptor destination"]
    #[inline(always)]
    pub const fn act_descr_dst(&self) -> &ActDescrDst {
        &self.act_descr_dst
    }
    #[doc = "0x30 - Active descriptor X loop control"]
    #[inline(always)]
    pub const fn act_descr_x_ctl(&self) -> &ActDescrXCtl {
        &self.act_descr_x_ctl
    }
    #[doc = "0x34 - Active descriptor Y loop control"]
    #[inline(always)]
    pub const fn act_descr_y_ctl(&self) -> &ActDescrYCtl {
        &self.act_descr_y_ctl
    }
    #[doc = "0x38 - Active descriptor next pointer"]
    #[inline(always)]
    pub const fn act_descr_next_ptr(&self) -> &ActDescrNextPtr {
        &self.act_descr_next_ptr
    }
    #[doc = "0x40 - Active source"]
    #[inline(always)]
    pub const fn act_src(&self) -> &ActSrc {
        &self.act_src
    }
    #[doc = "0x44 - Active destination"]
    #[inline(always)]
    pub const fn act_dst(&self) -> &ActDst {
        &self.act_dst
    }
    #[doc = "0x80 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &EccCtl {
        &self.ecc_ctl
    }
    #[doc = "0x100 - CRC control"]
    #[inline(always)]
    pub const fn crc_ctl(&self) -> &CrcCtl {
        &self.crc_ctl
    }
    #[doc = "0x110 - CRC data control"]
    #[inline(always)]
    pub const fn crc_data_ctl(&self) -> &CrcDataCtl {
        &self.crc_data_ctl
    }
    #[doc = "0x120 - CRC polynomial control"]
    #[inline(always)]
    pub const fn crc_pol_ctl(&self) -> &CrcPolCtl {
        &self.crc_pol_ctl
    }
    #[doc = "0x130 - CRC LFSR control"]
    #[inline(always)]
    pub const fn crc_lfsr_ctl(&self) -> &CrcLfsrCtl {
        &self.crc_lfsr_ctl
    }
    #[doc = "0x140 - CRC remainder control"]
    #[inline(always)]
    pub const fn crc_rem_ctl(&self) -> &CrcRemCtl {
        &self.crc_rem_ctl
    }
    #[doc = "0x148 - CRC remainder result"]
    #[inline(always)]
    pub const fn crc_rem_result(&self) -> &CrcRemResult {
        &self.crc_rem_result
    }
    #[doc = "0x8000..0x84fc - DW channel structure"]
    #[inline(always)]
    pub const fn ch_struct(&self, n: usize) -> &ChStruct {
        #[allow(clippy::no_effect)]
        [(); 29][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32768)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8000..0x84fc - DW channel structure"]
    #[inline(always)]
    pub fn ch_struct_iter(&self) -> impl Iterator<Item = &ChStruct> {
        (0..29).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32768)
                .add(64 * n)
                .cast()
        })
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "ACT_DESCR_CTL (r) register accessor: Active descriptor control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_ctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_ctl`]
module"]
#[doc(alias = "ACT_DESCR_CTL")]
pub type ActDescrCtl = crate::Reg<act_descr_ctl::ActDescrCtlSpec>;
#[doc = "Active descriptor control"]
pub mod act_descr_ctl;
#[doc = "ACT_DESCR_SRC (r) register accessor: Active descriptor source\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_src`]
module"]
#[doc(alias = "ACT_DESCR_SRC")]
pub type ActDescrSrc = crate::Reg<act_descr_src::ActDescrSrcSpec>;
#[doc = "Active descriptor source"]
pub mod act_descr_src;
#[doc = "ACT_DESCR_DST (r) register accessor: Active descriptor destination\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_dst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_dst`]
module"]
#[doc(alias = "ACT_DESCR_DST")]
pub type ActDescrDst = crate::Reg<act_descr_dst::ActDescrDstSpec>;
#[doc = "Active descriptor destination"]
pub mod act_descr_dst;
#[doc = "ACT_DESCR_X_CTL (r) register accessor: Active descriptor X loop control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_x_ctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_x_ctl`]
module"]
#[doc(alias = "ACT_DESCR_X_CTL")]
pub type ActDescrXCtl = crate::Reg<act_descr_x_ctl::ActDescrXCtlSpec>;
#[doc = "Active descriptor X loop control"]
pub mod act_descr_x_ctl;
#[doc = "ACT_DESCR_Y_CTL (r) register accessor: Active descriptor Y loop control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_y_ctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_y_ctl`]
module"]
#[doc(alias = "ACT_DESCR_Y_CTL")]
pub type ActDescrYCtl = crate::Reg<act_descr_y_ctl::ActDescrYCtlSpec>;
#[doc = "Active descriptor Y loop control"]
pub mod act_descr_y_ctl;
#[doc = "ACT_DESCR_NEXT_PTR (r) register accessor: Active descriptor next pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_next_ptr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_descr_next_ptr`]
module"]
#[doc(alias = "ACT_DESCR_NEXT_PTR")]
pub type ActDescrNextPtr = crate::Reg<act_descr_next_ptr::ActDescrNextPtrSpec>;
#[doc = "Active descriptor next pointer"]
pub mod act_descr_next_ptr;
#[doc = "ACT_SRC (r) register accessor: Active source\n\nYou can [`read`](crate::Reg::read) this register and get [`act_src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_src`]
module"]
#[doc(alias = "ACT_SRC")]
pub type ActSrc = crate::Reg<act_src::ActSrcSpec>;
#[doc = "Active source"]
pub mod act_src;
#[doc = "ACT_DST (r) register accessor: Active destination\n\nYou can [`read`](crate::Reg::read) this register and get [`act_dst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@act_dst`]
module"]
#[doc(alias = "ACT_DST")]
pub type ActDst = crate::Reg<act_dst::ActDstSpec>;
#[doc = "Active destination"]
pub mod act_dst;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
#[doc(alias = "ECC_CTL")]
pub type EccCtl = crate::Reg<ecc_ctl::EccCtlSpec>;
#[doc = "ECC control"]
pub mod ecc_ctl;
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
#[doc = "CRC_LFSR_CTL (rw) register accessor: CRC LFSR control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_lfsr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_lfsr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_lfsr_ctl`]
module"]
#[doc(alias = "CRC_LFSR_CTL")]
pub type CrcLfsrCtl = crate::Reg<crc_lfsr_ctl::CrcLfsrCtlSpec>;
#[doc = "CRC LFSR control"]
pub mod crc_lfsr_ctl;
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
#[doc = "DW channel structure"]
pub use self::ch_struct::ChStruct;
#[doc = r"Cluster"]
#[doc = "DW channel structure"]
pub mod ch_struct;
