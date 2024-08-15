#[repr(C)]
#[doc = "DW channel structure"]
#[doc(alias = "CH_STRUCT")]
pub struct ChStruct {
    ch_ctl: ChCtl,
    ch_status: ChStatus,
    ch_idx: ChIdx,
    ch_curr_ptr: ChCurrPtr,
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    sram_data0: SramData0,
    sram_data1: SramData1,
    tr_cmd: TrCmd,
}
impl ChStruct {
    #[doc = "0x00 - Channel control"]
    #[inline(always)]
    pub const fn ch_ctl(&self) -> &ChCtl {
        &self.ch_ctl
    }
    #[doc = "0x04 - Channel status"]
    #[inline(always)]
    pub const fn ch_status(&self) -> &ChStatus {
        &self.ch_status
    }
    #[doc = "0x08 - Channel current indices"]
    #[inline(always)]
    pub const fn ch_idx(&self) -> &ChIdx {
        &self.ch_idx
    }
    #[doc = "0x0c - Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn ch_curr_ptr(&self) -> &ChCurrPtr {
        &self.ch_curr_ptr
    }
    #[doc = "0x10 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x14 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x18 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x1c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x20 - SRAM data 0"]
    #[inline(always)]
    pub const fn sram_data0(&self) -> &SramData0 {
        &self.sram_data0
    }
    #[doc = "0x24 - SRAM data 1"]
    #[inline(always)]
    pub const fn sram_data1(&self) -> &SramData1 {
        &self.sram_data1
    }
    #[doc = "0x28 - Channel software trigger"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TrCmd {
        &self.tr_cmd
    }
}
#[doc = "CH_CTL (rw) register accessor: Channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ctl`]
module"]
#[doc(alias = "CH_CTL")]
pub type ChCtl = crate::Reg<ch_ctl::ChCtlSpec>;
#[doc = "Channel control"]
pub mod ch_ctl;
#[doc = "CH_STATUS (r) register accessor: Channel status\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_status`]
module"]
#[doc(alias = "CH_STATUS")]
pub type ChStatus = crate::Reg<ch_status::ChStatusSpec>;
#[doc = "Channel status"]
pub mod ch_status;
#[doc = "CH_IDX (rw) register accessor: Channel current indices\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_idx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_idx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_idx`]
module"]
#[doc(alias = "CH_IDX")]
pub type ChIdx = crate::Reg<ch_idx::ChIdxSpec>;
#[doc = "Channel current indices"]
pub mod ch_idx;
#[doc = "CH_CURR_PTR (rw) register accessor: Channel current descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_curr_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_curr_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_curr_ptr`]
module"]
#[doc(alias = "CH_CURR_PTR")]
pub type ChCurrPtr = crate::Reg<ch_curr_ptr::ChCurrPtrSpec>;
#[doc = "Channel current descriptor pointer"]
pub mod ch_curr_ptr;
#[doc = "INTR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
#[doc = "SRAM_DATA0 (rw) register accessor: SRAM data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data0`]
module"]
#[doc(alias = "SRAM_DATA0")]
pub type SramData0 = crate::Reg<sram_data0::SramData0Spec>;
#[doc = "SRAM data 0"]
pub mod sram_data0;
#[doc = "SRAM_DATA1 (rw) register accessor: SRAM data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_data1`]
module"]
#[doc(alias = "SRAM_DATA1")]
pub type SramData1 = crate::Reg<sram_data1::SramData1Spec>;
#[doc = "SRAM data 1"]
pub mod sram_data1;
#[doc = "TR_CMD (rw) register accessor: Channel software trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
#[doc(alias = "TR_CMD")]
pub type TrCmd = crate::Reg<tr_cmd::TrCmdSpec>;
#[doc = "Channel software trigger"]
pub mod tr_cmd;
