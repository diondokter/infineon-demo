#[repr(C)]
#[doc = "Fault structure"]
#[doc(alias = "STRUCT")]
pub struct Struct {
    ctl: Ctl,
    _reserved1: [u8; 0x08],
    status: Status,
    data: [Data; 4],
    _reserved3: [u8; 0x20],
    pending0: Pending0,
    pending1: Pending1,
    pending2: Pending2,
    _reserved6: [u8; 0x04],
    mask0: Mask0,
    mask1: Mask1,
    mask2: Mask2,
    _reserved9: [u8; 0x64],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
}
impl Struct {
    #[doc = "0x00 - Fault control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x0c - Fault status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10..0x20 - Fault data"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - Fault data"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x40 - Fault pending 0"]
    #[inline(always)]
    pub const fn pending0(&self) -> &Pending0 {
        &self.pending0
    }
    #[doc = "0x44 - Fault pending 1"]
    #[inline(always)]
    pub const fn pending1(&self) -> &Pending1 {
        &self.pending1
    }
    #[doc = "0x48 - Fault pending 2"]
    #[inline(always)]
    pub const fn pending2(&self) -> &Pending2 {
        &self.pending2
    }
    #[doc = "0x50 - Fault mask 0"]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
    #[doc = "0x54 - Fault mask 1"]
    #[inline(always)]
    pub const fn mask1(&self) -> &Mask1 {
        &self.mask1
    }
    #[doc = "0x58 - Fault mask 2"]
    #[inline(always)]
    pub const fn mask2(&self) -> &Mask2 {
        &self.mask2
    }
    #[doc = "0xc0 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0xc4 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0xc8 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0xcc - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Fault control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Fault control"]
pub mod ctl;
#[doc = "STATUS (rw) register accessor: Fault status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Fault status"]
pub mod status;
#[doc = "DATA (rw) register accessor: Fault data\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Fault data"]
pub mod data;
#[doc = "PENDING0 (r) register accessor: Fault pending 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pending0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending0`]
module"]
#[doc(alias = "PENDING0")]
pub type Pending0 = crate::Reg<pending0::Pending0Spec>;
#[doc = "Fault pending 0"]
pub mod pending0;
#[doc = "PENDING1 (r) register accessor: Fault pending 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pending1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending1`]
module"]
#[doc(alias = "PENDING1")]
pub type Pending1 = crate::Reg<pending1::Pending1Spec>;
#[doc = "Fault pending 1"]
pub mod pending1;
#[doc = "PENDING2 (r) register accessor: Fault pending 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pending2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending2`]
module"]
#[doc(alias = "PENDING2")]
pub type Pending2 = crate::Reg<pending2::Pending2Spec>;
#[doc = "Fault pending 2"]
pub mod pending2;
#[doc = "MASK0 (rw) register accessor: Fault mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`]
module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "Fault mask 0"]
pub mod mask0;
#[doc = "MASK1 (rw) register accessor: Fault mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mask1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask1`]
module"]
#[doc(alias = "MASK1")]
pub type Mask1 = crate::Reg<mask1::Mask1Spec>;
#[doc = "Fault mask 1"]
pub mod mask1;
#[doc = "MASK2 (rw) register accessor: Fault mask 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mask2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask2`]
module"]
#[doc(alias = "MASK2")]
pub type Mask2 = crate::Reg<mask2::Mask2Spec>;
#[doc = "Fault mask 2"]
pub mod mask2;
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
