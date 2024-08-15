#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    prt: (),
    _reserved1: [u8; 0x2000],
    amux_split_ctl: [AmuxSplitCtl; 64],
    _reserved2: [u8; 0x0100],
    monitor_ctl_0: MonitorCtl0,
    monitor_ctl_1: MonitorCtl1,
    monitor_ctl_2: MonitorCtl2,
    monitor_ctl_3: MonitorCtl3,
    _reserved6: [u8; 0x30],
    alt_jtag_en: AltJtagEn,
}
impl RegisterBlock {
    #[doc = "0x00..0x78 - HSIOM port registers"]
    #[inline(always)]
    pub const fn prt(&self, n: usize) -> &Prt {
        #[allow(clippy::no_effect)]
        [(); 15][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x78 - HSIOM port registers"]
    #[inline(always)]
    pub fn prt_iter(&self) -> impl Iterator<Item = &Prt> {
        (0..15)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(16 * n).cast() })
    }
    #[doc = "0x2000..0x2100 - AMUX splitter cell control"]
    #[inline(always)]
    pub const fn amux_split_ctl(&self, n: usize) -> &AmuxSplitCtl {
        &self.amux_split_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x2100 - AMUX splitter cell control"]
    #[inline(always)]
    pub fn amux_split_ctl_iter(&self) -> impl Iterator<Item = &AmuxSplitCtl> {
        self.amux_split_ctl.iter()
    }
    #[doc = "0x2200 - Power/Ground Monitor cell control 0"]
    #[inline(always)]
    pub const fn monitor_ctl_0(&self) -> &MonitorCtl0 {
        &self.monitor_ctl_0
    }
    #[doc = "0x2204 - Power/Ground Monitor cell control 1"]
    #[inline(always)]
    pub const fn monitor_ctl_1(&self) -> &MonitorCtl1 {
        &self.monitor_ctl_1
    }
    #[doc = "0x2208 - Power/Ground Monitor cell control 2"]
    #[inline(always)]
    pub const fn monitor_ctl_2(&self) -> &MonitorCtl2 {
        &self.monitor_ctl_2
    }
    #[doc = "0x220c - Power/Ground Monitor cell control 3"]
    #[inline(always)]
    pub const fn monitor_ctl_3(&self) -> &MonitorCtl3 {
        &self.monitor_ctl_3
    }
    #[doc = "0x2240 - Alternate JTAG IF selection register"]
    #[inline(always)]
    pub const fn alt_jtag_en(&self) -> &AltJtagEn {
        &self.alt_jtag_en
    }
}
#[doc = "HSIOM port registers"]
pub use self::prt::Prt;
#[doc = r"Cluster"]
#[doc = "HSIOM port registers"]
pub mod prt;
#[doc = "AMUX_SPLIT_CTL (rw) register accessor: AMUX splitter cell control\n\nYou can [`read`](crate::Reg::read) this register and get [`amux_split_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amux_split_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amux_split_ctl`]
module"]
#[doc(alias = "AMUX_SPLIT_CTL")]
pub type AmuxSplitCtl = crate::Reg<amux_split_ctl::AmuxSplitCtlSpec>;
#[doc = "AMUX splitter cell control"]
pub mod amux_split_ctl;
#[doc = "MONITOR_CTL_0 (rw) register accessor: Power/Ground Monitor cell control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`monitor_ctl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`monitor_ctl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_0`]
module"]
#[doc(alias = "MONITOR_CTL_0")]
pub type MonitorCtl0 = crate::Reg<monitor_ctl_0::MonitorCtl0Spec>;
#[doc = "Power/Ground Monitor cell control 0"]
pub mod monitor_ctl_0;
#[doc = "MONITOR_CTL_1 (rw) register accessor: Power/Ground Monitor cell control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`monitor_ctl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`monitor_ctl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_1`]
module"]
#[doc(alias = "MONITOR_CTL_1")]
pub type MonitorCtl1 = crate::Reg<monitor_ctl_1::MonitorCtl1Spec>;
#[doc = "Power/Ground Monitor cell control 1"]
pub mod monitor_ctl_1;
#[doc = "MONITOR_CTL_2 (rw) register accessor: Power/Ground Monitor cell control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`monitor_ctl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`monitor_ctl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_2`]
module"]
#[doc(alias = "MONITOR_CTL_2")]
pub type MonitorCtl2 = crate::Reg<monitor_ctl_2::MonitorCtl2Spec>;
#[doc = "Power/Ground Monitor cell control 2"]
pub mod monitor_ctl_2;
#[doc = "MONITOR_CTL_3 (rw) register accessor: Power/Ground Monitor cell control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`monitor_ctl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`monitor_ctl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@monitor_ctl_3`]
module"]
#[doc(alias = "MONITOR_CTL_3")]
pub type MonitorCtl3 = crate::Reg<monitor_ctl_3::MonitorCtl3Spec>;
#[doc = "Power/Ground Monitor cell control 3"]
pub mod monitor_ctl_3;
#[doc = "ALT_JTAG_EN (rw) register accessor: Alternate JTAG IF selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`alt_jtag_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alt_jtag_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alt_jtag_en`]
module"]
#[doc(alias = "ALT_JTAG_EN")]
pub type AltJtagEn = crate::Reg<alt_jtag_en::AltJtagEnSpec>;
#[doc = "Alternate JTAG IF selection register"]
pub mod alt_jtag_en;
