#[repr(C)]
#[doc = "MPU"]
#[doc(alias = "MPU")]
pub struct Mpu {
    ms_ctl: MsCtl,
    ms_ctl_read_mir: [MsCtlReadMir; 127],
    mpu_struct: (),
}
impl Mpu {
    #[doc = "0x00 - Master control"]
    #[inline(always)]
    pub const fn ms_ctl(&self) -> &MsCtl {
        &self.ms_ctl
    }
    #[doc = "0x04..0x200 - Master control read mirror"]
    #[inline(always)]
    pub const fn ms_ctl_read_mir(&self, n: usize) -> &MsCtlReadMir {
        &self.ms_ctl_read_mir[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x200 - Master control read mirror"]
    #[inline(always)]
    pub fn ms_ctl_read_mir_iter(&self) -> impl Iterator<Item = &MsCtlReadMir> {
        self.ms_ctl_read_mir.iter()
    }
    #[doc = "0x200..0x240 - MPU structure"]
    #[inline(always)]
    pub const fn mpu_struct(&self, n: usize) -> &MpuStruct {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x240 - MPU structure"]
    #[inline(always)]
    pub fn mpu_struct_iter(&self) -> impl Iterator<Item = &MpuStruct> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "MS_CTL (rw) register accessor: Master control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_ctl`]
module"]
#[doc(alias = "MS_CTL")]
pub type MsCtl = crate::Reg<ms_ctl::MsCtlSpec>;
#[doc = "Master control"]
pub mod ms_ctl;
#[doc = "MS_CTL_READ_MIR (r) register accessor: Master control read mirror\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_ctl_read_mir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ms_ctl_read_mir`]
module"]
#[doc(alias = "MS_CTL_READ_MIR")]
pub type MsCtlReadMir = crate::Reg<ms_ctl_read_mir::MsCtlReadMirSpec>;
#[doc = "Master control read mirror"]
pub mod ms_ctl_read_mir;
#[doc = "MPU structure"]
pub use self::mpu_struct::MpuStruct;
#[doc = r"Cluster"]
#[doc = "MPU structure"]
pub mod mpu_struct;
