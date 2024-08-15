#[repr(C)]
#[doc = "GPIO port registers"]
#[doc(alias = "PRT")]
pub struct Prt {
    out: Out,
    out_clr: OutClr,
    out_set: OutSet,
    out_inv: OutInv,
    in_: In,
    intr: Intr,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    intr_set: IntrSet,
    _reserved9: [u8; 0x1c],
    intr_cfg: IntrCfg,
    cfg: Cfg,
    cfg_in: CfgIn,
    cfg_out: CfgOut,
    cfg_sio: CfgSio,
    _reserved14: [u8; 0x04],
    cfg_in_autolvl: CfgInAutolvl,
}
impl Prt {
    #[doc = "0x00 - Port output data register"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x04 - Port output data clear register"]
    #[inline(always)]
    pub const fn out_clr(&self) -> &OutClr {
        &self.out_clr
    }
    #[doc = "0x08 - Port output data set register"]
    #[inline(always)]
    pub const fn out_set(&self) -> &OutSet {
        &self.out_set
    }
    #[doc = "0x0c - Port output data invert register"]
    #[inline(always)]
    pub const fn out_inv(&self) -> &OutInv {
        &self.out_inv
    }
    #[doc = "0x10 - Port input state register"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x14 - Port interrupt status register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x18 - Port interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x1c - Port interrupt masked status register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x20 - Port interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x40 - Port interrupt configuration register"]
    #[inline(always)]
    pub const fn intr_cfg(&self) -> &IntrCfg {
        &self.intr_cfg
    }
    #[doc = "0x44 - Port configuration register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x48 - Port input buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_in(&self) -> &CfgIn {
        &self.cfg_in
    }
    #[doc = "0x4c - Port output buffer configuration register"]
    #[inline(always)]
    pub const fn cfg_out(&self) -> &CfgOut {
        &self.cfg_out
    }
    #[doc = "0x50 - Port SIO configuration register"]
    #[inline(always)]
    pub const fn cfg_sio(&self) -> &CfgSio {
        &self.cfg_sio
    }
    #[doc = "0x58 - Port input buffer AUTOLVL configuration register"]
    #[inline(always)]
    pub const fn cfg_in_autolvl(&self) -> &CfgInAutolvl {
        &self.cfg_in_autolvl
    }
}
#[doc = "OUT (rw) register accessor: Port output data register\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`]
module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Port output data register"]
pub mod out;
#[doc = "OUT_CLR (rw) register accessor: Port output data clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_clr`]
module"]
#[doc(alias = "OUT_CLR")]
pub type OutClr = crate::Reg<out_clr::OutClrSpec>;
#[doc = "Port output data clear register"]
pub mod out_clr;
#[doc = "OUT_SET (rw) register accessor: Port output data set register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_set`]
module"]
#[doc(alias = "OUT_SET")]
pub type OutSet = crate::Reg<out_set::OutSetSpec>;
#[doc = "Port output data set register"]
pub mod out_set;
#[doc = "OUT_INV (rw) register accessor: Port output data invert register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_inv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_inv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_inv`]
module"]
#[doc(alias = "OUT_INV")]
pub type OutInv = crate::Reg<out_inv::OutInvSpec>;
#[doc = "Port output data invert register"]
pub mod out_inv;
#[doc = "IN (r) register accessor: Port input state register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`]
module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Port input state register"]
pub mod in_;
#[doc = "INTR (rw) register accessor: Port interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Port interrupt status register"]
pub mod intr;
#[doc = "INTR_MASK (rw) register accessor: Port interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Port interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Port interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Port interrupt masked status register"]
pub mod intr_masked;
#[doc = "INTR_SET (rw) register accessor: Port interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Port interrupt set register"]
pub mod intr_set;
#[doc = "INTR_CFG (rw) register accessor: Port interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cfg`]
module"]
#[doc(alias = "INTR_CFG")]
pub type IntrCfg = crate::Reg<intr_cfg::IntrCfgSpec>;
#[doc = "Port interrupt configuration register"]
pub mod intr_cfg;
#[doc = "CFG (rw) register accessor: Port configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Port configuration register"]
pub mod cfg;
#[doc = "CFG_IN (rw) register accessor: Port input buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_in::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_in::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_in`]
module"]
#[doc(alias = "CFG_IN")]
pub type CfgIn = crate::Reg<cfg_in::CfgInSpec>;
#[doc = "Port input buffer configuration register"]
pub mod cfg_in;
#[doc = "CFG_OUT (rw) register accessor: Port output buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_out`]
module"]
#[doc(alias = "CFG_OUT")]
pub type CfgOut = crate::Reg<cfg_out::CfgOutSpec>;
#[doc = "Port output buffer configuration register"]
pub mod cfg_out;
#[doc = "CFG_SIO (rw) register accessor: Port SIO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_sio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_sio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_sio`]
module"]
#[doc(alias = "CFG_SIO")]
pub type CfgSio = crate::Reg<cfg_sio::CfgSioSpec>;
#[doc = "Port SIO configuration register"]
pub mod cfg_sio;
#[doc = "CFG_IN_AUTOLVL (rw) register accessor: Port input buffer AUTOLVL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_in_autolvl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_in_autolvl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg_in_autolvl`]
module"]
#[doc(alias = "CFG_IN_AUTOLVL")]
pub type CfgInAutolvl = crate::Reg<cfg_in_autolvl::CfgInAutolvlSpec>;
#[doc = "Port input buffer AUTOLVL configuration register"]
pub mod cfg_in_autolvl;
