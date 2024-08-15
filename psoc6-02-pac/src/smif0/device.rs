#[repr(C)]
#[doc = "Device (only used in XIP mode)"]
#[doc(alias = "DEVICE")]
pub struct Device {
    ctl: Ctl,
    _reserved1: [u8; 0x04],
    addr: Addr,
    mask: Mask,
    _reserved3: [u8; 0x10],
    addr_ctl: AddrCtl,
    _reserved4: [u8; 0x1c],
    rd_cmd_ctl: RdCmdCtl,
    rd_addr_ctl: RdAddrCtl,
    rd_mode_ctl: RdModeCtl,
    rd_dummy_ctl: RdDummyCtl,
    rd_data_ctl: RdDataCtl,
    _reserved9: [u8; 0x0c],
    wr_cmd_ctl: WrCmdCtl,
    wr_addr_ctl: WrAddrCtl,
    wr_mode_ctl: WrModeCtl,
    wr_dummy_ctl: WrDummyCtl,
    wr_data_ctl: WrDataCtl,
}
impl Device {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x08 - Device region base address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x0c - Device region mask"]
    #[inline(always)]
    pub const fn mask(&self) -> &Mask {
        &self.mask
    }
    #[doc = "0x20 - Address control"]
    #[inline(always)]
    pub const fn addr_ctl(&self) -> &AddrCtl {
        &self.addr_ctl
    }
    #[doc = "0x40 - Read command control"]
    #[inline(always)]
    pub const fn rd_cmd_ctl(&self) -> &RdCmdCtl {
        &self.rd_cmd_ctl
    }
    #[doc = "0x44 - Read address control"]
    #[inline(always)]
    pub const fn rd_addr_ctl(&self) -> &RdAddrCtl {
        &self.rd_addr_ctl
    }
    #[doc = "0x48 - Read mode control"]
    #[inline(always)]
    pub const fn rd_mode_ctl(&self) -> &RdModeCtl {
        &self.rd_mode_ctl
    }
    #[doc = "0x4c - Read dummy control"]
    #[inline(always)]
    pub const fn rd_dummy_ctl(&self) -> &RdDummyCtl {
        &self.rd_dummy_ctl
    }
    #[doc = "0x50 - Read data control"]
    #[inline(always)]
    pub const fn rd_data_ctl(&self) -> &RdDataCtl {
        &self.rd_data_ctl
    }
    #[doc = "0x60 - Write command control"]
    #[inline(always)]
    pub const fn wr_cmd_ctl(&self) -> &WrCmdCtl {
        &self.wr_cmd_ctl
    }
    #[doc = "0x64 - Write address control"]
    #[inline(always)]
    pub const fn wr_addr_ctl(&self) -> &WrAddrCtl {
        &self.wr_addr_ctl
    }
    #[doc = "0x68 - Write mode control"]
    #[inline(always)]
    pub const fn wr_mode_ctl(&self) -> &WrModeCtl {
        &self.wr_mode_ctl
    }
    #[doc = "0x6c - Write dummy control"]
    #[inline(always)]
    pub const fn wr_dummy_ctl(&self) -> &WrDummyCtl {
        &self.wr_dummy_ctl
    }
    #[doc = "0x70 - Write data control"]
    #[inline(always)]
    pub const fn wr_data_ctl(&self) -> &WrDataCtl {
        &self.wr_data_ctl
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "ADDR (rw) register accessor: Device region base address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Device region base address"]
pub mod addr;
#[doc = "MASK (rw) register accessor: Device region mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "Device region mask"]
pub mod mask;
#[doc = "ADDR_CTL (rw) register accessor: Address control\n\nYou can [`read`](crate::Reg::read) this register and get [`addr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr_ctl`]
module"]
#[doc(alias = "ADDR_CTL")]
pub type AddrCtl = crate::Reg<addr_ctl::AddrCtlSpec>;
#[doc = "Address control"]
pub mod addr_ctl;
#[doc = "RD_CMD_CTL (rw) register accessor: Read command control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_cmd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_cmd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_cmd_ctl`]
module"]
#[doc(alias = "RD_CMD_CTL")]
pub type RdCmdCtl = crate::Reg<rd_cmd_ctl::RdCmdCtlSpec>;
#[doc = "Read command control"]
pub mod rd_cmd_ctl;
#[doc = "RD_ADDR_CTL (rw) register accessor: Read address control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_addr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_addr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_addr_ctl`]
module"]
#[doc(alias = "RD_ADDR_CTL")]
pub type RdAddrCtl = crate::Reg<rd_addr_ctl::RdAddrCtlSpec>;
#[doc = "Read address control"]
pub mod rd_addr_ctl;
#[doc = "RD_MODE_CTL (rw) register accessor: Read mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mode_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_mode_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mode_ctl`]
module"]
#[doc(alias = "RD_MODE_CTL")]
pub type RdModeCtl = crate::Reg<rd_mode_ctl::RdModeCtlSpec>;
#[doc = "Read mode control"]
pub mod rd_mode_ctl;
#[doc = "RD_DUMMY_CTL (rw) register accessor: Read dummy control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_dummy_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_dummy_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_dummy_ctl`]
module"]
#[doc(alias = "RD_DUMMY_CTL")]
pub type RdDummyCtl = crate::Reg<rd_dummy_ctl::RdDummyCtlSpec>;
#[doc = "Read dummy control"]
pub mod rd_dummy_ctl;
#[doc = "RD_DATA_CTL (rw) register accessor: Read data control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_data_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_data_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_data_ctl`]
module"]
#[doc(alias = "RD_DATA_CTL")]
pub type RdDataCtl = crate::Reg<rd_data_ctl::RdDataCtlSpec>;
#[doc = "Read data control"]
pub mod rd_data_ctl;
#[doc = "WR_CMD_CTL (rw) register accessor: Write command control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_cmd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_cmd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_cmd_ctl`]
module"]
#[doc(alias = "WR_CMD_CTL")]
pub type WrCmdCtl = crate::Reg<wr_cmd_ctl::WrCmdCtlSpec>;
#[doc = "Write command control"]
pub mod wr_cmd_ctl;
#[doc = "WR_ADDR_CTL (rw) register accessor: Write address control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_addr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_addr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_addr_ctl`]
module"]
#[doc(alias = "WR_ADDR_CTL")]
pub type WrAddrCtl = crate::Reg<wr_addr_ctl::WrAddrCtlSpec>;
#[doc = "Write address control"]
pub mod wr_addr_ctl;
#[doc = "WR_MODE_CTL (rw) register accessor: Write mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_mode_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_mode_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_mode_ctl`]
module"]
#[doc(alias = "WR_MODE_CTL")]
pub type WrModeCtl = crate::Reg<wr_mode_ctl::WrModeCtlSpec>;
#[doc = "Write mode control"]
pub mod wr_mode_ctl;
#[doc = "WR_DUMMY_CTL (rw) register accessor: Write dummy control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_dummy_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_dummy_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_dummy_ctl`]
module"]
#[doc(alias = "WR_DUMMY_CTL")]
pub type WrDummyCtl = crate::Reg<wr_dummy_ctl::WrDummyCtlSpec>;
#[doc = "Write dummy control"]
pub mod wr_dummy_ctl;
#[doc = "WR_DATA_CTL (rw) register accessor: Write data control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_data_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_data_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_data_ctl`]
module"]
#[doc(alias = "WR_DATA_CTL")]
pub type WrDataCtl = crate::Reg<wr_data_ctl::WrDataCtlSpec>;
#[doc = "Write data control"]
pub mod wr_data_ctl;
