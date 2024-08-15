#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    prt: (),
    _reserved1: [u8; 0x4000],
    intr_cause0: IntrCause0,
    intr_cause1: IntrCause1,
    intr_cause2: IntrCause2,
    intr_cause3: IntrCause3,
    vdd_active: VddActive,
    vdd_intr: VddIntr,
    vdd_intr_mask: VddIntrMask,
    vdd_intr_masked: VddIntrMasked,
    vdd_intr_set: VddIntrSet,
}
impl RegisterBlock {
    #[doc = "0x00..0x564 - GPIO port registers"]
    #[inline(always)]
    pub const fn prt(&self, n: usize) -> &Prt {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x564 - GPIO port registers"]
    #[inline(always)]
    pub fn prt_iter(&self) -> impl Iterator<Item = &Prt> {
        (0..15).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(128 * n)
                .cast()
        })
    }
    #[doc = "0x4000 - Interrupt port cause register 0"]
    #[inline(always)]
    pub const fn intr_cause0(&self) -> &IntrCause0 {
        &self.intr_cause0
    }
    #[doc = "0x4004 - Interrupt port cause register 1"]
    #[inline(always)]
    pub const fn intr_cause1(&self) -> &IntrCause1 {
        &self.intr_cause1
    }
    #[doc = "0x4008 - Interrupt port cause register 2"]
    #[inline(always)]
    pub const fn intr_cause2(&self) -> &IntrCause2 {
        &self.intr_cause2
    }
    #[doc = "0x400c - Interrupt port cause register 3"]
    #[inline(always)]
    pub const fn intr_cause3(&self) -> &IntrCause3 {
        &self.intr_cause3
    }
    #[doc = "0x4010 - Extern power supply detection register"]
    #[inline(always)]
    pub const fn vdd_active(&self) -> &VddActive {
        &self.vdd_active
    }
    #[doc = "0x4014 - Supply detection interrupt register"]
    #[inline(always)]
    pub const fn vdd_intr(&self) -> &VddIntr {
        &self.vdd_intr
    }
    #[doc = "0x4018 - Supply detection interrupt mask register"]
    #[inline(always)]
    pub const fn vdd_intr_mask(&self) -> &VddIntrMask {
        &self.vdd_intr_mask
    }
    #[doc = "0x401c - Supply detection interrupt masked register"]
    #[inline(always)]
    pub const fn vdd_intr_masked(&self) -> &VddIntrMasked {
        &self.vdd_intr_masked
    }
    #[doc = "0x4020 - Supply detection interrupt set register"]
    #[inline(always)]
    pub const fn vdd_intr_set(&self) -> &VddIntrSet {
        &self.vdd_intr_set
    }
}
#[doc = "GPIO port registers"]
pub use self::prt::Prt;
#[doc = r"Cluster"]
#[doc = "GPIO port registers"]
pub mod prt;
#[doc = "INTR_CAUSE0 (r) register accessor: Interrupt port cause register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause0`]
module"]
#[doc(alias = "INTR_CAUSE0")]
pub type IntrCause0 = crate::Reg<intr_cause0::IntrCause0Spec>;
#[doc = "Interrupt port cause register 0"]
pub mod intr_cause0;
#[doc = "INTR_CAUSE1 (r) register accessor: Interrupt port cause register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause1`]
module"]
#[doc(alias = "INTR_CAUSE1")]
pub type IntrCause1 = crate::Reg<intr_cause1::IntrCause1Spec>;
#[doc = "Interrupt port cause register 1"]
pub mod intr_cause1;
#[doc = "INTR_CAUSE2 (r) register accessor: Interrupt port cause register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause2`]
module"]
#[doc(alias = "INTR_CAUSE2")]
pub type IntrCause2 = crate::Reg<intr_cause2::IntrCause2Spec>;
#[doc = "Interrupt port cause register 2"]
pub mod intr_cause2;
#[doc = "INTR_CAUSE3 (r) register accessor: Interrupt port cause register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause3`]
module"]
#[doc(alias = "INTR_CAUSE3")]
pub type IntrCause3 = crate::Reg<intr_cause3::IntrCause3Spec>;
#[doc = "Interrupt port cause register 3"]
pub mod intr_cause3;
#[doc = "VDD_ACTIVE (r) register accessor: Extern power supply detection register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_active::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_active`]
module"]
#[doc(alias = "VDD_ACTIVE")]
pub type VddActive = crate::Reg<vdd_active::VddActiveSpec>;
#[doc = "Extern power supply detection register"]
pub mod vdd_active;
#[doc = "VDD_INTR (rw) register accessor: Supply detection interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr`]
module"]
#[doc(alias = "VDD_INTR")]
pub type VddIntr = crate::Reg<vdd_intr::VddIntrSpec>;
#[doc = "Supply detection interrupt register"]
pub mod vdd_intr;
#[doc = "VDD_INTR_MASK (rw) register accessor: Supply detection interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_mask`]
module"]
#[doc(alias = "VDD_INTR_MASK")]
pub type VddIntrMask = crate::Reg<vdd_intr_mask::VddIntrMaskSpec>;
#[doc = "Supply detection interrupt mask register"]
pub mod vdd_intr_mask;
#[doc = "VDD_INTR_MASKED (r) register accessor: Supply detection interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_masked`]
module"]
#[doc(alias = "VDD_INTR_MASKED")]
pub type VddIntrMasked = crate::Reg<vdd_intr_masked::VddIntrMaskedSpec>;
#[doc = "Supply detection interrupt masked register"]
pub mod vdd_intr_masked;
#[doc = "VDD_INTR_SET (rw) register accessor: Supply detection interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdd_intr_set`]
module"]
#[doc(alias = "VDD_INTR_SET")]
pub type VddIntrSet = crate::Reg<vdd_intr_set::VddIntrSetSpec>;
#[doc = "Supply detection interrupt set register"]
pub mod vdd_intr_set;
