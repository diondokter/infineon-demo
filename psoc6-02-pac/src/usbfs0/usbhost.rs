#[repr(C)]
#[doc = "USB Host Controller"]
#[doc(alias = "USBHOST")]
pub struct Usbhost {
    host_ctl0: HostCtl0,
    _reserved1: [u8; 0x0c],
    host_ctl1: HostCtl1,
    _reserved2: [u8; 0xec],
    host_ctl2: HostCtl2,
    host_err: HostErr,
    host_status: HostStatus,
    host_fcomp: HostFcomp,
    host_rtimer: HostRtimer,
    host_addr: HostAddr,
    host_eof: HostEof,
    host_frame: HostFrame,
    host_token: HostToken,
    _reserved11: [u8; 0x02dc],
    host_ep1_ctl: HostEp1Ctl,
    host_ep1_status: HostEp1Status,
    host_ep1_rw1_dr: HostEp1Rw1Dr,
    host_ep1_rw2_dr: HostEp1Rw2Dr,
    _reserved15: [u8; 0xf0],
    host_ep2_ctl: HostEp2Ctl,
    host_ep2_status: HostEp2Status,
    host_ep2_rw1_dr: HostEp2Rw1Dr,
    host_ep2_rw2_dr: HostEp2Rw2Dr,
    _reserved19: [u8; 0x02f0],
    host_lvl1_sel: HostLvl1Sel,
    host_lvl2_sel: HostLvl2Sel,
    _reserved21: [u8; 0xf8],
    intr_usbhost_cause_hi: IntrUsbhostCauseHi,
    intr_usbhost_cause_med: IntrUsbhostCauseMed,
    intr_usbhost_cause_lo: IntrUsbhostCauseLo,
    _reserved24: [u8; 0x14],
    intr_host_ep_cause_hi: IntrHostEpCauseHi,
    intr_host_ep_cause_med: IntrHostEpCauseMed,
    intr_host_ep_cause_lo: IntrHostEpCauseLo,
    _reserved27: [u8; 0x14],
    intr_usbhost: IntrUsbhost,
    intr_usbhost_set: IntrUsbhostSet,
    intr_usbhost_mask: IntrUsbhostMask,
    intr_usbhost_masked: IntrUsbhostMasked,
    _reserved31: [u8; 0xb0],
    intr_host_ep: IntrHostEp,
    intr_host_ep_set: IntrHostEpSet,
    intr_host_ep_mask: IntrHostEpMask,
    intr_host_ep_masked: IntrHostEpMasked,
    _reserved35: [u8; 0xf0],
    host_dma_enbl: HostDmaEnbl,
    _reserved36: [u8; 0x1c],
    host_ep1_blk: HostEp1Blk,
    _reserved37: [u8; 0x0c],
    host_ep2_blk: HostEp2Blk,
}
impl Usbhost {
    #[doc = "0x00 - Host Control 0 Register."]
    #[inline(always)]
    pub const fn host_ctl0(&self) -> &HostCtl0 {
        &self.host_ctl0
    }
    #[doc = "0x10 - Host Control 1 Register."]
    #[inline(always)]
    pub const fn host_ctl1(&self) -> &HostCtl1 {
        &self.host_ctl1
    }
    #[doc = "0x100 - Host Control 2 Register."]
    #[inline(always)]
    pub const fn host_ctl2(&self) -> &HostCtl2 {
        &self.host_ctl2
    }
    #[doc = "0x104 - Host Error Status Register."]
    #[inline(always)]
    pub const fn host_err(&self) -> &HostErr {
        &self.host_err
    }
    #[doc = "0x108 - Host Status Register."]
    #[inline(always)]
    pub const fn host_status(&self) -> &HostStatus {
        &self.host_status
    }
    #[doc = "0x10c - Host SOF Interrupt Frame Compare Register"]
    #[inline(always)]
    pub const fn host_fcomp(&self) -> &HostFcomp {
        &self.host_fcomp
    }
    #[doc = "0x110 - Host Retry Timer Setup Register"]
    #[inline(always)]
    pub const fn host_rtimer(&self) -> &HostRtimer {
        &self.host_rtimer
    }
    #[doc = "0x114 - Host Address Register"]
    #[inline(always)]
    pub const fn host_addr(&self) -> &HostAddr {
        &self.host_addr
    }
    #[doc = "0x118 - Host EOF Setup Register"]
    #[inline(always)]
    pub const fn host_eof(&self) -> &HostEof {
        &self.host_eof
    }
    #[doc = "0x11c - Host Frame Setup Register"]
    #[inline(always)]
    pub const fn host_frame(&self) -> &HostFrame {
        &self.host_frame
    }
    #[doc = "0x120 - Host Token Endpoint Register"]
    #[inline(always)]
    pub const fn host_token(&self) -> &HostToken {
        &self.host_token
    }
    #[doc = "0x400 - Host Endpoint 1 Control Register"]
    #[inline(always)]
    pub const fn host_ep1_ctl(&self) -> &HostEp1Ctl {
        &self.host_ep1_ctl
    }
    #[doc = "0x404 - Host Endpoint 1 Status Register"]
    #[inline(always)]
    pub const fn host_ep1_status(&self) -> &HostEp1Status {
        &self.host_ep1_status
    }
    #[doc = "0x408 - Host Endpoint 1 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw1_dr(&self) -> &HostEp1Rw1Dr {
        &self.host_ep1_rw1_dr
    }
    #[doc = "0x40c - Host Endpoint 1 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep1_rw2_dr(&self) -> &HostEp1Rw2Dr {
        &self.host_ep1_rw2_dr
    }
    #[doc = "0x500 - Host Endpoint 2 Control Register"]
    #[inline(always)]
    pub const fn host_ep2_ctl(&self) -> &HostEp2Ctl {
        &self.host_ep2_ctl
    }
    #[doc = "0x504 - Host Endpoint 2 Status Register"]
    #[inline(always)]
    pub const fn host_ep2_status(&self) -> &HostEp2Status {
        &self.host_ep2_status
    }
    #[doc = "0x508 - Host Endpoint 2 Data 1-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw1_dr(&self) -> &HostEp2Rw1Dr {
        &self.host_ep2_rw1_dr
    }
    #[doc = "0x50c - Host Endpoint 2 Data 2-Byte Register"]
    #[inline(always)]
    pub const fn host_ep2_rw2_dr(&self) -> &HostEp2Rw2Dr {
        &self.host_ep2_rw2_dr
    }
    #[doc = "0x800 - Host Interrupt Level 1 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl1_sel(&self) -> &HostLvl1Sel {
        &self.host_lvl1_sel
    }
    #[doc = "0x804 - Host Interrupt Level 2 Selection Register"]
    #[inline(always)]
    pub const fn host_lvl2_sel(&self) -> &HostLvl2Sel {
        &self.host_lvl2_sel
    }
    #[doc = "0x900 - Interrupt USB Host Cause High Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_hi(&self) -> &IntrUsbhostCauseHi {
        &self.intr_usbhost_cause_hi
    }
    #[doc = "0x904 - Interrupt USB Host Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_med(&self) -> &IntrUsbhostCauseMed {
        &self.intr_usbhost_cause_med
    }
    #[doc = "0x908 - Interrupt USB Host Cause Low Register"]
    #[inline(always)]
    pub const fn intr_usbhost_cause_lo(&self) -> &IntrUsbhostCauseLo {
        &self.intr_usbhost_cause_lo
    }
    #[doc = "0x920 - Interrupt USB Host Endpoint Cause High Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_hi(&self) -> &IntrHostEpCauseHi {
        &self.intr_host_ep_cause_hi
    }
    #[doc = "0x924 - Interrupt USB Host Endpoint Cause Medium Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_med(&self) -> &IntrHostEpCauseMed {
        &self.intr_host_ep_cause_med
    }
    #[doc = "0x928 - Interrupt USB Host Endpoint Cause Low Register"]
    #[inline(always)]
    pub const fn intr_host_ep_cause_lo(&self) -> &IntrHostEpCauseLo {
        &self.intr_host_ep_cause_lo
    }
    #[doc = "0x940 - Interrupt USB Host Register"]
    #[inline(always)]
    pub const fn intr_usbhost(&self) -> &IntrUsbhost {
        &self.intr_usbhost
    }
    #[doc = "0x944 - Interrupt USB Host Set Register"]
    #[inline(always)]
    pub const fn intr_usbhost_set(&self) -> &IntrUsbhostSet {
        &self.intr_usbhost_set
    }
    #[doc = "0x948 - Interrupt USB Host Mask Register"]
    #[inline(always)]
    pub const fn intr_usbhost_mask(&self) -> &IntrUsbhostMask {
        &self.intr_usbhost_mask
    }
    #[doc = "0x94c - Interrupt USB Host Masked Register"]
    #[inline(always)]
    pub const fn intr_usbhost_masked(&self) -> &IntrUsbhostMasked {
        &self.intr_usbhost_masked
    }
    #[doc = "0xa00 - Interrupt USB Host Endpoint Register"]
    #[inline(always)]
    pub const fn intr_host_ep(&self) -> &IntrHostEp {
        &self.intr_host_ep
    }
    #[doc = "0xa04 - Interrupt USB Host Endpoint Set Register"]
    #[inline(always)]
    pub const fn intr_host_ep_set(&self) -> &IntrHostEpSet {
        &self.intr_host_ep_set
    }
    #[doc = "0xa08 - Interrupt USB Host Endpoint Mask Register"]
    #[inline(always)]
    pub const fn intr_host_ep_mask(&self) -> &IntrHostEpMask {
        &self.intr_host_ep_mask
    }
    #[doc = "0xa0c - Interrupt USB Host Endpoint Masked Register"]
    #[inline(always)]
    pub const fn intr_host_ep_masked(&self) -> &IntrHostEpMasked {
        &self.intr_host_ep_masked
    }
    #[doc = "0xb00 - Host DMA Enable Register"]
    #[inline(always)]
    pub const fn host_dma_enbl(&self) -> &HostDmaEnbl {
        &self.host_dma_enbl
    }
    #[doc = "0xb20 - Host Endpoint 1 Block Register"]
    #[inline(always)]
    pub const fn host_ep1_blk(&self) -> &HostEp1Blk {
        &self.host_ep1_blk
    }
    #[doc = "0xb30 - Host Endpoint 2 Block Register"]
    #[inline(always)]
    pub const fn host_ep2_blk(&self) -> &HostEp2Blk {
        &self.host_ep2_blk
    }
}
#[doc = "HOST_CTL0 (rw) register accessor: Host Control 0 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl0`]
module"]
#[doc(alias = "HOST_CTL0")]
pub type HostCtl0 = crate::Reg<host_ctl0::HostCtl0Spec>;
#[doc = "Host Control 0 Register."]
pub mod host_ctl0;
#[doc = "HOST_CTL1 (rw) register accessor: Host Control 1 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl1`]
module"]
#[doc(alias = "HOST_CTL1")]
pub type HostCtl1 = crate::Reg<host_ctl1::HostCtl1Spec>;
#[doc = "Host Control 1 Register."]
pub mod host_ctl1;
#[doc = "HOST_CTL2 (rw) register accessor: Host Control 2 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctl2`]
module"]
#[doc(alias = "HOST_CTL2")]
pub type HostCtl2 = crate::Reg<host_ctl2::HostCtl2Spec>;
#[doc = "Host Control 2 Register."]
pub mod host_ctl2;
#[doc = "HOST_ERR (rw) register accessor: Host Error Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_err`]
module"]
#[doc(alias = "HOST_ERR")]
pub type HostErr = crate::Reg<host_err::HostErrSpec>;
#[doc = "Host Error Status Register."]
pub mod host_err;
#[doc = "HOST_STATUS (rw) register accessor: Host Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_status`]
module"]
#[doc(alias = "HOST_STATUS")]
pub type HostStatus = crate::Reg<host_status::HostStatusSpec>;
#[doc = "Host Status Register."]
pub mod host_status;
#[doc = "HOST_FCOMP (rw) register accessor: Host SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_fcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_fcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_fcomp`]
module"]
#[doc(alias = "HOST_FCOMP")]
pub type HostFcomp = crate::Reg<host_fcomp::HostFcompSpec>;
#[doc = "Host SOF Interrupt Frame Compare Register"]
pub mod host_fcomp;
#[doc = "HOST_RTIMER (rw) register accessor: Host Retry Timer Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_rtimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_rtimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_rtimer`]
module"]
#[doc(alias = "HOST_RTIMER")]
pub type HostRtimer = crate::Reg<host_rtimer::HostRtimerSpec>;
#[doc = "Host Retry Timer Setup Register"]
pub mod host_rtimer;
#[doc = "HOST_ADDR (rw) register accessor: Host Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_addr`]
module"]
#[doc(alias = "HOST_ADDR")]
pub type HostAddr = crate::Reg<host_addr::HostAddrSpec>;
#[doc = "Host Address Register"]
pub mod host_addr;
#[doc = "HOST_EOF (rw) register accessor: Host EOF Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_eof::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_eof::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_eof`]
module"]
#[doc(alias = "HOST_EOF")]
pub type HostEof = crate::Reg<host_eof::HostEofSpec>;
#[doc = "Host EOF Setup Register"]
pub mod host_eof;
#[doc = "HOST_FRAME (rw) register accessor: Host Frame Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_frame`]
module"]
#[doc(alias = "HOST_FRAME")]
pub type HostFrame = crate::Reg<host_frame::HostFrameSpec>;
#[doc = "Host Frame Setup Register"]
pub mod host_frame;
#[doc = "HOST_TOKEN (rw) register accessor: Host Token Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_token::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_token::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_token`]
module"]
#[doc(alias = "HOST_TOKEN")]
pub type HostToken = crate::Reg<host_token::HostTokenSpec>;
#[doc = "Host Token Endpoint Register"]
pub mod host_token;
#[doc = "HOST_EP1_CTL (rw) register accessor: Host Endpoint 1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_ctl`]
module"]
#[doc(alias = "HOST_EP1_CTL")]
pub type HostEp1Ctl = crate::Reg<host_ep1_ctl::HostEp1CtlSpec>;
#[doc = "Host Endpoint 1 Control Register"]
pub mod host_ep1_ctl;
#[doc = "HOST_EP1_STATUS (r) register accessor: Host Endpoint 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_status`]
module"]
#[doc(alias = "HOST_EP1_STATUS")]
pub type HostEp1Status = crate::Reg<host_ep1_status::HostEp1StatusSpec>;
#[doc = "Host Endpoint 1 Status Register"]
pub mod host_ep1_status;
#[doc = "HOST_EP1_RW1_DR (rw) register accessor: Host Endpoint 1 Data 1-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_rw1_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_rw1_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_rw1_dr`]
module"]
#[doc(alias = "HOST_EP1_RW1_DR")]
pub type HostEp1Rw1Dr = crate::Reg<host_ep1_rw1_dr::HostEp1Rw1DrSpec>;
#[doc = "Host Endpoint 1 Data 1-Byte Register"]
pub mod host_ep1_rw1_dr;
#[doc = "HOST_EP1_RW2_DR (rw) register accessor: Host Endpoint 1 Data 2-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_rw2_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_rw2_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_rw2_dr`]
module"]
#[doc(alias = "HOST_EP1_RW2_DR")]
pub type HostEp1Rw2Dr = crate::Reg<host_ep1_rw2_dr::HostEp1Rw2DrSpec>;
#[doc = "Host Endpoint 1 Data 2-Byte Register"]
pub mod host_ep1_rw2_dr;
#[doc = "HOST_EP2_CTL (rw) register accessor: Host Endpoint 2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep2_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_ctl`]
module"]
#[doc(alias = "HOST_EP2_CTL")]
pub type HostEp2Ctl = crate::Reg<host_ep2_ctl::HostEp2CtlSpec>;
#[doc = "Host Endpoint 2 Control Register"]
pub mod host_ep2_ctl;
#[doc = "HOST_EP2_STATUS (r) register accessor: Host Endpoint 2 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_status`]
module"]
#[doc(alias = "HOST_EP2_STATUS")]
pub type HostEp2Status = crate::Reg<host_ep2_status::HostEp2StatusSpec>;
#[doc = "Host Endpoint 2 Status Register"]
pub mod host_ep2_status;
#[doc = "HOST_EP2_RW1_DR (rw) register accessor: Host Endpoint 2 Data 1-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_rw1_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep2_rw1_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_rw1_dr`]
module"]
#[doc(alias = "HOST_EP2_RW1_DR")]
pub type HostEp2Rw1Dr = crate::Reg<host_ep2_rw1_dr::HostEp2Rw1DrSpec>;
#[doc = "Host Endpoint 2 Data 1-Byte Register"]
pub mod host_ep2_rw1_dr;
#[doc = "HOST_EP2_RW2_DR (rw) register accessor: Host Endpoint 2 Data 2-Byte Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_rw2_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep2_rw2_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_rw2_dr`]
module"]
#[doc(alias = "HOST_EP2_RW2_DR")]
pub type HostEp2Rw2Dr = crate::Reg<host_ep2_rw2_dr::HostEp2Rw2DrSpec>;
#[doc = "Host Endpoint 2 Data 2-Byte Register"]
pub mod host_ep2_rw2_dr;
#[doc = "HOST_LVL1_SEL (rw) register accessor: Host Interrupt Level 1 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_lvl1_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_lvl1_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_lvl1_sel`]
module"]
#[doc(alias = "HOST_LVL1_SEL")]
pub type HostLvl1Sel = crate::Reg<host_lvl1_sel::HostLvl1SelSpec>;
#[doc = "Host Interrupt Level 1 Selection Register"]
pub mod host_lvl1_sel;
#[doc = "HOST_LVL2_SEL (rw) register accessor: Host Interrupt Level 2 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_lvl2_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_lvl2_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_lvl2_sel`]
module"]
#[doc(alias = "HOST_LVL2_SEL")]
pub type HostLvl2Sel = crate::Reg<host_lvl2_sel::HostLvl2SelSpec>;
#[doc = "Host Interrupt Level 2 Selection Register"]
pub mod host_lvl2_sel;
#[doc = "INTR_USBHOST_CAUSE_HI (r) register accessor: Interrupt USB Host Cause High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_cause_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_hi`]
module"]
#[doc(alias = "INTR_USBHOST_CAUSE_HI")]
pub type IntrUsbhostCauseHi = crate::Reg<intr_usbhost_cause_hi::IntrUsbhostCauseHiSpec>;
#[doc = "Interrupt USB Host Cause High Register"]
pub mod intr_usbhost_cause_hi;
#[doc = "INTR_USBHOST_CAUSE_MED (r) register accessor: Interrupt USB Host Cause Medium Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_cause_med::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_med`]
module"]
#[doc(alias = "INTR_USBHOST_CAUSE_MED")]
pub type IntrUsbhostCauseMed = crate::Reg<intr_usbhost_cause_med::IntrUsbhostCauseMedSpec>;
#[doc = "Interrupt USB Host Cause Medium Register"]
pub mod intr_usbhost_cause_med;
#[doc = "INTR_USBHOST_CAUSE_LO (r) register accessor: Interrupt USB Host Cause Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_cause_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_cause_lo`]
module"]
#[doc(alias = "INTR_USBHOST_CAUSE_LO")]
pub type IntrUsbhostCauseLo = crate::Reg<intr_usbhost_cause_lo::IntrUsbhostCauseLoSpec>;
#[doc = "Interrupt USB Host Cause Low Register"]
pub mod intr_usbhost_cause_lo;
#[doc = "INTR_HOST_EP_CAUSE_HI (r) register accessor: Interrupt USB Host Endpoint Cause High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_cause_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_hi`]
module"]
#[doc(alias = "INTR_HOST_EP_CAUSE_HI")]
pub type IntrHostEpCauseHi = crate::Reg<intr_host_ep_cause_hi::IntrHostEpCauseHiSpec>;
#[doc = "Interrupt USB Host Endpoint Cause High Register"]
pub mod intr_host_ep_cause_hi;
#[doc = "INTR_HOST_EP_CAUSE_MED (r) register accessor: Interrupt USB Host Endpoint Cause Medium Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_cause_med::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_med`]
module"]
#[doc(alias = "INTR_HOST_EP_CAUSE_MED")]
pub type IntrHostEpCauseMed = crate::Reg<intr_host_ep_cause_med::IntrHostEpCauseMedSpec>;
#[doc = "Interrupt USB Host Endpoint Cause Medium Register"]
pub mod intr_host_ep_cause_med;
#[doc = "INTR_HOST_EP_CAUSE_LO (r) register accessor: Interrupt USB Host Endpoint Cause Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_cause_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_cause_lo`]
module"]
#[doc(alias = "INTR_HOST_EP_CAUSE_LO")]
pub type IntrHostEpCauseLo = crate::Reg<intr_host_ep_cause_lo::IntrHostEpCauseLoSpec>;
#[doc = "Interrupt USB Host Endpoint Cause Low Register"]
pub mod intr_host_ep_cause_lo;
#[doc = "INTR_USBHOST (rw) register accessor: Interrupt USB Host Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_usbhost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost`]
module"]
#[doc(alias = "INTR_USBHOST")]
pub type IntrUsbhost = crate::Reg<intr_usbhost::IntrUsbhostSpec>;
#[doc = "Interrupt USB Host Register"]
pub mod intr_usbhost;
#[doc = "INTR_USBHOST_SET (rw) register accessor: Interrupt USB Host Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_usbhost_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_set`]
module"]
#[doc(alias = "INTR_USBHOST_SET")]
pub type IntrUsbhostSet = crate::Reg<intr_usbhost_set::IntrUsbhostSetSpec>;
#[doc = "Interrupt USB Host Set Register"]
pub mod intr_usbhost_set;
#[doc = "INTR_USBHOST_MASK (rw) register accessor: Interrupt USB Host Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_usbhost_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_mask`]
module"]
#[doc(alias = "INTR_USBHOST_MASK")]
pub type IntrUsbhostMask = crate::Reg<intr_usbhost_mask::IntrUsbhostMaskSpec>;
#[doc = "Interrupt USB Host Mask Register"]
pub mod intr_usbhost_mask;
#[doc = "INTR_USBHOST_MASKED (r) register accessor: Interrupt USB Host Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_usbhost_masked`]
module"]
#[doc(alias = "INTR_USBHOST_MASKED")]
pub type IntrUsbhostMasked = crate::Reg<intr_usbhost_masked::IntrUsbhostMaskedSpec>;
#[doc = "Interrupt USB Host Masked Register"]
pub mod intr_usbhost_masked;
#[doc = "INTR_HOST_EP (rw) register accessor: Interrupt USB Host Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep`]
module"]
#[doc(alias = "INTR_HOST_EP")]
pub type IntrHostEp = crate::Reg<intr_host_ep::IntrHostEpSpec>;
#[doc = "Interrupt USB Host Endpoint Register"]
pub mod intr_host_ep;
#[doc = "INTR_HOST_EP_SET (rw) register accessor: Interrupt USB Host Endpoint Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_set`]
module"]
#[doc(alias = "INTR_HOST_EP_SET")]
pub type IntrHostEpSet = crate::Reg<intr_host_ep_set::IntrHostEpSetSpec>;
#[doc = "Interrupt USB Host Endpoint Set Register"]
pub mod intr_host_ep_set;
#[doc = "INTR_HOST_EP_MASK (rw) register accessor: Interrupt USB Host Endpoint Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_mask`]
module"]
#[doc(alias = "INTR_HOST_EP_MASK")]
pub type IntrHostEpMask = crate::Reg<intr_host_ep_mask::IntrHostEpMaskSpec>;
#[doc = "Interrupt USB Host Endpoint Mask Register"]
pub mod intr_host_ep_mask;
#[doc = "INTR_HOST_EP_MASKED (r) register accessor: Interrupt USB Host Endpoint Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_host_ep_masked`]
module"]
#[doc(alias = "INTR_HOST_EP_MASKED")]
pub type IntrHostEpMasked = crate::Reg<intr_host_ep_masked::IntrHostEpMaskedSpec>;
#[doc = "Interrupt USB Host Endpoint Masked Register"]
pub mod intr_host_ep_masked;
#[doc = "HOST_DMA_ENBL (rw) register accessor: Host DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_dma_enbl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_dma_enbl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_dma_enbl`]
module"]
#[doc(alias = "HOST_DMA_ENBL")]
pub type HostDmaEnbl = crate::Reg<host_dma_enbl::HostDmaEnblSpec>;
#[doc = "Host DMA Enable Register"]
pub mod host_dma_enbl;
#[doc = "HOST_EP1_BLK (rw) register accessor: Host Endpoint 1 Block Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_blk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_blk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep1_blk`]
module"]
#[doc(alias = "HOST_EP1_BLK")]
pub type HostEp1Blk = crate::Reg<host_ep1_blk::HostEp1BlkSpec>;
#[doc = "Host Endpoint 1 Block Register"]
pub mod host_ep1_blk;
#[doc = "HOST_EP2_BLK (rw) register accessor: Host Endpoint 2 Block Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep2_blk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep2_blk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ep2_blk`]
module"]
#[doc(alias = "HOST_EP2_BLK")]
pub type HostEp2Blk = crate::Reg<host_ep2_blk::HostEp2BlkSpec>;
#[doc = "Host Endpoint 2 Block Register"]
pub mod host_ep2_blk;
