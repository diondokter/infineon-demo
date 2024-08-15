#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    status: Status,
    _reserved2: [u8; 0x08],
    cmd: Cmd,
    _reserved3: [u8; 0x07ac],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    _reserved7: [u8; 0x30],
    cnt_struct: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Profile control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Profile status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - Profile command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x7c0 - Profile interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x7c4 - Profile interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x7c8 - Profile interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x7cc - Profile interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x800..0x860 - Profile counter structure"]
    #[inline(always)]
    pub const fn cnt_struct(&self, n: usize) -> &CntStruct {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x860 - Profile counter structure"]
    #[inline(always)]
    pub fn cnt_struct_iter(&self) -> impl Iterator<Item = &CntStruct> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "CTL (rw) register accessor: Profile control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Profile control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: Profile status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Profile status"]
pub mod status;
#[doc = "CMD (rw) register accessor: Profile command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Profile command"]
pub mod cmd;
#[doc = "INTR (rw) register accessor: Profile interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Profile interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Profile interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Profile interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Profile interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Profile interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Profile interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Profile interrupt masked"]
pub mod intr_masked;
#[doc = "Profile counter structure"]
pub use self::cnt_struct::CntStruct;
#[doc = r"Cluster"]
#[doc = "Profile counter structure"]
pub mod cnt_struct;
