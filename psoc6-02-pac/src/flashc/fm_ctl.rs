#[repr(C)]
#[doc = "Flash Macro Registers"]
#[doc(alias = "FM_CTL")]
pub struct FmCtl {
    fm_ctl: FmCtlReg,
    status: Status,
    fm_addr: FmAddr,
    bookmark: Bookmark,
    geometry: Geometry,
    geometry_supervisory: GeometrySupervisory,
    ana_ctl0: AnaCtl0,
    ana_ctl1: AnaCtl1,
    _reserved8: [u8; 0x08],
    wait_ctl: WaitCtl,
    _reserved9: [u8; 0x08],
    timer_clk_ctl: TimerClkCtl,
    timer_ctl: TimerCtl,
    aclk_ctl: AclkCtl,
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    cal_ctl0: CalCtl0,
    cal_ctl1: CalCtl1,
    cal_ctl2: CalCtl2,
    cal_ctl3: CalCtl3,
    cal_ctl4: CalCtl4,
    cal_ctl5: CalCtl5,
    cal_ctl6: CalCtl6,
    cal_ctl7: CalCtl7,
    _reserved24: [u8; 0x10],
    red_ctl01: RedCtl01,
    red_ctl23: RedCtl23,
    red_ctl45: RedCtl45,
    red_ctl67: RedCtl67,
    red_ctl_sm01: RedCtlSm01,
    _reserved29: [u8; 0x04],
    rgrant_delay_prg: RgrantDelayPrg,
    _reserved30: [u8; 0x04],
    pw_seq12: PwSeq12,
    pw_seq23: PwSeq23,
    rgrant_scale_ers: RgrantScaleErs,
    rgrant_delay_ers: RgrantDelayErs,
    _reserved34: [u8; 0x074c],
    fm_pl_wrdata_all: FmPlWrdataAll,
    fm_pl_data: [FmPlData; 256],
    fm_mem_data: [FmMemData; 256],
}
impl FmCtl {
    #[doc = "0x00 - Flash macro control"]
    #[inline(always)]
    pub const fn fm_ctl(&self) -> &FmCtlReg {
        &self.fm_ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Flash macro address"]
    #[inline(always)]
    pub const fn fm_addr(&self) -> &FmAddr {
        &self.fm_addr
    }
    #[doc = "0x0c - Bookmark register - keeps the current FW HV seq"]
    #[inline(always)]
    pub const fn bookmark(&self) -> &Bookmark {
        &self.bookmark
    }
    #[doc = "0x10 - Regular flash geometry"]
    #[inline(always)]
    pub const fn geometry(&self) -> &Geometry {
        &self.geometry
    }
    #[doc = "0x14 - Supervisory flash geometry"]
    #[inline(always)]
    pub const fn geometry_supervisory(&self) -> &GeometrySupervisory {
        &self.geometry_supervisory
    }
    #[doc = "0x18 - Analog control 0"]
    #[inline(always)]
    pub const fn ana_ctl0(&self) -> &AnaCtl0 {
        &self.ana_ctl0
    }
    #[doc = "0x1c - Analog control 1"]
    #[inline(always)]
    pub const fn ana_ctl1(&self) -> &AnaCtl1 {
        &self.ana_ctl1
    }
    #[doc = "0x28 - Wait State control"]
    #[inline(always)]
    pub const fn wait_ctl(&self) -> &WaitCtl {
        &self.wait_ctl
    }
    #[doc = "0x34 - Timer prescaler (clk_t to timer clock frequency divider)"]
    #[inline(always)]
    pub const fn timer_clk_ctl(&self) -> &TimerClkCtl {
        &self.timer_clk_ctl
    }
    #[doc = "0x38 - Timer control"]
    #[inline(always)]
    pub const fn timer_ctl(&self) -> &TimerCtl {
        &self.timer_ctl
    }
    #[doc = "0x3c - MPCON clock"]
    #[inline(always)]
    pub const fn aclk_ctl(&self) -> &AclkCtl {
        &self.aclk_ctl
    }
    #[doc = "0x40 - Interrupt"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x44 - Interrupt set"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x48 - Interrupt mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x4c - Interrupt masked"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x50 - Cal control BG LO trim bits"]
    #[inline(always)]
    pub const fn cal_ctl0(&self) -> &CalCtl0 {
        &self.cal_ctl0
    }
    #[doc = "0x54 - Cal control BG HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl1(&self) -> &CalCtl1 {
        &self.cal_ctl1
    }
    #[doc = "0x58 - Cal control BG LO&amp;HI trim bits"]
    #[inline(always)]
    pub const fn cal_ctl2(&self) -> &CalCtl2 {
        &self.cal_ctl2
    }
    #[doc = "0x5c - Cal control osc trim bits, idac, sdac, itim"]
    #[inline(always)]
    pub const fn cal_ctl3(&self) -> &CalCtl3 {
        &self.cal_ctl3
    }
    #[doc = "0x60 - Cal Control Vlim, SA, fdiv, reg_act"]
    #[inline(always)]
    pub const fn cal_ctl4(&self) -> &CalCtl4 {
        &self.cal_ctl4
    }
    #[doc = "0x64 - Cal control"]
    #[inline(always)]
    pub const fn cal_ctl5(&self) -> &CalCtl5 {
        &self.cal_ctl5
    }
    #[doc = "0x68 - SA trim LP/ULP"]
    #[inline(always)]
    pub const fn cal_ctl6(&self) -> &CalCtl6 {
        &self.cal_ctl6
    }
    #[doc = "0x6c - Cal control"]
    #[inline(always)]
    pub const fn cal_ctl7(&self) -> &CalCtl7 {
        &self.cal_ctl7
    }
    #[doc = "0x80 - Redundancy Control normal sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl01(&self) -> &RedCtl01 {
        &self.red_ctl01
    }
    #[doc = "0x84 - Redundancy Control normal sectors 2,3"]
    #[inline(always)]
    pub const fn red_ctl23(&self) -> &RedCtl23 {
        &self.red_ctl23
    }
    #[doc = "0x88 - Redundancy Control normal sectors 4,5"]
    #[inline(always)]
    pub const fn red_ctl45(&self) -> &RedCtl45 {
        &self.red_ctl45
    }
    #[doc = "0x8c - Redundancy Control normal sectors 6,7"]
    #[inline(always)]
    pub const fn red_ctl67(&self) -> &RedCtl67 {
        &self.red_ctl67
    }
    #[doc = "0x90 - Redundancy Control special sectors 0,1"]
    #[inline(always)]
    pub const fn red_ctl_sm01(&self) -> &RedCtlSm01 {
        &self.red_ctl_sm01
    }
    #[doc = "0x98 - R-grant delay for program"]
    #[inline(always)]
    pub const fn rgrant_delay_prg(&self) -> &RgrantDelayPrg {
        &self.rgrant_delay_prg
    }
    #[doc = "0xa0 - HV Pulse Delay for seq 1&amp;2 pre"]
    #[inline(always)]
    pub const fn pw_seq12(&self) -> &PwSeq12 {
        &self.pw_seq12
    }
    #[doc = "0xa4 - HV Pulse Delay for seq2 post &amp; seq3"]
    #[inline(always)]
    pub const fn pw_seq23(&self) -> &PwSeq23 {
        &self.pw_seq23
    }
    #[doc = "0xa8 - R-grant delay scale for erase"]
    #[inline(always)]
    pub const fn rgrant_scale_ers(&self) -> &RgrantScaleErs {
        &self.rgrant_scale_ers
    }
    #[doc = "0xac - R-grant delay for erase"]
    #[inline(always)]
    pub const fn rgrant_delay_ers(&self) -> &RgrantDelayErs {
        &self.rgrant_delay_ers
    }
    #[doc = "0x7fc - Flash macro write page latches all"]
    #[inline(always)]
    pub const fn fm_pl_wrdata_all(&self) -> &FmPlWrdataAll {
        &self.fm_pl_wrdata_all
    }
    #[doc = "0x800..0xc00 - Flash macro Page Latches data"]
    #[inline(always)]
    pub const fn fm_pl_data(&self, n: usize) -> &FmPlData {
        &self.fm_pl_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0xc00 - Flash macro Page Latches data"]
    #[inline(always)]
    pub fn fm_pl_data_iter(&self) -> impl Iterator<Item = &FmPlData> {
        self.fm_pl_data.iter()
    }
    #[doc = "0xc00..0x1000 - Flash macro memory sense amplifier and column decoder data"]
    #[inline(always)]
    pub const fn fm_mem_data(&self, n: usize) -> &FmMemData {
        &self.fm_mem_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc00..0x1000 - Flash macro memory sense amplifier and column decoder data"]
    #[inline(always)]
    pub fn fm_mem_data_iter(&self) -> impl Iterator<Item = &FmMemData> {
        self.fm_mem_data.iter()
    }
}
#[doc = "FM_CTL (rw) register accessor: Flash macro control\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_ctl`]
module"]
#[doc(alias = "FM_CTL")]
pub type FmCtlReg = crate::Reg<fm_ctl::FmCtlSpec>;
#[doc = "Flash macro control"]
pub mod fm_ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "FM_ADDR (rw) register accessor: Flash macro address\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_addr`]
module"]
#[doc(alias = "FM_ADDR")]
pub type FmAddr = crate::Reg<fm_addr::FmAddrSpec>;
#[doc = "Flash macro address"]
pub mod fm_addr;
#[doc = "BOOKMARK (rw) register accessor: Bookmark register - keeps the current FW HV seq\n\nYou can [`read`](crate::Reg::read) this register and get [`bookmark::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bookmark::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bookmark`]
module"]
#[doc(alias = "BOOKMARK")]
pub type Bookmark = crate::Reg<bookmark::BookmarkSpec>;
#[doc = "Bookmark register - keeps the current FW HV seq"]
pub mod bookmark;
#[doc = "GEOMETRY (r) register accessor: Regular flash geometry\n\nYou can [`read`](crate::Reg::read) this register and get [`geometry::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@geometry`]
module"]
#[doc(alias = "GEOMETRY")]
pub type Geometry = crate::Reg<geometry::GeometrySpec>;
#[doc = "Regular flash geometry"]
pub mod geometry;
#[doc = "GEOMETRY_SUPERVISORY (r) register accessor: Supervisory flash geometry\n\nYou can [`read`](crate::Reg::read) this register and get [`geometry_supervisory::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@geometry_supervisory`]
module"]
#[doc(alias = "GEOMETRY_SUPERVISORY")]
pub type GeometrySupervisory = crate::Reg<geometry_supervisory::GeometrySupervisorySpec>;
#[doc = "Supervisory flash geometry"]
pub mod geometry_supervisory;
#[doc = "ANA_CTL0 (rw) register accessor: Analog control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_ctl0`]
module"]
#[doc(alias = "ANA_CTL0")]
pub type AnaCtl0 = crate::Reg<ana_ctl0::AnaCtl0Spec>;
#[doc = "Analog control 0"]
pub mod ana_ctl0;
#[doc = "ANA_CTL1 (rw) register accessor: Analog control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ana_ctl1`]
module"]
#[doc(alias = "ANA_CTL1")]
pub type AnaCtl1 = crate::Reg<ana_ctl1::AnaCtl1Spec>;
#[doc = "Analog control 1"]
pub mod ana_ctl1;
#[doc = "WAIT_CTL (rw) register accessor: Wait State control\n\nYou can [`read`](crate::Reg::read) this register and get [`wait_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wait_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wait_ctl`]
module"]
#[doc(alias = "WAIT_CTL")]
pub type WaitCtl = crate::Reg<wait_ctl::WaitCtlSpec>;
#[doc = "Wait State control"]
pub mod wait_ctl;
#[doc = "TIMER_CLK_CTL (rw) register accessor: Timer prescaler (clk_t to timer clock frequency divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_clk_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_clk_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_clk_ctl`]
module"]
#[doc(alias = "TIMER_CLK_CTL")]
pub type TimerClkCtl = crate::Reg<timer_clk_ctl::TimerClkCtlSpec>;
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)"]
pub mod timer_clk_ctl;
#[doc = "TIMER_CTL (rw) register accessor: Timer control\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_ctl`]
module"]
#[doc(alias = "TIMER_CTL")]
pub type TimerCtl = crate::Reg<timer_ctl::TimerCtlSpec>;
#[doc = "Timer control"]
pub mod timer_ctl;
#[doc = "ACLK_CTL (w) register accessor: MPCON clock\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aclk_ctl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aclk_ctl`]
module"]
#[doc(alias = "ACLK_CTL")]
pub type AclkCtl = crate::Reg<aclk_ctl::AclkCtlSpec>;
#[doc = "MPCON clock"]
pub mod aclk_ctl;
#[doc = "INTR (rw) register accessor: Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked"]
pub mod intr_masked;
#[doc = "CAL_CTL0 (rw) register accessor: Cal control BG LO trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl0`]
module"]
#[doc(alias = "CAL_CTL0")]
pub type CalCtl0 = crate::Reg<cal_ctl0::CalCtl0Spec>;
#[doc = "Cal control BG LO trim bits"]
pub mod cal_ctl0;
#[doc = "CAL_CTL1 (rw) register accessor: Cal control BG HI trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl1`]
module"]
#[doc(alias = "CAL_CTL1")]
pub type CalCtl1 = crate::Reg<cal_ctl1::CalCtl1Spec>;
#[doc = "Cal control BG HI trim bits"]
pub mod cal_ctl1;
#[doc = "CAL_CTL2 (rw) register accessor: Cal control BG LO&amp;HI trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl2`]
module"]
#[doc(alias = "CAL_CTL2")]
pub type CalCtl2 = crate::Reg<cal_ctl2::CalCtl2Spec>;
#[doc = "Cal control BG LO&amp;HI trim bits"]
pub mod cal_ctl2;
#[doc = "CAL_CTL3 (rw) register accessor: Cal control osc trim bits, idac, sdac, itim\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl3`]
module"]
#[doc(alias = "CAL_CTL3")]
pub type CalCtl3 = crate::Reg<cal_ctl3::CalCtl3Spec>;
#[doc = "Cal control osc trim bits, idac, sdac, itim"]
pub mod cal_ctl3;
#[doc = "CAL_CTL4 (rw) register accessor: Cal Control Vlim, SA, fdiv, reg_act\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl4`]
module"]
#[doc(alias = "CAL_CTL4")]
pub type CalCtl4 = crate::Reg<cal_ctl4::CalCtl4Spec>;
#[doc = "Cal Control Vlim, SA, fdiv, reg_act"]
pub mod cal_ctl4;
#[doc = "CAL_CTL5 (rw) register accessor: Cal control\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl5`]
module"]
#[doc(alias = "CAL_CTL5")]
pub type CalCtl5 = crate::Reg<cal_ctl5::CalCtl5Spec>;
#[doc = "Cal control"]
pub mod cal_ctl5;
#[doc = "CAL_CTL6 (rw) register accessor: SA trim LP/ULP\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl6`]
module"]
#[doc(alias = "CAL_CTL6")]
pub type CalCtl6 = crate::Reg<cal_ctl6::CalCtl6Spec>;
#[doc = "SA trim LP/ULP"]
pub mod cal_ctl6;
#[doc = "CAL_CTL7 (rw) register accessor: Cal control\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_ctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_ctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_ctl7`]
module"]
#[doc(alias = "CAL_CTL7")]
pub type CalCtl7 = crate::Reg<cal_ctl7::CalCtl7Spec>;
#[doc = "Cal control"]
pub mod cal_ctl7;
#[doc = "RED_CTL01 (rw) register accessor: Redundancy Control normal sectors 0,1\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl01`]
module"]
#[doc(alias = "RED_CTL01")]
pub type RedCtl01 = crate::Reg<red_ctl01::RedCtl01Spec>;
#[doc = "Redundancy Control normal sectors 0,1"]
pub mod red_ctl01;
#[doc = "RED_CTL23 (rw) register accessor: Redundancy Control normal sectors 2,3\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl23`]
module"]
#[doc(alias = "RED_CTL23")]
pub type RedCtl23 = crate::Reg<red_ctl23::RedCtl23Spec>;
#[doc = "Redundancy Control normal sectors 2,3"]
pub mod red_ctl23;
#[doc = "RED_CTL45 (rw) register accessor: Redundancy Control normal sectors 4,5\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl45`]
module"]
#[doc(alias = "RED_CTL45")]
pub type RedCtl45 = crate::Reg<red_ctl45::RedCtl45Spec>;
#[doc = "Redundancy Control normal sectors 4,5"]
pub mod red_ctl45;
#[doc = "RED_CTL67 (rw) register accessor: Redundancy Control normal sectors 6,7\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl67`]
module"]
#[doc(alias = "RED_CTL67")]
pub type RedCtl67 = crate::Reg<red_ctl67::RedCtl67Spec>;
#[doc = "Redundancy Control normal sectors 6,7"]
pub mod red_ctl67;
#[doc = "RED_CTL_SM01 (rw) register accessor: Redundancy Control special sectors 0,1\n\nYou can [`read`](crate::Reg::read) this register and get [`red_ctl_sm01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`red_ctl_sm01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@red_ctl_sm01`]
module"]
#[doc(alias = "RED_CTL_SM01")]
pub type RedCtlSm01 = crate::Reg<red_ctl_sm01::RedCtlSm01Spec>;
#[doc = "Redundancy Control special sectors 0,1"]
pub mod red_ctl_sm01;
#[doc = "RGRANT_DELAY_PRG (rw) register accessor: R-grant delay for program\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_delay_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_delay_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_delay_prg`]
module"]
#[doc(alias = "RGRANT_DELAY_PRG")]
pub type RgrantDelayPrg = crate::Reg<rgrant_delay_prg::RgrantDelayPrgSpec>;
#[doc = "R-grant delay for program"]
pub mod rgrant_delay_prg;
#[doc = "PW_SEQ12 (rw) register accessor: HV Pulse Delay for seq 1&amp;2 pre\n\nYou can [`read`](crate::Reg::read) this register and get [`pw_seq12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pw_seq12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pw_seq12`]
module"]
#[doc(alias = "PW_SEQ12")]
pub type PwSeq12 = crate::Reg<pw_seq12::PwSeq12Spec>;
#[doc = "HV Pulse Delay for seq 1&amp;2 pre"]
pub mod pw_seq12;
#[doc = "PW_SEQ23 (rw) register accessor: HV Pulse Delay for seq2 post &amp; seq3\n\nYou can [`read`](crate::Reg::read) this register and get [`pw_seq23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pw_seq23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pw_seq23`]
module"]
#[doc(alias = "PW_SEQ23")]
pub type PwSeq23 = crate::Reg<pw_seq23::PwSeq23Spec>;
#[doc = "HV Pulse Delay for seq2 post &amp; seq3"]
pub mod pw_seq23;
#[doc = "RGRANT_SCALE_ERS (rw) register accessor: R-grant delay scale for erase\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_scale_ers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_scale_ers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_scale_ers`]
module"]
#[doc(alias = "RGRANT_SCALE_ERS")]
pub type RgrantScaleErs = crate::Reg<rgrant_scale_ers::RgrantScaleErsSpec>;
#[doc = "R-grant delay scale for erase"]
pub mod rgrant_scale_ers;
#[doc = "RGRANT_DELAY_ERS (rw) register accessor: R-grant delay for erase\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_delay_ers::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_delay_ers::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgrant_delay_ers`]
module"]
#[doc(alias = "RGRANT_DELAY_ERS")]
pub type RgrantDelayErs = crate::Reg<rgrant_delay_ers::RgrantDelayErsSpec>;
#[doc = "R-grant delay for erase"]
pub mod rgrant_delay_ers;
#[doc = "FM_PL_WRDATA_ALL (rw) register accessor: Flash macro write page latches all\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_pl_wrdata_all::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_pl_wrdata_all::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_pl_wrdata_all`]
module"]
#[doc(alias = "FM_PL_WRDATA_ALL")]
pub type FmPlWrdataAll = crate::Reg<fm_pl_wrdata_all::FmPlWrdataAllSpec>;
#[doc = "Flash macro write page latches all"]
pub mod fm_pl_wrdata_all;
#[doc = "FM_PL_DATA (rw) register accessor: Flash macro Page Latches data\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_pl_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_pl_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_pl_data`]
module"]
#[doc(alias = "FM_PL_DATA")]
pub type FmPlData = crate::Reg<fm_pl_data::FmPlDataSpec>;
#[doc = "Flash macro Page Latches data"]
pub mod fm_pl_data;
#[doc = "FM_MEM_DATA (r) register accessor: Flash macro memory sense amplifier and column decoder data\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_mem_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm_mem_data`]
module"]
#[doc(alias = "FM_MEM_DATA")]
pub type FmMemData = crate::Reg<fm_mem_data::FmMemDataSpec>;
#[doc = "Flash macro memory sense amplifier and column decoder data"]
pub mod fm_mem_data;
