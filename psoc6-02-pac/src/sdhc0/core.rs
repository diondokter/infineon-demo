#[repr(C)]
#[doc = "MMIO for Synopsys Mobile Storage Host Controller IP"]
#[doc(alias = "CORE")]
pub struct Core {
    sdmasa_r: SdmasaR,
    blocksize_r: BlocksizeR,
    blockcount_r: BlockcountR,
    argument_r: ArgumentR,
    xfer_mode_r: XferModeR,
    cmd_r: CmdR,
    resp01_r: Resp01R,
    resp23_r: Resp23R,
    resp45_r: Resp45R,
    resp67_r: Resp67R,
    buf_data_r: BufDataR,
    pstate_reg: PstateReg,
    host_ctrl1_r: HostCtrl1R,
    pwr_ctrl_r: PwrCtrlR,
    bgap_ctrl_r: BgapCtrlR,
    wup_ctrl_r: WupCtrlR,
    clk_ctrl_r: ClkCtrlR,
    tout_ctrl_r: ToutCtrlR,
    sw_rst_r: SwRstR,
    normal_int_stat_r: NormalIntStatR,
    error_int_stat_r: ErrorIntStatR,
    normal_int_stat_en_r: NormalIntStatEnR,
    error_int_stat_en_r: ErrorIntStatEnR,
    normal_int_signal_en_r: NormalIntSignalEnR,
    error_int_signal_en_r: ErrorIntSignalEnR,
    auto_cmd_stat_r: AutoCmdStatR,
    host_ctrl2_r: HostCtrl2R,
    capabilities1_r: Capabilities1R,
    capabilities2_r: Capabilities2R,
    curr_capabilities1_r: CurrCapabilities1R,
    curr_capabilities2_r: CurrCapabilities2R,
    force_auto_cmd_stat_r: ForceAutoCmdStatR,
    force_error_int_stat_r: ForceErrorIntStatR,
    adma_err_stat_r: AdmaErrStatR,
    _reserved34: [u8; 0x03],
    adma_sa_low_r: AdmaSaLowR,
    _reserved35: [u8; 0x1c],
    adma_id_low_r: AdmaIdLowR,
    _reserved36: [u8; 0x82],
    host_cntrl_vers_r: HostCntrlVersR,
    _reserved37: [u8; 0x80],
    cqver: Cqver,
    cqcap: Cqcap,
    cqcfg: Cqcfg,
    cqctl: Cqctl,
    cqis: Cqis,
    cqise: Cqise,
    cqisge: Cqisge,
    cqic: Cqic,
    cqtdlba: Cqtdlba,
    _reserved46: [u8; 0x04],
    cqtdbr: Cqtdbr,
    cqtcn: Cqtcn,
    cqdqs: Cqdqs,
    cqdpt: Cqdpt,
    cqtclr: Cqtclr,
    _reserved51: [u8; 0x04],
    cqssc1: Cqssc1,
    cqssc2: Cqssc2,
    cqcrdct: Cqcrdct,
    _reserved54: [u8; 0x04],
    cqrmem: Cqrmem,
    cqterri: Cqterri,
    cqcri: Cqcri,
    cqcra: Cqcra,
    _reserved58: [u8; 0x0320],
    mshc_ver_id_r: MshcVerIdR,
    mshc_ver_type_r: MshcVerTypeR,
    mshc_ctrl_r: MshcCtrlR,
    _reserved61: [u8; 0x07],
    mbiu_ctrl_r: MbiuCtrlR,
    _reserved62: [u8; 0x1b],
    emmc_ctrl_r: EmmcCtrlR,
    boot_ctrl_r: BootCtrlR,
    gp_in_r: GpInR,
    gp_out_r: GpOutR,
}
impl Core {
    #[doc = "0x00 - SDMA System Address register"]
    #[inline(always)]
    pub const fn sdmasa_r(&self) -> &SdmasaR {
        &self.sdmasa_r
    }
    #[doc = "0x04 - Block Size register"]
    #[inline(always)]
    pub const fn blocksize_r(&self) -> &BlocksizeR {
        &self.blocksize_r
    }
    #[doc = "0x06 - 16-bit Block Count register"]
    #[inline(always)]
    pub const fn blockcount_r(&self) -> &BlockcountR {
        &self.blockcount_r
    }
    #[doc = "0x08 - Argument register"]
    #[inline(always)]
    pub const fn argument_r(&self) -> &ArgumentR {
        &self.argument_r
    }
    #[doc = "0x0c - Transfer Mode register"]
    #[inline(always)]
    pub const fn xfer_mode_r(&self) -> &XferModeR {
        &self.xfer_mode_r
    }
    #[doc = "0x0e - Command register"]
    #[inline(always)]
    pub const fn cmd_r(&self) -> &CmdR {
        &self.cmd_r
    }
    #[doc = "0x10 - Response Register 0/1"]
    #[inline(always)]
    pub const fn resp01_r(&self) -> &Resp01R {
        &self.resp01_r
    }
    #[doc = "0x14 - Response Register 2/3"]
    #[inline(always)]
    pub const fn resp23_r(&self) -> &Resp23R {
        &self.resp23_r
    }
    #[doc = "0x18 - Response Register 4/5"]
    #[inline(always)]
    pub const fn resp45_r(&self) -> &Resp45R {
        &self.resp45_r
    }
    #[doc = "0x1c - Response Register 6/7"]
    #[inline(always)]
    pub const fn resp67_r(&self) -> &Resp67R {
        &self.resp67_r
    }
    #[doc = "0x20 - Buffer Data Port Register"]
    #[inline(always)]
    pub const fn buf_data_r(&self) -> &BufDataR {
        &self.buf_data_r
    }
    #[doc = "0x24 - Present State Register"]
    #[inline(always)]
    pub const fn pstate_reg(&self) -> &PstateReg {
        &self.pstate_reg
    }
    #[doc = "0x28 - Host Control 1 Register"]
    #[inline(always)]
    pub const fn host_ctrl1_r(&self) -> &HostCtrl1R {
        &self.host_ctrl1_r
    }
    #[doc = "0x29 - Power Control Register"]
    #[inline(always)]
    pub const fn pwr_ctrl_r(&self) -> &PwrCtrlR {
        &self.pwr_ctrl_r
    }
    #[doc = "0x2a - Block Gap Control Register"]
    #[inline(always)]
    pub const fn bgap_ctrl_r(&self) -> &BgapCtrlR {
        &self.bgap_ctrl_r
    }
    #[doc = "0x2b - Wakeup Control Register"]
    #[inline(always)]
    pub const fn wup_ctrl_r(&self) -> &WupCtrlR {
        &self.wup_ctrl_r
    }
    #[doc = "0x2c - Clock Control Register"]
    #[inline(always)]
    pub const fn clk_ctrl_r(&self) -> &ClkCtrlR {
        &self.clk_ctrl_r
    }
    #[doc = "0x2e - Timeout Control Register"]
    #[inline(always)]
    pub const fn tout_ctrl_r(&self) -> &ToutCtrlR {
        &self.tout_ctrl_r
    }
    #[doc = "0x2f - Software Reset Register"]
    #[inline(always)]
    pub const fn sw_rst_r(&self) -> &SwRstR {
        &self.sw_rst_r
    }
    #[doc = "0x30 - Normal Interrupt Status Register"]
    #[inline(always)]
    pub const fn normal_int_stat_r(&self) -> &NormalIntStatR {
        &self.normal_int_stat_r
    }
    #[doc = "0x32 - Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn error_int_stat_r(&self) -> &ErrorIntStatR {
        &self.error_int_stat_r
    }
    #[doc = "0x34 - Normal Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn normal_int_stat_en_r(&self) -> &NormalIntStatEnR {
        &self.normal_int_stat_en_r
    }
    #[doc = "0x36 - Error Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn error_int_stat_en_r(&self) -> &ErrorIntStatEnR {
        &self.error_int_stat_en_r
    }
    #[doc = "0x38 - Normal Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn normal_int_signal_en_r(&self) -> &NormalIntSignalEnR {
        &self.normal_int_signal_en_r
    }
    #[doc = "0x3a - Error Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn error_int_signal_en_r(&self) -> &ErrorIntSignalEnR {
        &self.error_int_signal_en_r
    }
    #[doc = "0x3c - Auto CMD Status Register"]
    #[inline(always)]
    pub const fn auto_cmd_stat_r(&self) -> &AutoCmdStatR {
        &self.auto_cmd_stat_r
    }
    #[doc = "0x3e - Host Control 2 Register"]
    #[inline(always)]
    pub const fn host_ctrl2_r(&self) -> &HostCtrl2R {
        &self.host_ctrl2_r
    }
    #[doc = "0x40 - Capabilities 1 Register - 0 to 31"]
    #[inline(always)]
    pub const fn capabilities1_r(&self) -> &Capabilities1R {
        &self.capabilities1_r
    }
    #[doc = "0x44 - Capabilities Register - 32 to 63"]
    #[inline(always)]
    pub const fn capabilities2_r(&self) -> &Capabilities2R {
        &self.capabilities2_r
    }
    #[doc = "0x48 - Current Capabilities Register - 0 to 31"]
    #[inline(always)]
    pub const fn curr_capabilities1_r(&self) -> &CurrCapabilities1R {
        &self.curr_capabilities1_r
    }
    #[doc = "0x4c - Maximum Current Capabilities Register - 32 to 63"]
    #[inline(always)]
    pub const fn curr_capabilities2_r(&self) -> &CurrCapabilities2R {
        &self.curr_capabilities2_r
    }
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status register"]
    #[inline(always)]
    pub const fn force_auto_cmd_stat_r(&self) -> &ForceAutoCmdStatR {
        &self.force_auto_cmd_stat_r
    }
    #[doc = "0x52 - Force Event Register for Error Interrupt Status"]
    #[inline(always)]
    pub const fn force_error_int_stat_r(&self) -> &ForceErrorIntStatR {
        &self.force_error_int_stat_r
    }
    #[doc = "0x54 - ADMA Error Status Register"]
    #[inline(always)]
    pub const fn adma_err_stat_r(&self) -> &AdmaErrStatR {
        &self.adma_err_stat_r
    }
    #[doc = "0x58 - ADMA System Address Register - Low"]
    #[inline(always)]
    pub const fn adma_sa_low_r(&self) -> &AdmaSaLowR {
        &self.adma_sa_low_r
    }
    #[doc = "0x78 - ADMA3 Integrated Descriptor Address Register - Low"]
    #[inline(always)]
    pub const fn adma_id_low_r(&self) -> &AdmaIdLowR {
        &self.adma_id_low_r
    }
    #[doc = "0xfe - Host Controller Version"]
    #[inline(always)]
    pub const fn host_cntrl_vers_r(&self) -> &HostCntrlVersR {
        &self.host_cntrl_vers_r
    }
    #[doc = "0x180 - Command Queuing Version register"]
    #[inline(always)]
    pub const fn cqver(&self) -> &Cqver {
        &self.cqver
    }
    #[doc = "0x184 - Command Queuing Capabilities register"]
    #[inline(always)]
    pub const fn cqcap(&self) -> &Cqcap {
        &self.cqcap
    }
    #[doc = "0x188 - Command Queuing Configuration register"]
    #[inline(always)]
    pub const fn cqcfg(&self) -> &Cqcfg {
        &self.cqcfg
    }
    #[doc = "0x18c - Command Queuing Control register"]
    #[inline(always)]
    pub const fn cqctl(&self) -> &Cqctl {
        &self.cqctl
    }
    #[doc = "0x190 - Command Queuing Interrupt Status register"]
    #[inline(always)]
    pub const fn cqis(&self) -> &Cqis {
        &self.cqis
    }
    #[doc = "0x194 - Command Queuing Interrupt Status Enable register"]
    #[inline(always)]
    pub const fn cqise(&self) -> &Cqise {
        &self.cqise
    }
    #[doc = "0x198 - Command Queuing Interrupt signal enable register"]
    #[inline(always)]
    pub const fn cqisge(&self) -> &Cqisge {
        &self.cqisge
    }
    #[doc = "0x19c - Command Queuing Interrupt Coalescing register"]
    #[inline(always)]
    pub const fn cqic(&self) -> &Cqic {
        &self.cqic
    }
    #[doc = "0x1a0 - Command Queuing Task Descriptor List Base Address register"]
    #[inline(always)]
    pub const fn cqtdlba(&self) -> &Cqtdlba {
        &self.cqtdlba
    }
    #[doc = "0x1a8 - Command Queuing DoorBell register"]
    #[inline(always)]
    pub const fn cqtdbr(&self) -> &Cqtdbr {
        &self.cqtdbr
    }
    #[doc = "0x1ac - Command Queuing TaskClear Notification register"]
    #[inline(always)]
    pub const fn cqtcn(&self) -> &Cqtcn {
        &self.cqtcn
    }
    #[doc = "0x1b0 - Device queue status register"]
    #[inline(always)]
    pub const fn cqdqs(&self) -> &Cqdqs {
        &self.cqdqs
    }
    #[doc = "0x1b4 - Device pending tasks register"]
    #[inline(always)]
    pub const fn cqdpt(&self) -> &Cqdpt {
        &self.cqdpt
    }
    #[doc = "0x1b8 - Command Queuing DoorBell register"]
    #[inline(always)]
    pub const fn cqtclr(&self) -> &Cqtclr {
        &self.cqtclr
    }
    #[doc = "0x1c0 - CQ Send Status Configuration 1 register"]
    #[inline(always)]
    pub const fn cqssc1(&self) -> &Cqssc1 {
        &self.cqssc1
    }
    #[doc = "0x1c4 - CQ Send Status Configuration 2 register"]
    #[inline(always)]
    pub const fn cqssc2(&self) -> &Cqssc2 {
        &self.cqssc2
    }
    #[doc = "0x1c8 - Command response for direct command register"]
    #[inline(always)]
    pub const fn cqcrdct(&self) -> &Cqcrdct {
        &self.cqcrdct
    }
    #[doc = "0x1d0 - Command response mode error mask register"]
    #[inline(always)]
    pub const fn cqrmem(&self) -> &Cqrmem {
        &self.cqrmem
    }
    #[doc = "0x1d4 - CQ Task Error Information register"]
    #[inline(always)]
    pub const fn cqterri(&self) -> &Cqterri {
        &self.cqterri
    }
    #[doc = "0x1d8 - CQ Command response index"]
    #[inline(always)]
    pub const fn cqcri(&self) -> &Cqcri {
        &self.cqcri
    }
    #[doc = "0x1dc - CQ Command response argument register"]
    #[inline(always)]
    pub const fn cqcra(&self) -> &Cqcra {
        &self.cqcra
    }
    #[doc = "0x500 - MSHC version"]
    #[inline(always)]
    pub const fn mshc_ver_id_r(&self) -> &MshcVerIdR {
        &self.mshc_ver_id_r
    }
    #[doc = "0x504 - MSHC version type"]
    #[inline(always)]
    pub const fn mshc_ver_type_r(&self) -> &MshcVerTypeR {
        &self.mshc_ver_type_r
    }
    #[doc = "0x508 - MSHC Control register"]
    #[inline(always)]
    pub const fn mshc_ctrl_r(&self) -> &MshcCtrlR {
        &self.mshc_ctrl_r
    }
    #[doc = "0x510 - MBIU Control register"]
    #[inline(always)]
    pub const fn mbiu_ctrl_r(&self) -> &MbiuCtrlR {
        &self.mbiu_ctrl_r
    }
    #[doc = "0x52c - eMMC Control register"]
    #[inline(always)]
    pub const fn emmc_ctrl_r(&self) -> &EmmcCtrlR {
        &self.emmc_ctrl_r
    }
    #[doc = "0x52e - eMMC Boot Control register"]
    #[inline(always)]
    pub const fn boot_ctrl_r(&self) -> &BootCtrlR {
        &self.boot_ctrl_r
    }
    #[doc = "0x530 - General Purpose Input register"]
    #[inline(always)]
    pub const fn gp_in_r(&self) -> &GpInR {
        &self.gp_in_r
    }
    #[doc = "0x534 - General Purpose Output register"]
    #[inline(always)]
    pub const fn gp_out_r(&self) -> &GpOutR {
        &self.gp_out_r
    }
}
#[doc = "SDMASA_R (rw) register accessor: SDMA System Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmasa_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmasa_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmasa_r`]
module"]
#[doc(alias = "SDMASA_R")]
pub type SdmasaR = crate::Reg<sdmasa_r::SdmasaRSpec>;
#[doc = "SDMA System Address register"]
pub mod sdmasa_r;
#[doc = "BLOCKSIZE_R (rw) register accessor: Block Size register\n\nYou can [`read`](crate::Reg::read) this register and get [`blocksize_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blocksize_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blocksize_r`]
module"]
#[doc(alias = "BLOCKSIZE_R")]
pub type BlocksizeR = crate::Reg<blocksize_r::BlocksizeRSpec>;
#[doc = "Block Size register"]
pub mod blocksize_r;
#[doc = "BLOCKCOUNT_R (rw) register accessor: 16-bit Block Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`blockcount_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blockcount_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blockcount_r`]
module"]
#[doc(alias = "BLOCKCOUNT_R")]
pub type BlockcountR = crate::Reg<blockcount_r::BlockcountRSpec>;
#[doc = "16-bit Block Count register"]
pub mod blockcount_r;
#[doc = "ARGUMENT_R (rw) register accessor: Argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`argument_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument_r`]
module"]
#[doc(alias = "ARGUMENT_R")]
pub type ArgumentR = crate::Reg<argument_r::ArgumentRSpec>;
#[doc = "Argument register"]
pub mod argument_r;
#[doc = "XFER_MODE_R (rw) register accessor: Transfer Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`xfer_mode_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xfer_mode_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xfer_mode_r`]
module"]
#[doc(alias = "XFER_MODE_R")]
pub type XferModeR = crate::Reg<xfer_mode_r::XferModeRSpec>;
#[doc = "Transfer Mode register"]
pub mod xfer_mode_r;
#[doc = "CMD_R (rw) register accessor: Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_r`]
module"]
#[doc(alias = "CMD_R")]
pub type CmdR = crate::Reg<cmd_r::CmdRSpec>;
#[doc = "Command register"]
pub mod cmd_r;
#[doc = "RESP01_R (r) register accessor: Response Register 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp01_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp01_r`]
module"]
#[doc(alias = "RESP01_R")]
pub type Resp01R = crate::Reg<resp01_r::Resp01RSpec>;
#[doc = "Response Register 0/1"]
pub mod resp01_r;
#[doc = "RESP23_R (r) register accessor: Response Register 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp23_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp23_r`]
module"]
#[doc(alias = "RESP23_R")]
pub type Resp23R = crate::Reg<resp23_r::Resp23RSpec>;
#[doc = "Response Register 2/3"]
pub mod resp23_r;
#[doc = "RESP45_R (r) register accessor: Response Register 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`resp45_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp45_r`]
module"]
#[doc(alias = "RESP45_R")]
pub type Resp45R = crate::Reg<resp45_r::Resp45RSpec>;
#[doc = "Response Register 4/5"]
pub mod resp45_r;
#[doc = "RESP67_R (r) register accessor: Response Register 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`resp67_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resp67_r`]
module"]
#[doc(alias = "RESP67_R")]
pub type Resp67R = crate::Reg<resp67_r::Resp67RSpec>;
#[doc = "Response Register 6/7"]
pub mod resp67_r;
#[doc = "BUF_DATA_R (rw) register accessor: Buffer Data Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_data_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_data_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_data_r`]
module"]
#[doc(alias = "BUF_DATA_R")]
pub type BufDataR = crate::Reg<buf_data_r::BufDataRSpec>;
#[doc = "Buffer Data Port Register"]
pub mod buf_data_r;
#[doc = "PSTATE_REG (r) register accessor: Present State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pstate_reg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pstate_reg`]
module"]
#[doc(alias = "PSTATE_REG")]
pub type PstateReg = crate::Reg<pstate_reg::PstateRegSpec>;
#[doc = "Present State Register"]
pub mod pstate_reg;
#[doc = "HOST_CTRL1_R (rw) register accessor: Host Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl1_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl1_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl1_r`]
module"]
#[doc(alias = "HOST_CTRL1_R")]
pub type HostCtrl1R = crate::Reg<host_ctrl1_r::HostCtrl1RSpec>;
#[doc = "Host Control 1 Register"]
pub mod host_ctrl1_r;
#[doc = "PWR_CTRL_R (rw) register accessor: Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ctrl_r`]
module"]
#[doc(alias = "PWR_CTRL_R")]
pub type PwrCtrlR = crate::Reg<pwr_ctrl_r::PwrCtrlRSpec>;
#[doc = "Power Control Register"]
pub mod pwr_ctrl_r;
#[doc = "BGAP_CTRL_R (rw) register accessor: Block Gap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgap_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgap_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgap_ctrl_r`]
module"]
#[doc(alias = "BGAP_CTRL_R")]
pub type BgapCtrlR = crate::Reg<bgap_ctrl_r::BgapCtrlRSpec>;
#[doc = "Block Gap Control Register"]
pub mod bgap_ctrl_r;
#[doc = "WUP_CTRL_R (rw) register accessor: Wakeup Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wup_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wup_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wup_ctrl_r`]
module"]
#[doc(alias = "WUP_CTRL_R")]
pub type WupCtrlR = crate::Reg<wup_ctrl_r::WupCtrlRSpec>;
#[doc = "Wakeup Control Register"]
pub mod wup_ctrl_r;
#[doc = "CLK_CTRL_R (rw) register accessor: Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ctrl_r`]
module"]
#[doc(alias = "CLK_CTRL_R")]
pub type ClkCtrlR = crate::Reg<clk_ctrl_r::ClkCtrlRSpec>;
#[doc = "Clock Control Register"]
pub mod clk_ctrl_r;
#[doc = "TOUT_CTRL_R (rw) register accessor: Timeout Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tout_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tout_ctrl_r`]
module"]
#[doc(alias = "TOUT_CTRL_R")]
pub type ToutCtrlR = crate::Reg<tout_ctrl_r::ToutCtrlRSpec>;
#[doc = "Timeout Control Register"]
pub mod tout_ctrl_r;
#[doc = "SW_RST_R (rw) register accessor: Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_rst_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_rst_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_rst_r`]
module"]
#[doc(alias = "SW_RST_R")]
pub type SwRstR = crate::Reg<sw_rst_r::SwRstRSpec>;
#[doc = "Software Reset Register"]
pub mod sw_rst_r;
#[doc = "NORMAL_INT_STAT_R (rw) register accessor: Normal Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_stat_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_stat_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_stat_r`]
module"]
#[doc(alias = "NORMAL_INT_STAT_R")]
pub type NormalIntStatR = crate::Reg<normal_int_stat_r::NormalIntStatRSpec>;
#[doc = "Normal Interrupt Status Register"]
pub mod normal_int_stat_r;
#[doc = "ERROR_INT_STAT_R (rw) register accessor: Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_stat_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_stat_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_stat_r`]
module"]
#[doc(alias = "ERROR_INT_STAT_R")]
pub type ErrorIntStatR = crate::Reg<error_int_stat_r::ErrorIntStatRSpec>;
#[doc = "Error Interrupt Status Register"]
pub mod error_int_stat_r;
#[doc = "NORMAL_INT_STAT_EN_R (rw) register accessor: Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_stat_en_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_stat_en_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_stat_en_r`]
module"]
#[doc(alias = "NORMAL_INT_STAT_EN_R")]
pub type NormalIntStatEnR = crate::Reg<normal_int_stat_en_r::NormalIntStatEnRSpec>;
#[doc = "Normal Interrupt Status Enable Register"]
pub mod normal_int_stat_en_r;
#[doc = "ERROR_INT_STAT_EN_R (rw) register accessor: Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_stat_en_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_stat_en_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_stat_en_r`]
module"]
#[doc(alias = "ERROR_INT_STAT_EN_R")]
pub type ErrorIntStatEnR = crate::Reg<error_int_stat_en_r::ErrorIntStatEnRSpec>;
#[doc = "Error Interrupt Status Enable Register"]
pub mod error_int_stat_en_r;
#[doc = "NORMAL_INT_SIGNAL_EN_R (rw) register accessor: Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_signal_en_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_signal_en_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@normal_int_signal_en_r`]
module"]
#[doc(alias = "NORMAL_INT_SIGNAL_EN_R")]
pub type NormalIntSignalEnR = crate::Reg<normal_int_signal_en_r::NormalIntSignalEnRSpec>;
#[doc = "Normal Interrupt Signal Enable Register"]
pub mod normal_int_signal_en_r;
#[doc = "ERROR_INT_SIGNAL_EN_R (rw) register accessor: Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_signal_en_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_signal_en_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_int_signal_en_r`]
module"]
#[doc(alias = "ERROR_INT_SIGNAL_EN_R")]
pub type ErrorIntSignalEnR = crate::Reg<error_int_signal_en_r::ErrorIntSignalEnRSpec>;
#[doc = "Error Interrupt Signal Enable Register"]
pub mod error_int_signal_en_r;
#[doc = "AUTO_CMD_STAT_R (r) register accessor: Auto CMD Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`auto_cmd_stat_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auto_cmd_stat_r`]
module"]
#[doc(alias = "AUTO_CMD_STAT_R")]
pub type AutoCmdStatR = crate::Reg<auto_cmd_stat_r::AutoCmdStatRSpec>;
#[doc = "Auto CMD Status Register"]
pub mod auto_cmd_stat_r;
#[doc = "HOST_CTRL2_R (rw) register accessor: Host Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctrl2_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctrl2_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_ctrl2_r`]
module"]
#[doc(alias = "HOST_CTRL2_R")]
pub type HostCtrl2R = crate::Reg<host_ctrl2_r::HostCtrl2RSpec>;
#[doc = "Host Control 2 Register"]
pub mod host_ctrl2_r;
#[doc = "CAPABILITIES1_R (r) register accessor: Capabilities 1 Register - 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities1_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities1_r`]
module"]
#[doc(alias = "CAPABILITIES1_R")]
pub type Capabilities1R = crate::Reg<capabilities1_r::Capabilities1RSpec>;
#[doc = "Capabilities 1 Register - 0 to 31"]
pub mod capabilities1_r;
#[doc = "CAPABILITIES2_R (r) register accessor: Capabilities Register - 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`capabilities2_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capabilities2_r`]
module"]
#[doc(alias = "CAPABILITIES2_R")]
pub type Capabilities2R = crate::Reg<capabilities2_r::Capabilities2RSpec>;
#[doc = "Capabilities Register - 32 to 63"]
pub mod capabilities2_r;
#[doc = "CURR_CAPABILITIES1_R (r) register accessor: Current Capabilities Register - 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_capabilities1_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr_capabilities1_r`]
module"]
#[doc(alias = "CURR_CAPABILITIES1_R")]
pub type CurrCapabilities1R = crate::Reg<curr_capabilities1_r::CurrCapabilities1RSpec>;
#[doc = "Current Capabilities Register - 0 to 31"]
pub mod curr_capabilities1_r;
#[doc = "CURR_CAPABILITIES2_R (r) register accessor: Maximum Current Capabilities Register - 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_capabilities2_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@curr_capabilities2_r`]
module"]
#[doc(alias = "CURR_CAPABILITIES2_R")]
pub type CurrCapabilities2R = crate::Reg<curr_capabilities2_r::CurrCapabilities2RSpec>;
#[doc = "Maximum Current Capabilities Register - 32 to 63"]
pub mod curr_capabilities2_r;
#[doc = "FORCE_AUTO_CMD_STAT_R (w) register accessor: Force Event Register for Auto CMD Error Status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_auto_cmd_stat_r::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_auto_cmd_stat_r`]
module"]
#[doc(alias = "FORCE_AUTO_CMD_STAT_R")]
pub type ForceAutoCmdStatR = crate::Reg<force_auto_cmd_stat_r::ForceAutoCmdStatRSpec>;
#[doc = "Force Event Register for Auto CMD Error Status register"]
pub mod force_auto_cmd_stat_r;
#[doc = "FORCE_ERROR_INT_STAT_R (rw) register accessor: Force Event Register for Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`force_error_int_stat_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_error_int_stat_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@force_error_int_stat_r`]
module"]
#[doc(alias = "FORCE_ERROR_INT_STAT_R")]
pub type ForceErrorIntStatR = crate::Reg<force_error_int_stat_r::ForceErrorIntStatRSpec>;
#[doc = "Force Event Register for Error Interrupt Status"]
pub mod force_error_int_stat_r;
#[doc = "ADMA_ERR_STAT_R (r) register accessor: ADMA Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_err_stat_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_err_stat_r`]
module"]
#[doc(alias = "ADMA_ERR_STAT_R")]
pub type AdmaErrStatR = crate::Reg<adma_err_stat_r::AdmaErrStatRSpec>;
#[doc = "ADMA Error Status Register"]
pub mod adma_err_stat_r;
#[doc = "ADMA_SA_LOW_R (rw) register accessor: ADMA System Address Register - Low\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_sa_low_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_sa_low_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_sa_low_r`]
module"]
#[doc(alias = "ADMA_SA_LOW_R")]
pub type AdmaSaLowR = crate::Reg<adma_sa_low_r::AdmaSaLowRSpec>;
#[doc = "ADMA System Address Register - Low"]
pub mod adma_sa_low_r;
#[doc = "ADMA_ID_LOW_R (rw) register accessor: ADMA3 Integrated Descriptor Address Register - Low\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_id_low_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_id_low_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adma_id_low_r`]
module"]
#[doc(alias = "ADMA_ID_LOW_R")]
pub type AdmaIdLowR = crate::Reg<adma_id_low_r::AdmaIdLowRSpec>;
#[doc = "ADMA3 Integrated Descriptor Address Register - Low"]
pub mod adma_id_low_r;
#[doc = "HOST_CNTRL_VERS_R (r) register accessor: Host Controller Version\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cntrl_vers_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@host_cntrl_vers_r`]
module"]
#[doc(alias = "HOST_CNTRL_VERS_R")]
pub type HostCntrlVersR = crate::Reg<host_cntrl_vers_r::HostCntrlVersRSpec>;
#[doc = "Host Controller Version"]
pub mod host_cntrl_vers_r;
#[doc = "CQVER (r) register accessor: Command Queuing Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqver::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqver`]
module"]
#[doc(alias = "CQVER")]
pub type Cqver = crate::Reg<cqver::CqverSpec>;
#[doc = "Command Queuing Version register"]
pub mod cqver;
#[doc = "CQCAP (r) register accessor: Command Queuing Capabilities register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcap`]
module"]
#[doc(alias = "CQCAP")]
pub type Cqcap = crate::Reg<cqcap::CqcapSpec>;
#[doc = "Command Queuing Capabilities register"]
pub mod cqcap;
#[doc = "CQCFG (rw) register accessor: Command Queuing Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcfg`]
module"]
#[doc(alias = "CQCFG")]
pub type Cqcfg = crate::Reg<cqcfg::CqcfgSpec>;
#[doc = "Command Queuing Configuration register"]
pub mod cqcfg;
#[doc = "CQCTL (rw) register accessor: Command Queuing Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqctl`]
module"]
#[doc(alias = "CQCTL")]
pub type Cqctl = crate::Reg<cqctl::CqctlSpec>;
#[doc = "Command Queuing Control register"]
pub mod cqctl;
#[doc = "CQIS (rw) register accessor: Command Queuing Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqis`]
module"]
#[doc(alias = "CQIS")]
pub type Cqis = crate::Reg<cqis::CqisSpec>;
#[doc = "Command Queuing Interrupt Status register"]
pub mod cqis;
#[doc = "CQISE (rw) register accessor: Command Queuing Interrupt Status Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqise::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqise::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqise`]
module"]
#[doc(alias = "CQISE")]
pub type Cqise = crate::Reg<cqise::CqiseSpec>;
#[doc = "Command Queuing Interrupt Status Enable register"]
pub mod cqise;
#[doc = "CQISGE (rw) register accessor: Command Queuing Interrupt signal enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqisge::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqisge::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqisge`]
module"]
#[doc(alias = "CQISGE")]
pub type Cqisge = crate::Reg<cqisge::CqisgeSpec>;
#[doc = "Command Queuing Interrupt signal enable register"]
pub mod cqisge;
#[doc = "CQIC (rw) register accessor: Command Queuing Interrupt Coalescing register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqic`]
module"]
#[doc(alias = "CQIC")]
pub type Cqic = crate::Reg<cqic::CqicSpec>;
#[doc = "Command Queuing Interrupt Coalescing register"]
pub mod cqic;
#[doc = "CQTDLBA (rw) register accessor: Command Queuing Task Descriptor List Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtdlba::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtdlba::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdlba`]
module"]
#[doc(alias = "CQTDLBA")]
pub type Cqtdlba = crate::Reg<cqtdlba::CqtdlbaSpec>;
#[doc = "Command Queuing Task Descriptor List Base Address register"]
pub mod cqtdlba;
#[doc = "CQTDBR (rw) register accessor: Command Queuing DoorBell register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtdbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtdbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtdbr`]
module"]
#[doc(alias = "CQTDBR")]
pub type Cqtdbr = crate::Reg<cqtdbr::CqtdbrSpec>;
#[doc = "Command Queuing DoorBell register"]
pub mod cqtdbr;
#[doc = "CQTCN (rw) register accessor: Command Queuing TaskClear Notification register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtcn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtcn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtcn`]
module"]
#[doc(alias = "CQTCN")]
pub type Cqtcn = crate::Reg<cqtcn::CqtcnSpec>;
#[doc = "Command Queuing TaskClear Notification register"]
pub mod cqtcn;
#[doc = "CQDQS (r) register accessor: Device queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqdqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdqs`]
module"]
#[doc(alias = "CQDQS")]
pub type Cqdqs = crate::Reg<cqdqs::CqdqsSpec>;
#[doc = "Device queue status register"]
pub mod cqdqs;
#[doc = "CQDPT (r) register accessor: Device pending tasks register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqdpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqdpt`]
module"]
#[doc(alias = "CQDPT")]
pub type Cqdpt = crate::Reg<cqdpt::CqdptSpec>;
#[doc = "Device pending tasks register"]
pub mod cqdpt;
#[doc = "CQTCLR (rw) register accessor: Command Queuing DoorBell register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqtclr`]
module"]
#[doc(alias = "CQTCLR")]
pub type Cqtclr = crate::Reg<cqtclr::CqtclrSpec>;
#[doc = "Command Queuing DoorBell register"]
pub mod cqtclr;
#[doc = "CQSSC1 (rw) register accessor: CQ Send Status Configuration 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqssc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqssc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc1`]
module"]
#[doc(alias = "CQSSC1")]
pub type Cqssc1 = crate::Reg<cqssc1::Cqssc1Spec>;
#[doc = "CQ Send Status Configuration 1 register"]
pub mod cqssc1;
#[doc = "CQSSC2 (rw) register accessor: CQ Send Status Configuration 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqssc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqssc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqssc2`]
module"]
#[doc(alias = "CQSSC2")]
pub type Cqssc2 = crate::Reg<cqssc2::Cqssc2Spec>;
#[doc = "CQ Send Status Configuration 2 register"]
pub mod cqssc2;
#[doc = "CQCRDCT (r) register accessor: Command response for direct command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcrdct::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcrdct`]
module"]
#[doc(alias = "CQCRDCT")]
pub type Cqcrdct = crate::Reg<cqcrdct::CqcrdctSpec>;
#[doc = "Command response for direct command register"]
pub mod cqcrdct;
#[doc = "CQRMEM (rw) register accessor: Command response mode error mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqrmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqrmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqrmem`]
module"]
#[doc(alias = "CQRMEM")]
pub type Cqrmem = crate::Reg<cqrmem::CqrmemSpec>;
#[doc = "Command response mode error mask register"]
pub mod cqrmem;
#[doc = "CQTERRI (r) register accessor: CQ Task Error Information register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqterri::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqterri`]
module"]
#[doc(alias = "CQTERRI")]
pub type Cqterri = crate::Reg<cqterri::CqterriSpec>;
#[doc = "CQ Task Error Information register"]
pub mod cqterri;
#[doc = "CQCRI (r) register accessor: CQ Command response index\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcri::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcri`]
module"]
#[doc(alias = "CQCRI")]
pub type Cqcri = crate::Reg<cqcri::CqcriSpec>;
#[doc = "CQ Command response index"]
pub mod cqcri;
#[doc = "CQCRA (r) register accessor: CQ Command response argument register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cqcra`]
module"]
#[doc(alias = "CQCRA")]
pub type Cqcra = crate::Reg<cqcra::CqcraSpec>;
#[doc = "CQ Command response argument register"]
pub mod cqcra;
#[doc = "MSHC_VER_ID_R (r) register accessor: MSHC version\n\nYou can [`read`](crate::Reg::read) this register and get [`mshc_ver_id_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ver_id_r`]
module"]
#[doc(alias = "MSHC_VER_ID_R")]
pub type MshcVerIdR = crate::Reg<mshc_ver_id_r::MshcVerIdRSpec>;
#[doc = "MSHC version"]
pub mod mshc_ver_id_r;
#[doc = "MSHC_VER_TYPE_R (r) register accessor: MSHC version type\n\nYou can [`read`](crate::Reg::read) this register and get [`mshc_ver_type_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ver_type_r`]
module"]
#[doc(alias = "MSHC_VER_TYPE_R")]
pub type MshcVerTypeR = crate::Reg<mshc_ver_type_r::MshcVerTypeRSpec>;
#[doc = "MSHC version type"]
pub mod mshc_ver_type_r;
#[doc = "MSHC_CTRL_R (rw) register accessor: MSHC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mshc_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mshc_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mshc_ctrl_r`]
module"]
#[doc(alias = "MSHC_CTRL_R")]
pub type MshcCtrlR = crate::Reg<mshc_ctrl_r::MshcCtrlRSpec>;
#[doc = "MSHC Control register"]
pub mod mshc_ctrl_r;
#[doc = "MBIU_CTRL_R (rw) register accessor: MBIU Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mbiu_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbiu_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbiu_ctrl_r`]
module"]
#[doc(alias = "MBIU_CTRL_R")]
pub type MbiuCtrlR = crate::Reg<mbiu_ctrl_r::MbiuCtrlRSpec>;
#[doc = "MBIU Control register"]
pub mod mbiu_ctrl_r;
#[doc = "EMMC_CTRL_R (rw) register accessor: eMMC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`emmc_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emmc_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmc_ctrl_r`]
module"]
#[doc(alias = "EMMC_CTRL_R")]
pub type EmmcCtrlR = crate::Reg<emmc_ctrl_r::EmmcCtrlRSpec>;
#[doc = "eMMC Control register"]
pub mod emmc_ctrl_r;
#[doc = "BOOT_CTRL_R (rw) register accessor: eMMC Boot Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_ctrl_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_ctrl_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_ctrl_r`]
module"]
#[doc(alias = "BOOT_CTRL_R")]
pub type BootCtrlR = crate::Reg<boot_ctrl_r::BootCtrlRSpec>;
#[doc = "eMMC Boot Control register"]
pub mod boot_ctrl_r;
#[doc = "GP_IN_R (r) register accessor: General Purpose Input register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_in_r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_in_r`]
module"]
#[doc(alias = "GP_IN_R")]
pub type GpInR = crate::Reg<gp_in_r::GpInRSpec>;
#[doc = "General Purpose Input register"]
pub mod gp_in_r;
#[doc = "GP_OUT_R (rw) register accessor: General Purpose Output register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_out_r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gp_out_r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gp_out_r`]
module"]
#[doc(alias = "GP_OUT_R")]
pub type GpOutR = crate::Reg<gp_out_r::GpOutRSpec>;
#[doc = "General Purpose Output register"]
pub mod gp_out_r;
