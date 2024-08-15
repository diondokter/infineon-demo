#[repr(C)]
#[doc = "DMA controller channel"]
#[doc(alias = "CH")]
pub struct Ch {
    ctl: Ctl,
    _reserved1: [u8; 0x0c],
    idx: Idx,
    src: Src,
    dst: Dst,
    _reserved4: [u8; 0x04],
    curr: Curr,
    _reserved5: [u8; 0x04],
    tr_cmd: TrCmd,
    _reserved6: [u8; 0x14],
    descr_status: DescrStatus,
    _reserved7: [u8; 0x1c],
    descr_ctl: DescrCtl,
    descr_src: DescrSrc,
    descr_dst: DescrDst,
    descr_x_size: DescrXSize,
    descr_x_incr: DescrXIncr,
    descr_y_size: DescrYSize,
    descr_y_incr: DescrYIncr,
    descr_next: DescrNext,
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
}
impl Ch {
    #[doc = "0x00 - Channel control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - Channel current indices"]
    #[inline(always)]
    pub const fn idx(&self) -> &Idx {
        &self.idx
    }
    #[doc = "0x14 - Channel current source address"]
    #[inline(always)]
    pub const fn src(&self) -> &Src {
        &self.src
    }
    #[doc = "0x18 - Channel current destination address"]
    #[inline(always)]
    pub const fn dst(&self) -> &Dst {
        &self.dst
    }
    #[doc = "0x20 - Channel current descriptor pointer"]
    #[inline(always)]
    pub const fn curr(&self) -> &Curr {
        &self.curr
    }
    #[doc = "0x28 - Channle software trigger"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TrCmd {
        &self.tr_cmd
    }
    #[doc = "0x40 - Channel descriptor status"]
    #[inline(always)]
    pub const fn descr_status(&self) -> &DescrStatus {
        &self.descr_status
    }
    #[doc = "0x60 - Channel descriptor control"]
    #[inline(always)]
    pub const fn descr_ctl(&self) -> &DescrCtl {
        &self.descr_ctl
    }
    #[doc = "0x64 - Channel descriptor source"]
    #[inline(always)]
    pub const fn descr_src(&self) -> &DescrSrc {
        &self.descr_src
    }
    #[doc = "0x68 - Channel descriptor destination"]
    #[inline(always)]
    pub const fn descr_dst(&self) -> &DescrDst {
        &self.descr_dst
    }
    #[doc = "0x6c - Channel descriptor X size"]
    #[inline(always)]
    pub const fn descr_x_size(&self) -> &DescrXSize {
        &self.descr_x_size
    }
    #[doc = "0x70 - Channel descriptor X increment"]
    #[inline(always)]
    pub const fn descr_x_incr(&self) -> &DescrXIncr {
        &self.descr_x_incr
    }
    #[doc = "0x74 - Channel descriptor Y size"]
    #[inline(always)]
    pub const fn descr_y_size(&self) -> &DescrYSize {
        &self.descr_y_size
    }
    #[doc = "0x78 - Channel descriptor Y increment"]
    #[inline(always)]
    pub const fn descr_y_incr(&self) -> &DescrYIncr {
        &self.descr_y_incr
    }
    #[doc = "0x7c - Channel descriptor next pointer"]
    #[inline(always)]
    pub const fn descr_next(&self) -> &DescrNext {
        &self.descr_next
    }
    #[doc = "0x80 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x84 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x88 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x8c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Channel control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Channel control"]
pub mod ctl;
#[doc = "IDX (r) register accessor: Channel current indices\n\nYou can [`read`](crate::Reg::read) this register and get [`idx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idx`]
module"]
#[doc(alias = "IDX")]
pub type Idx = crate::Reg<idx::IdxSpec>;
#[doc = "Channel current indices"]
pub mod idx;
#[doc = "SRC (r) register accessor: Channel current source address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src`]
module"]
#[doc(alias = "SRC")]
pub type Src = crate::Reg<src::SrcSpec>;
#[doc = "Channel current source address"]
pub mod src;
#[doc = "DST (r) register accessor: Channel current destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst`]
module"]
#[doc(alias = "DST")]
pub type Dst = crate::Reg<dst::DstSpec>;
#[doc = "Channel current destination address"]
pub mod dst;
#[doc = "CURR (rw) register accessor: Channel current descriptor pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`curr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`curr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr`]
module"]
#[doc(alias = "CURR")]
pub type Curr = crate::Reg<curr::CurrSpec>;
#[doc = "Channel current descriptor pointer"]
pub mod curr;
#[doc = "TR_CMD (rw) register accessor: Channle software trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
#[doc(alias = "TR_CMD")]
pub type TrCmd = crate::Reg<tr_cmd::TrCmdSpec>;
#[doc = "Channle software trigger"]
pub mod tr_cmd;
#[doc = "DESCR_STATUS (r) register accessor: Channel descriptor status\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_status`]
module"]
#[doc(alias = "DESCR_STATUS")]
pub type DescrStatus = crate::Reg<descr_status::DescrStatusSpec>;
#[doc = "Channel descriptor status"]
pub mod descr_status;
#[doc = "DESCR_CTL (r) register accessor: Channel descriptor control\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_ctl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_ctl`]
module"]
#[doc(alias = "DESCR_CTL")]
pub type DescrCtl = crate::Reg<descr_ctl::DescrCtlSpec>;
#[doc = "Channel descriptor control"]
pub mod descr_ctl;
#[doc = "DESCR_SRC (r) register accessor: Channel descriptor source\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_src::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_src`]
module"]
#[doc(alias = "DESCR_SRC")]
pub type DescrSrc = crate::Reg<descr_src::DescrSrcSpec>;
#[doc = "Channel descriptor source"]
pub mod descr_src;
#[doc = "DESCR_DST (r) register accessor: Channel descriptor destination\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_dst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_dst`]
module"]
#[doc(alias = "DESCR_DST")]
pub type DescrDst = crate::Reg<descr_dst::DescrDstSpec>;
#[doc = "Channel descriptor destination"]
pub mod descr_dst;
#[doc = "DESCR_X_SIZE (r) register accessor: Channel descriptor X size\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_x_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_x_size`]
module"]
#[doc(alias = "DESCR_X_SIZE")]
pub type DescrXSize = crate::Reg<descr_x_size::DescrXSizeSpec>;
#[doc = "Channel descriptor X size"]
pub mod descr_x_size;
#[doc = "DESCR_X_INCR (r) register accessor: Channel descriptor X increment\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_x_incr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_x_incr`]
module"]
#[doc(alias = "DESCR_X_INCR")]
pub type DescrXIncr = crate::Reg<descr_x_incr::DescrXIncrSpec>;
#[doc = "Channel descriptor X increment"]
pub mod descr_x_incr;
#[doc = "DESCR_Y_SIZE (r) register accessor: Channel descriptor Y size\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_y_size::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_y_size`]
module"]
#[doc(alias = "DESCR_Y_SIZE")]
pub type DescrYSize = crate::Reg<descr_y_size::DescrYSizeSpec>;
#[doc = "Channel descriptor Y size"]
pub mod descr_y_size;
#[doc = "DESCR_Y_INCR (r) register accessor: Channel descriptor Y increment\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_y_incr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_y_incr`]
module"]
#[doc(alias = "DESCR_Y_INCR")]
pub type DescrYIncr = crate::Reg<descr_y_incr::DescrYIncrSpec>;
#[doc = "Channel descriptor Y increment"]
pub mod descr_y_incr;
#[doc = "DESCR_NEXT (r) register accessor: Channel descriptor next pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_next::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descr_next`]
module"]
#[doc(alias = "DESCR_NEXT")]
pub type DescrNext = crate::Reg<descr_next::DescrNextSpec>;
#[doc = "Channel descriptor next pointer"]
pub mod descr_next;
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
