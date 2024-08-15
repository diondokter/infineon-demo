#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    _reserved1: [u8; 0x04],
    active: Active,
    _reserved2: [u8; 0x0ff4],
    ch: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - Active channels"]
    #[inline(always)]
    pub const fn active(&self) -> &Active {
        &self.active
    }
    #[doc = "0x1000..0x1240 - DMA controller channel"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1240 - DMA controller channel"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(4096)
                .add(256 * n)
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
#[doc = "ACTIVE (r) register accessor: Active channels\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@active`]
module"]
#[doc(alias = "ACTIVE")]
pub type Active = crate::Reg<active::ActiveSpec>;
#[doc = "Active channels"]
pub mod active;
#[doc = "DMA controller channel"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "DMA controller channel"]
pub mod ch;
