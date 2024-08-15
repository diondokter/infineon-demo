#[repr(C)]
#[doc = "SMPU"]
#[doc(alias = "SMPU")]
pub struct Smpu {
    ms0_ctl: Ms0Ctl,
    ms1_ctl: Ms1Ctl,
    ms2_ctl: Ms2Ctl,
    ms3_ctl: Ms3Ctl,
    ms4_ctl: Ms4Ctl,
    ms5_ctl: Ms5Ctl,
    ms6_ctl: Ms6Ctl,
    ms7_ctl: Ms7Ctl,
    ms8_ctl: Ms8Ctl,
    ms9_ctl: Ms9Ctl,
    ms10_ctl: Ms10Ctl,
    ms11_ctl: Ms11Ctl,
    ms12_ctl: Ms12Ctl,
    ms13_ctl: Ms13Ctl,
    ms14_ctl: Ms14Ctl,
    ms15_ctl: Ms15Ctl,
    _reserved16: [u8; 0x1fc0],
    smpu_struct: (),
}
impl Smpu {
    #[doc = "0x00 - Master 0 protection context control"]
    #[inline(always)]
    pub const fn ms0_ctl(&self) -> &Ms0Ctl {
        &self.ms0_ctl
    }
    #[doc = "0x04 - Master 1 protection context control"]
    #[inline(always)]
    pub const fn ms1_ctl(&self) -> &Ms1Ctl {
        &self.ms1_ctl
    }
    #[doc = "0x08 - Master 2 protection context control"]
    #[inline(always)]
    pub const fn ms2_ctl(&self) -> &Ms2Ctl {
        &self.ms2_ctl
    }
    #[doc = "0x0c - Master 3 protection context control"]
    #[inline(always)]
    pub const fn ms3_ctl(&self) -> &Ms3Ctl {
        &self.ms3_ctl
    }
    #[doc = "0x10 - Master 4 protection context control"]
    #[inline(always)]
    pub const fn ms4_ctl(&self) -> &Ms4Ctl {
        &self.ms4_ctl
    }
    #[doc = "0x14 - Master 5 protection context control"]
    #[inline(always)]
    pub const fn ms5_ctl(&self) -> &Ms5Ctl {
        &self.ms5_ctl
    }
    #[doc = "0x18 - Master 6 protection context control"]
    #[inline(always)]
    pub const fn ms6_ctl(&self) -> &Ms6Ctl {
        &self.ms6_ctl
    }
    #[doc = "0x1c - Master 7 protection context control"]
    #[inline(always)]
    pub const fn ms7_ctl(&self) -> &Ms7Ctl {
        &self.ms7_ctl
    }
    #[doc = "0x20 - Master 8 protection context control"]
    #[inline(always)]
    pub const fn ms8_ctl(&self) -> &Ms8Ctl {
        &self.ms8_ctl
    }
    #[doc = "0x24 - Master 9 protection context control"]
    #[inline(always)]
    pub const fn ms9_ctl(&self) -> &Ms9Ctl {
        &self.ms9_ctl
    }
    #[doc = "0x28 - Master 10 protection context control"]
    #[inline(always)]
    pub const fn ms10_ctl(&self) -> &Ms10Ctl {
        &self.ms10_ctl
    }
    #[doc = "0x2c - Master 11 protection context control"]
    #[inline(always)]
    pub const fn ms11_ctl(&self) -> &Ms11Ctl {
        &self.ms11_ctl
    }
    #[doc = "0x30 - Master 12 protection context control"]
    #[inline(always)]
    pub const fn ms12_ctl(&self) -> &Ms12Ctl {
        &self.ms12_ctl
    }
    #[doc = "0x34 - Master 13 protection context control"]
    #[inline(always)]
    pub const fn ms13_ctl(&self) -> &Ms13Ctl {
        &self.ms13_ctl
    }
    #[doc = "0x38 - Master 14 protection context control"]
    #[inline(always)]
    pub const fn ms14_ctl(&self) -> &Ms14Ctl {
        &self.ms14_ctl
    }
    #[doc = "0x3c - Master 15 protection context control"]
    #[inline(always)]
    pub const fn ms15_ctl(&self) -> &Ms15Ctl {
        &self.ms15_ctl
    }
    #[doc = "0x2000..0x2280 - SMPU structure"]
    #[inline(always)]
    pub const fn smpu_struct(&self, n: usize) -> &SmpuStruct {
        #[allow(clippy::no_effect)]
        [(); 16][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2280 - SMPU structure"]
    #[inline(always)]
    pub fn smpu_struct_iter(&self) -> impl Iterator<Item = &SmpuStruct> {
        (0..16).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(8192)
                .add(64 * n)
                .cast()
        })
    }
}
#[doc = "MS0_CTL (rw) register accessor: Master 0 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms0_ctl`]
module"]
#[doc(alias = "MS0_CTL")]
pub type Ms0Ctl = crate::Reg<ms0_ctl::Ms0CtlSpec>;
#[doc = "Master 0 protection context control"]
pub mod ms0_ctl;
#[doc = "MS1_CTL (rw) register accessor: Master 1 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms1_ctl`]
module"]
#[doc(alias = "MS1_CTL")]
pub type Ms1Ctl = crate::Reg<ms1_ctl::Ms1CtlSpec>;
#[doc = "Master 1 protection context control"]
pub mod ms1_ctl;
#[doc = "MS2_CTL (rw) register accessor: Master 2 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms2_ctl`]
module"]
#[doc(alias = "MS2_CTL")]
pub type Ms2Ctl = crate::Reg<ms2_ctl::Ms2CtlSpec>;
#[doc = "Master 2 protection context control"]
pub mod ms2_ctl;
#[doc = "MS3_CTL (rw) register accessor: Master 3 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms3_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms3_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms3_ctl`]
module"]
#[doc(alias = "MS3_CTL")]
pub type Ms3Ctl = crate::Reg<ms3_ctl::Ms3CtlSpec>;
#[doc = "Master 3 protection context control"]
pub mod ms3_ctl;
#[doc = "MS4_CTL (rw) register accessor: Master 4 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms4_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms4_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms4_ctl`]
module"]
#[doc(alias = "MS4_CTL")]
pub type Ms4Ctl = crate::Reg<ms4_ctl::Ms4CtlSpec>;
#[doc = "Master 4 protection context control"]
pub mod ms4_ctl;
#[doc = "MS5_CTL (rw) register accessor: Master 5 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms5_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms5_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms5_ctl`]
module"]
#[doc(alias = "MS5_CTL")]
pub type Ms5Ctl = crate::Reg<ms5_ctl::Ms5CtlSpec>;
#[doc = "Master 5 protection context control"]
pub mod ms5_ctl;
#[doc = "MS6_CTL (rw) register accessor: Master 6 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms6_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms6_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms6_ctl`]
module"]
#[doc(alias = "MS6_CTL")]
pub type Ms6Ctl = crate::Reg<ms6_ctl::Ms6CtlSpec>;
#[doc = "Master 6 protection context control"]
pub mod ms6_ctl;
#[doc = "MS7_CTL (rw) register accessor: Master 7 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms7_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms7_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms7_ctl`]
module"]
#[doc(alias = "MS7_CTL")]
pub type Ms7Ctl = crate::Reg<ms7_ctl::Ms7CtlSpec>;
#[doc = "Master 7 protection context control"]
pub mod ms7_ctl;
#[doc = "MS8_CTL (rw) register accessor: Master 8 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms8_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms8_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms8_ctl`]
module"]
#[doc(alias = "MS8_CTL")]
pub type Ms8Ctl = crate::Reg<ms8_ctl::Ms8CtlSpec>;
#[doc = "Master 8 protection context control"]
pub mod ms8_ctl;
#[doc = "MS9_CTL (rw) register accessor: Master 9 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms9_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms9_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms9_ctl`]
module"]
#[doc(alias = "MS9_CTL")]
pub type Ms9Ctl = crate::Reg<ms9_ctl::Ms9CtlSpec>;
#[doc = "Master 9 protection context control"]
pub mod ms9_ctl;
#[doc = "MS10_CTL (rw) register accessor: Master 10 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms10_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms10_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms10_ctl`]
module"]
#[doc(alias = "MS10_CTL")]
pub type Ms10Ctl = crate::Reg<ms10_ctl::Ms10CtlSpec>;
#[doc = "Master 10 protection context control"]
pub mod ms10_ctl;
#[doc = "MS11_CTL (rw) register accessor: Master 11 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms11_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms11_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms11_ctl`]
module"]
#[doc(alias = "MS11_CTL")]
pub type Ms11Ctl = crate::Reg<ms11_ctl::Ms11CtlSpec>;
#[doc = "Master 11 protection context control"]
pub mod ms11_ctl;
#[doc = "MS12_CTL (rw) register accessor: Master 12 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms12_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms12_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms12_ctl`]
module"]
#[doc(alias = "MS12_CTL")]
pub type Ms12Ctl = crate::Reg<ms12_ctl::Ms12CtlSpec>;
#[doc = "Master 12 protection context control"]
pub mod ms12_ctl;
#[doc = "MS13_CTL (rw) register accessor: Master 13 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms13_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms13_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms13_ctl`]
module"]
#[doc(alias = "MS13_CTL")]
pub type Ms13Ctl = crate::Reg<ms13_ctl::Ms13CtlSpec>;
#[doc = "Master 13 protection context control"]
pub mod ms13_ctl;
#[doc = "MS14_CTL (rw) register accessor: Master 14 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms14_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms14_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms14_ctl`]
module"]
#[doc(alias = "MS14_CTL")]
pub type Ms14Ctl = crate::Reg<ms14_ctl::Ms14CtlSpec>;
#[doc = "Master 14 protection context control"]
pub mod ms14_ctl;
#[doc = "MS15_CTL (rw) register accessor: Master 15 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms15_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms15_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms15_ctl`]
module"]
#[doc(alias = "MS15_CTL")]
pub type Ms15Ctl = crate::Reg<ms15_ctl::Ms15CtlSpec>;
#[doc = "Master 15 protection context control"]
pub mod ms15_ctl;
#[doc = "SMPU structure"]
pub use self::smpu_struct::SmpuStruct;
#[doc = r"Cluster"]
#[doc = "SMPU structure"]
pub mod smpu_struct;
