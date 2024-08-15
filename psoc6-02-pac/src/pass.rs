#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intr_cause: IntrCause,
    _reserved1: [u8; 0x0dfc],
    aref: Aref,
    _reserved2: [u8; 0xfc],
    vref_trim0: VrefTrim0,
    vref_trim1: VrefTrim1,
    vref_trim2: VrefTrim2,
    vref_trim3: VrefTrim3,
    iztat_trim0: IztatTrim0,
    iztat_trim1: IztatTrim1,
    iptat_trim0: IptatTrim0,
    ictat_trim0: IctatTrim0,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &IntrCause {
        &self.intr_cause
    }
    #[doc = "0xe00 - AREF configuration"]
    #[inline(always)]
    pub const fn aref(&self) -> &Aref {
        &self.aref
    }
    #[doc = "0xf00 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim0(&self) -> &VrefTrim0 {
        &self.vref_trim0
    }
    #[doc = "0xf04 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim1(&self) -> &VrefTrim1 {
        &self.vref_trim1
    }
    #[doc = "0xf08 - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim2(&self) -> &VrefTrim2 {
        &self.vref_trim2
    }
    #[doc = "0xf0c - VREF Trim bits"]
    #[inline(always)]
    pub const fn vref_trim3(&self) -> &VrefTrim3 {
        &self.vref_trim3
    }
    #[doc = "0xf10 - IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim0(&self) -> &IztatTrim0 {
        &self.iztat_trim0
    }
    #[doc = "0xf14 - IZTAT Trim bits"]
    #[inline(always)]
    pub const fn iztat_trim1(&self) -> &IztatTrim1 {
        &self.iztat_trim1
    }
    #[doc = "0xf18 - IPTAT Trim bits"]
    #[inline(always)]
    pub const fn iptat_trim0(&self) -> &IptatTrim0 {
        &self.iptat_trim0
    }
    #[doc = "0xf1c - ICTAT Trim bits"]
    #[inline(always)]
    pub const fn ictat_trim0(&self) -> &IctatTrim0 {
        &self.ictat_trim0
    }
}
#[doc = "INTR_CAUSE (r) register accessor: Interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
#[doc(alias = "INTR_CAUSE")]
pub type IntrCause = crate::Reg<intr_cause::IntrCauseSpec>;
#[doc = "Interrupt cause register"]
pub mod intr_cause;
#[doc = "AREF configuration"]
pub use self::aref::Aref;
#[doc = r"Cluster"]
#[doc = "AREF configuration"]
pub mod aref;
#[doc = "VREF_TRIM0 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim0`]
module"]
#[doc(alias = "VREF_TRIM0")]
pub type VrefTrim0 = crate::Reg<vref_trim0::VrefTrim0Spec>;
#[doc = "VREF Trim bits"]
pub mod vref_trim0;
#[doc = "VREF_TRIM1 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim1`]
module"]
#[doc(alias = "VREF_TRIM1")]
pub type VrefTrim1 = crate::Reg<vref_trim1::VrefTrim1Spec>;
#[doc = "VREF Trim bits"]
pub mod vref_trim1;
#[doc = "VREF_TRIM2 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim2`]
module"]
#[doc(alias = "VREF_TRIM2")]
pub type VrefTrim2 = crate::Reg<vref_trim2::VrefTrim2Spec>;
#[doc = "VREF Trim bits"]
pub mod vref_trim2;
#[doc = "VREF_TRIM3 (rw) register accessor: VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vref_trim3`]
module"]
#[doc(alias = "VREF_TRIM3")]
pub type VrefTrim3 = crate::Reg<vref_trim3::VrefTrim3Spec>;
#[doc = "VREF Trim bits"]
pub mod vref_trim3;
#[doc = "IZTAT_TRIM0 (rw) register accessor: IZTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iztat_trim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iztat_trim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iztat_trim0`]
module"]
#[doc(alias = "IZTAT_TRIM0")]
pub type IztatTrim0 = crate::Reg<iztat_trim0::IztatTrim0Spec>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim0;
#[doc = "IZTAT_TRIM1 (rw) register accessor: IZTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iztat_trim1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iztat_trim1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iztat_trim1`]
module"]
#[doc(alias = "IZTAT_TRIM1")]
pub type IztatTrim1 = crate::Reg<iztat_trim1::IztatTrim1Spec>;
#[doc = "IZTAT Trim bits"]
pub mod iztat_trim1;
#[doc = "IPTAT_TRIM0 (rw) register accessor: IPTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iptat_trim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iptat_trim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iptat_trim0`]
module"]
#[doc(alias = "IPTAT_TRIM0")]
pub type IptatTrim0 = crate::Reg<iptat_trim0::IptatTrim0Spec>;
#[doc = "IPTAT Trim bits"]
pub mod iptat_trim0;
#[doc = "ICTAT_TRIM0 (rw) register accessor: ICTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`ictat_trim0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ictat_trim0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ictat_trim0`]
module"]
#[doc(alias = "ICTAT_TRIM0")]
pub type IctatTrim0 = crate::Reg<ictat_trim0::IctatTrim0Spec>;
#[doc = "ICTAT Trim bits"]
pub mod ictat_trim0;
