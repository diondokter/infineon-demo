#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    _reserved1: [u8; 0x0c],
    clock_ctl: ClockCtl,
    mode_ctl: ModeCtl,
    data_ctl: DataCtl,
    _reserved4: [u8; 0x04],
    cmd: Cmd,
    _reserved5: [u8; 0x1c],
    tr_ctl: TrCtl,
    _reserved6: [u8; 0x02bc],
    rx_fifo_ctl: RxFifoCtl,
    rx_fifo_status: RxFifoStatus,
    rx_fifo_rd: RxFifoRd,
    rx_fifo_rd_silent: RxFifoRdSilent,
    _reserved10: [u8; 0x0bf0],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - Clock control"]
    #[inline(always)]
    pub const fn clock_ctl(&self) -> &ClockCtl {
        &self.clock_ctl
    }
    #[doc = "0x14 - Mode control"]
    #[inline(always)]
    pub const fn mode_ctl(&self) -> &ModeCtl {
        &self.mode_ctl
    }
    #[doc = "0x18 - Data control"]
    #[inline(always)]
    pub const fn data_ctl(&self) -> &DataCtl {
        &self.data_ctl
    }
    #[doc = "0x20 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x40 - Trigger control"]
    #[inline(always)]
    pub const fn tr_ctl(&self) -> &TrCtl {
        &self.tr_ctl
    }
    #[doc = "0x300 - RX FIFO control"]
    #[inline(always)]
    pub const fn rx_fifo_ctl(&self) -> &RxFifoCtl {
        &self.rx_fifo_ctl
    }
    #[doc = "0x304 - RX FIFO status"]
    #[inline(always)]
    pub const fn rx_fifo_status(&self) -> &RxFifoStatus {
        &self.rx_fifo_status
    }
    #[doc = "0x308 - RX FIFO read"]
    #[inline(always)]
    pub const fn rx_fifo_rd(&self) -> &RxFifoRd {
        &self.rx_fifo_rd
    }
    #[doc = "0x30c - RX FIFO silent read"]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(&self) -> &RxFifoRdSilent {
        &self.rx_fifo_rd_silent
    }
    #[doc = "0xf00 - Interrupt register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0xf04 - Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0xf08 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0xf0c - Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CLOCK_CTL (rw) register accessor: Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_ctl`]
module"]
#[doc(alias = "CLOCK_CTL")]
pub type ClockCtl = crate::Reg<clock_ctl::ClockCtlSpec>;
#[doc = "Clock control"]
pub mod clock_ctl;
#[doc = "MODE_CTL (rw) register accessor: Mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_ctl`]
module"]
#[doc(alias = "MODE_CTL")]
pub type ModeCtl = crate::Reg<mode_ctl::ModeCtlSpec>;
#[doc = "Mode control"]
pub mod mode_ctl;
#[doc = "DATA_CTL (rw) register accessor: Data control\n\nYou can [`read`](crate::Reg::read) this register and get [`data_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_ctl`]
module"]
#[doc(alias = "DATA_CTL")]
pub type DataCtl = crate::Reg<data_ctl::DataCtlSpec>;
#[doc = "Data control"]
pub mod data_ctl;
#[doc = "CMD (rw) register accessor: Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TR_CTL (rw) register accessor: Trigger control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr_ctl`]
module"]
#[doc(alias = "TR_CTL")]
pub type TrCtl = crate::Reg<tr_ctl::TrCtlSpec>;
#[doc = "Trigger control"]
pub mod tr_ctl;
#[doc = "RX_FIFO_CTL (rw) register accessor: RX FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_ctl`]
module"]
#[doc(alias = "RX_FIFO_CTL")]
pub type RxFifoCtl = crate::Reg<rx_fifo_ctl::RxFifoCtlSpec>;
#[doc = "RX FIFO control"]
pub mod rx_fifo_ctl;
#[doc = "RX_FIFO_STATUS (r) register accessor: RX FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_status`]
module"]
#[doc(alias = "RX_FIFO_STATUS")]
pub type RxFifoStatus = crate::Reg<rx_fifo_status::RxFifoStatusSpec>;
#[doc = "RX FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_FIFO_RD (r) register accessor: RX FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd`]
module"]
#[doc(alias = "RX_FIFO_RD")]
pub type RxFifoRd = crate::Reg<rx_fifo_rd::RxFifoRdSpec>;
#[doc = "RX FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: RX FIFO silent read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd_silent::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd_silent`]
module"]
#[doc(alias = "RX_FIFO_RD_SILENT")]
pub type RxFifoRdSilent = crate::Reg<rx_fifo_rd_silent::RxFifoRdSilentSpec>;
#[doc = "RX FIFO silent read"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
