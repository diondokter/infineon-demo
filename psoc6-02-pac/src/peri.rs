#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0200],
    timeout_ctl: TimeoutCtl,
    _reserved1: [u8; 0x1c],
    tr_cmd: TrCmd,
    _reserved2: [u8; 0x01dc],
    div_cmd: DivCmd,
    _reserved3: [u8; 0x07fc],
    clock_ctl: [ClockCtl; 256],
    div_8_ctl: [Div8Ctl; 256],
    div_16_ctl: [Div16Ctl; 256],
    div_16_5_ctl: [Div16_5Ctl; 256],
    div_24_5_ctl: [Div24_5Ctl; 255],
    _reserved8: [u8; 0x04],
    ecc_ctl: EccCtl,
    _reserved9: [u8; 0x1ffc],
    gr: (),
    _reserved10: [u8; 0x4000],
    tr_gr: [TrGr; 10],
    _reserved11: [u8; 0x1800],
    tr_1to1_gr: [Tr1to1Gr; 7],
}
impl RegisterBlock {
    #[doc = "0x200 - Timeout control"]
    #[inline(always)]
    pub const fn timeout_ctl(&self) -> &TimeoutCtl {
        &self.timeout_ctl
    }
    #[doc = "0x220 - Trigger command"]
    #[inline(always)]
    pub const fn tr_cmd(&self) -> &TrCmd {
        &self.tr_cmd
    }
    #[doc = "0x400 - Divider command"]
    #[inline(always)]
    pub const fn div_cmd(&self) -> &DivCmd {
        &self.div_cmd
    }
    #[doc = "0xc00..0x1000 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self, n: usize) -> &ClockCtl {
        &self.clock_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc00..0x1000 - Clock control"]
    #[inline(always)]
    pub fn clock_ctl_iter(&self) -> impl Iterator<Item = &ClockCtl> {
        self.clock_ctl.iter()
    }
    #[doc = "0x1000..0x1400 - Divider control (for 8.0 divider)"]
    #[inline(always)]
    pub const fn div_8_ctl(&self, n: usize) -> &Div8Ctl {
        &self.div_8_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x1400 - Divider control (for 8.0 divider)"]
    #[inline(always)]
    pub fn div_8_ctl_iter(&self) -> impl Iterator<Item = &Div8Ctl> {
        self.div_8_ctl.iter()
    }
    #[doc = "0x1400..0x1800 - Divider control (for 16.0 divider)"]
    #[inline(always)]
    pub const fn div_16_ctl(&self, n: usize) -> &Div16Ctl {
        &self.div_16_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1400..0x1800 - Divider control (for 16.0 divider)"]
    #[inline(always)]
    pub fn div_16_ctl_iter(&self) -> impl Iterator<Item = &Div16Ctl> {
        self.div_16_ctl.iter()
    }
    #[doc = "0x1800..0x1c00 - Divider control (for 16.5 divider)"]
    #[inline(always)]
    pub const fn div_16_5_ctl(&self, n: usize) -> &Div16_5Ctl {
        &self.div_16_5_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1800..0x1c00 - Divider control (for 16.5 divider)"]
    #[inline(always)]
    pub fn div_16_5_ctl_iter(&self) -> impl Iterator<Item = &Div16_5Ctl> {
        self.div_16_5_ctl.iter()
    }
    #[doc = "0x1c00..0x1ffc - Divider control (for 24.5 divider)"]
    #[inline(always)]
    pub const fn div_24_5_ctl(&self, n: usize) -> &Div24_5Ctl {
        &self.div_24_5_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c00..0x1ffc - Divider control (for 24.5 divider)"]
    #[inline(always)]
    pub fn div_24_5_ctl_iter(&self) -> impl Iterator<Item = &Div24_5Ctl> {
        self.div_24_5_ctl.iter()
    }
    #[doc = "0x2000 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &EccCtl {
        &self.ecc_ctl
    }
    #[doc = "0x4000..0x40dc - Peripheral group structure"]
    #[inline(always)]
    pub const fn gr(&self, n: usize) -> &Gr {
        #[allow(clippy::no_effect)]
        [(); 11][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16384)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4000..0x40dc - Peripheral group structure"]
    #[inline(always)]
    pub fn gr_iter(&self) -> impl Iterator<Item = &Gr> {
        (0..11).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16384)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x8000..0xa800 - Trigger group"]
    #[inline(always)]
    pub const fn tr_gr(&self, n: usize) -> &TrGr {
        &self.tr_gr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8000..0xa800 - Trigger group"]
    #[inline(always)]
    pub fn tr_gr_iter(&self) -> impl Iterator<Item = &TrGr> {
        self.tr_gr.iter()
    }
    #[doc = "0xc000..0xdc00 - Trigger 1-to-1 group"]
    #[inline(always)]
    pub const fn tr_1to1_gr(&self, n: usize) -> &Tr1to1Gr {
        &self.tr_1to1_gr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc000..0xdc00 - Trigger 1-to-1 group"]
    #[inline(always)]
    pub fn tr_1to1_gr_iter(&self) -> impl Iterator<Item = &Tr1to1Gr> {
        self.tr_1to1_gr.iter()
    }
}
#[doc = "TIMEOUT_CTL (rw) register accessor: Timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout_ctl`]
module"]
#[doc(alias = "TIMEOUT_CTL")]
pub type TimeoutCtl = crate::Reg<timeout_ctl::TimeoutCtlSpec>;
#[doc = "Timeout control"]
pub mod timeout_ctl;
#[doc = "TR_CMD (rw) register accessor: Trigger command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_cmd`]
module"]
#[doc(alias = "TR_CMD")]
pub type TrCmd = crate::Reg<tr_cmd::TrCmdSpec>;
#[doc = "Trigger command"]
pub mod tr_cmd;
#[doc = "DIV_CMD (rw) register accessor: Divider command\n\nYou can [`read`](crate::Reg::read) this register and get [`div_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_cmd`]
module"]
#[doc(alias = "DIV_CMD")]
pub type DivCmd = crate::Reg<div_cmd::DivCmdSpec>;
#[doc = "Divider command"]
pub mod div_cmd;
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
#[doc(alias = "CLOCK_CTL")]
pub type ClockCtl = crate::Reg<clock_ctl::ClockCtlSpec>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "DIV_8_CTL (rw) register accessor: Divider control (for 8.0 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_8_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_8_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_8_ctl`]
module"]
#[doc(alias = "DIV_8_CTL")]
pub type Div8Ctl = crate::Reg<div_8_ctl::Div8CtlSpec>;
#[doc = "Divider control (for 8.0 divider)"]
pub mod div_8_ctl;
#[doc = "DIV_16_CTL (rw) register accessor: Divider control (for 16.0 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_16_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_16_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_16_ctl`]
module"]
#[doc(alias = "DIV_16_CTL")]
pub type Div16Ctl = crate::Reg<div_16_ctl::Div16CtlSpec>;
#[doc = "Divider control (for 16.0 divider)"]
pub mod div_16_ctl;
#[doc = "DIV_16_5_CTL (rw) register accessor: Divider control (for 16.5 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_16_5_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_16_5_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_16_5_ctl`]
module"]
#[doc(alias = "DIV_16_5_CTL")]
pub type Div16_5Ctl = crate::Reg<div_16_5_ctl::Div16_5CtlSpec>;
#[doc = "Divider control (for 16.5 divider)"]
pub mod div_16_5_ctl;
#[doc = "DIV_24_5_CTL (rw) register accessor: Divider control (for 24.5 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_24_5_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_24_5_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div_24_5_ctl`]
module"]
#[doc(alias = "DIV_24_5_CTL")]
pub type Div24_5Ctl = crate::Reg<div_24_5_ctl::Div24_5CtlSpec>;
#[doc = "Divider control (for 24.5 divider)"]
pub mod div_24_5_ctl;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
#[doc(alias = "ECC_CTL")]
pub type EccCtl = crate::Reg<ecc_ctl::EccCtlSpec>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "Peripheral group structure"]
pub use self::gr::Gr;
#[doc = r"Cluster"]
#[doc = "Peripheral group structure"]
pub mod gr;
#[doc = "Trigger group"]
pub use self::tr_gr::TrGr;
#[doc = r"Cluster"]
#[doc = "Trigger group"]
pub mod tr_gr;
#[doc = "Trigger 1-to-1 group"]
pub use self::tr_1to1_gr::Tr1to1Gr;
#[doc = r"Cluster"]
#[doc = "Trigger 1-to-1 group"]
pub mod tr_1to1_gr;
