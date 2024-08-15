#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwr_ctl: PwrCtl,
    pwr_hibernate: PwrHibernate,
    pwr_lvd_ctl: PwrLvdCtl,
    _reserved3: [u8; 0x08],
    pwr_buck_ctl: PwrBuckCtl,
    pwr_buck_ctl2: PwrBuckCtl2,
    pwr_lvd_status: PwrLvdStatus,
    _reserved6: [u8; 0x60],
    pwr_hib_data: [PwrHibData; 16],
    _reserved7: [u8; 0xc0],
    wdt_ctl: WdtCtl,
    wdt_cnt: WdtCnt,
    wdt_match: WdtMatch,
    _reserved10: [u8; 0x74],
    mcwdt_struct: (),
    _reserved11: [u8; 0x0100],
    clk_dsi_select: [ClkDsiSelect; 16],
    clk_path_select: [ClkPathSelect; 16],
    clk_root_select: [ClkRootSelect; 16],
    _reserved14: [u8; 0x0140],
    clk_select: ClkSelect,
    clk_timer_ctl: ClkTimerCtl,
    _reserved16: [u8; 0x04],
    clk_ilo_config: ClkIloConfig,
    clk_imo_config: ClkImoConfig,
    clk_output_fast: ClkOutputFast,
    clk_output_slow: ClkOutputSlow,
    clk_cal_cnt1: ClkCalCnt1,
    clk_cal_cnt2: ClkCalCnt2,
    _reserved22: [u8; 0x08],
    clk_eco_config: ClkEcoConfig,
    clk_eco_status: ClkEcoStatus,
    _reserved24: [u8; 0x08],
    clk_pilo_config: ClkPiloConfig,
    _reserved25: [u8; 0x40],
    clk_fll_config: ClkFllConfig,
    clk_fll_config2: ClkFllConfig2,
    clk_fll_config3: ClkFllConfig3,
    clk_fll_config4: ClkFllConfig4,
    clk_fll_status: ClkFllStatus,
    _reserved30: [u8; 0x6c],
    clk_pll_config: [ClkPllConfig; 15],
    _reserved31: [u8; 0x04],
    clk_pll_status: [ClkPllStatus; 15],
    _reserved32: [u8; 0x84],
    srss_intr: SrssIntr,
    srss_intr_set: SrssIntrSet,
    srss_intr_mask: SrssIntrMask,
    srss_intr_masked: SrssIntrMasked,
    srss_intr_cfg: SrssIntrCfg,
    _reserved37: [u8; 0xec],
    res_cause: ResCause,
    res_cause2: ResCause2,
    _reserved39: [u8; 0x76f8],
    pwr_trim_ref_ctl: PwrTrimRefCtl,
    pwr_trim_bodovp_ctl: PwrTrimBodovpCtl,
    clk_trim_cco_ctl: ClkTrimCcoCtl,
    clk_trim_cco_ctl2: ClkTrimCcoCtl2,
    _reserved43: [u8; 0x20],
    pwr_trim_wake_ctl: PwrTrimWakeCtl,
    _reserved44: [u8; 0x7fdc],
    pwr_trim_lvd_ctl: PwrTrimLvdCtl,
    _reserved45: [u8; 0x04],
    clk_trim_ilo_ctl: ClkTrimIloCtl,
    pwr_trim_pwrsys_ctl: PwrTrimPwrsysCtl,
    clk_trim_eco_ctl: ClkTrimEcoCtl,
    clk_trim_pilo_ctl: ClkTrimPiloCtl,
    clk_trim_pilo_ctl2: ClkTrimPiloCtl2,
    clk_trim_pilo_ctl3: ClkTrimPiloCtl3,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Mode Control"]
    #[inline(always)]
    pub const fn pwr_ctl(&self) -> &PwrCtl {
        &self.pwr_ctl
    }
    #[doc = "0x04 - HIBERNATE Mode Register"]
    #[inline(always)]
    pub const fn pwr_hibernate(&self) -> &PwrHibernate {
        &self.pwr_hibernate
    }
    #[doc = "0x08 - Low Voltage Detector (LVD) Configuration Register"]
    #[inline(always)]
    pub const fn pwr_lvd_ctl(&self) -> &PwrLvdCtl {
        &self.pwr_lvd_ctl
    }
    #[doc = "0x14 - Buck Control Register"]
    #[inline(always)]
    pub const fn pwr_buck_ctl(&self) -> &PwrBuckCtl {
        &self.pwr_buck_ctl
    }
    #[doc = "0x18 - Buck Control Register 2"]
    #[inline(always)]
    pub const fn pwr_buck_ctl2(&self) -> &PwrBuckCtl2 {
        &self.pwr_buck_ctl2
    }
    #[doc = "0x1c - Low Voltage Detector (LVD) Status Register"]
    #[inline(always)]
    pub const fn pwr_lvd_status(&self) -> &PwrLvdStatus {
        &self.pwr_lvd_status
    }
    #[doc = "0x80..0xc0 - HIBERNATE Data Register"]
    #[inline(always)]
    pub const fn pwr_hib_data(&self, n: usize) -> &PwrHibData {
        &self.pwr_hib_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - HIBERNATE Data Register"]
    #[inline(always)]
    pub fn pwr_hib_data_iter(&self) -> impl Iterator<Item = &PwrHibData> {
        self.pwr_hib_data.iter()
    }
    #[doc = "0x180 - Watchdog Counter Control Register"]
    #[inline(always)]
    pub const fn wdt_ctl(&self) -> &WdtCtl {
        &self.wdt_ctl
    }
    #[doc = "0x184 - Watchdog Counter Count Register"]
    #[inline(always)]
    pub const fn wdt_cnt(&self) -> &WdtCnt {
        &self.wdt_cnt
    }
    #[doc = "0x188 - Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn wdt_match(&self) -> &WdtMatch {
        &self.wdt_match
    }
    #[doc = "0x200..0x258 - Multi-Counter Watchdog Timer"]
    #[inline(always)]
    pub const fn mcwdt_struct(&self, n: usize) -> &McwdtStruct {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(64 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x258 - Multi-Counter Watchdog Timer"]
    #[inline(always)]
    pub fn mcwdt_struct_iter(&self) -> impl Iterator<Item = &McwdtStruct> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(512)
                .add(64 * n)
                .cast()
        })
    }
    #[doc = "0x300..0x340 - Clock DSI Select Register"]
    #[inline(always)]
    pub const fn clk_dsi_select(&self, n: usize) -> &ClkDsiSelect {
        &self.clk_dsi_select[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x340 - Clock DSI Select Register"]
    #[inline(always)]
    pub fn clk_dsi_select_iter(&self) -> impl Iterator<Item = &ClkDsiSelect> {
        self.clk_dsi_select.iter()
    }
    #[doc = "0x340..0x380 - Clock Path Select Register"]
    #[inline(always)]
    pub const fn clk_path_select(&self, n: usize) -> &ClkPathSelect {
        &self.clk_path_select[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x340..0x380 - Clock Path Select Register"]
    #[inline(always)]
    pub fn clk_path_select_iter(&self) -> impl Iterator<Item = &ClkPathSelect> {
        self.clk_path_select.iter()
    }
    #[doc = "0x380..0x3c0 - Clock Root Select Register"]
    #[inline(always)]
    pub const fn clk_root_select(&self, n: usize) -> &ClkRootSelect {
        &self.clk_root_select[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x380..0x3c0 - Clock Root Select Register"]
    #[inline(always)]
    pub fn clk_root_select_iter(&self) -> impl Iterator<Item = &ClkRootSelect> {
        self.clk_root_select.iter()
    }
    #[doc = "0x500 - Clock selection register"]
    #[inline(always)]
    pub const fn clk_select(&self) -> &ClkSelect {
        &self.clk_select
    }
    #[doc = "0x504 - Timer Clock Control Register"]
    #[inline(always)]
    pub const fn clk_timer_ctl(&self) -> &ClkTimerCtl {
        &self.clk_timer_ctl
    }
    #[doc = "0x50c - ILO Configuration"]
    #[inline(always)]
    pub const fn clk_ilo_config(&self) -> &ClkIloConfig {
        &self.clk_ilo_config
    }
    #[doc = "0x510 - IMO Configuration"]
    #[inline(always)]
    pub const fn clk_imo_config(&self) -> &ClkImoConfig {
        &self.clk_imo_config
    }
    #[doc = "0x514 - Fast Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_fast(&self) -> &ClkOutputFast {
        &self.clk_output_fast
    }
    #[doc = "0x518 - Slow Clock Output Select Register"]
    #[inline(always)]
    pub const fn clk_output_slow(&self) -> &ClkOutputSlow {
        &self.clk_output_slow
    }
    #[doc = "0x51c - Clock Calibration Counter 1"]
    #[inline(always)]
    pub const fn clk_cal_cnt1(&self) -> &ClkCalCnt1 {
        &self.clk_cal_cnt1
    }
    #[doc = "0x520 - Clock Calibration Counter 2"]
    #[inline(always)]
    pub const fn clk_cal_cnt2(&self) -> &ClkCalCnt2 {
        &self.clk_cal_cnt2
    }
    #[doc = "0x52c - ECO Configuration Register"]
    #[inline(always)]
    pub const fn clk_eco_config(&self) -> &ClkEcoConfig {
        &self.clk_eco_config
    }
    #[doc = "0x530 - ECO Status Register"]
    #[inline(always)]
    pub const fn clk_eco_status(&self) -> &ClkEcoStatus {
        &self.clk_eco_status
    }
    #[doc = "0x53c - Precision ILO Configuration Register"]
    #[inline(always)]
    pub const fn clk_pilo_config(&self) -> &ClkPiloConfig {
        &self.clk_pilo_config
    }
    #[doc = "0x580 - FLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_fll_config(&self) -> &ClkFllConfig {
        &self.clk_fll_config
    }
    #[doc = "0x584 - FLL Configuration Register 2"]
    #[inline(always)]
    pub const fn clk_fll_config2(&self) -> &ClkFllConfig2 {
        &self.clk_fll_config2
    }
    #[doc = "0x588 - FLL Configuration Register 3"]
    #[inline(always)]
    pub const fn clk_fll_config3(&self) -> &ClkFllConfig3 {
        &self.clk_fll_config3
    }
    #[doc = "0x58c - FLL Configuration Register 4"]
    #[inline(always)]
    pub const fn clk_fll_config4(&self) -> &ClkFllConfig4 {
        &self.clk_fll_config4
    }
    #[doc = "0x590 - FLL Status Register"]
    #[inline(always)]
    pub const fn clk_fll_status(&self) -> &ClkFllStatus {
        &self.clk_fll_status
    }
    #[doc = "0x600..0x63c - PLL Configuration Register"]
    #[inline(always)]
    pub const fn clk_pll_config(&self, n: usize) -> &ClkPllConfig {
        &self.clk_pll_config[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x63c - PLL Configuration Register"]
    #[inline(always)]
    pub fn clk_pll_config_iter(&self) -> impl Iterator<Item = &ClkPllConfig> {
        self.clk_pll_config.iter()
    }
    #[doc = "0x640..0x67c - PLL Status Register"]
    #[inline(always)]
    pub const fn clk_pll_status(&self, n: usize) -> &ClkPllStatus {
        &self.clk_pll_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x640..0x67c - PLL Status Register"]
    #[inline(always)]
    pub fn clk_pll_status_iter(&self) -> impl Iterator<Item = &ClkPllStatus> {
        self.clk_pll_status.iter()
    }
    #[doc = "0x700 - SRSS Interrupt Register"]
    #[inline(always)]
    pub const fn srss_intr(&self) -> &SrssIntr {
        &self.srss_intr
    }
    #[doc = "0x704 - SRSS Interrupt Set Register"]
    #[inline(always)]
    pub const fn srss_intr_set(&self) -> &SrssIntrSet {
        &self.srss_intr_set
    }
    #[doc = "0x708 - SRSS Interrupt Mask Register"]
    #[inline(always)]
    pub const fn srss_intr_mask(&self) -> &SrssIntrMask {
        &self.srss_intr_mask
    }
    #[doc = "0x70c - SRSS Interrupt Masked Register"]
    #[inline(always)]
    pub const fn srss_intr_masked(&self) -> &SrssIntrMasked {
        &self.srss_intr_masked
    }
    #[doc = "0x710 - SRSS Interrupt Configuration Register"]
    #[inline(always)]
    pub const fn srss_intr_cfg(&self) -> &SrssIntrCfg {
        &self.srss_intr_cfg
    }
    #[doc = "0x800 - Reset Cause Observation Register"]
    #[inline(always)]
    pub const fn res_cause(&self) -> &ResCause {
        &self.res_cause
    }
    #[doc = "0x804 - Reset Cause Observation Register 2"]
    #[inline(always)]
    pub const fn res_cause2(&self) -> &ResCause2 {
        &self.res_cause2
    }
    #[doc = "0x7f00 - Reference Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_ref_ctl(&self) -> &PwrTrimRefCtl {
        &self.pwr_trim_ref_ctl
    }
    #[doc = "0x7f04 - BOD/OVP Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_bodovp_ctl(&self) -> &PwrTrimBodovpCtl {
        &self.pwr_trim_bodovp_ctl
    }
    #[doc = "0x7f08 - CCO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl(&self) -> &ClkTrimCcoCtl {
        &self.clk_trim_cco_ctl
    }
    #[doc = "0x7f0c - CCO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_cco_ctl2(&self) -> &ClkTrimCcoCtl2 {
        &self.clk_trim_cco_ctl2
    }
    #[doc = "0x7f30 - Wakeup Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_wake_ctl(&self) -> &PwrTrimWakeCtl {
        &self.pwr_trim_wake_ctl
    }
    #[doc = "0xff10 - LVD Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_lvd_ctl(&self) -> &PwrTrimLvdCtl {
        &self.pwr_trim_lvd_ctl
    }
    #[doc = "0xff18 - ILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_ilo_ctl(&self) -> &ClkTrimIloCtl {
        &self.clk_trim_ilo_ctl
    }
    #[doc = "0xff1c - Power System Trim Register"]
    #[inline(always)]
    pub const fn pwr_trim_pwrsys_ctl(&self) -> &PwrTrimPwrsysCtl {
        &self.pwr_trim_pwrsys_ctl
    }
    #[doc = "0xff20 - ECO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_eco_ctl(&self) -> &ClkTrimEcoCtl {
        &self.clk_trim_eco_ctl
    }
    #[doc = "0xff24 - PILO Trim Register"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl(&self) -> &ClkTrimPiloCtl {
        &self.clk_trim_pilo_ctl
    }
    #[doc = "0xff28 - PILO Trim Register 2"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl2(&self) -> &ClkTrimPiloCtl2 {
        &self.clk_trim_pilo_ctl2
    }
    #[doc = "0xff2c - PILO Trim Register 3"]
    #[inline(always)]
    pub const fn clk_trim_pilo_ctl3(&self) -> &ClkTrimPiloCtl3 {
        &self.clk_trim_pilo_ctl3
    }
}
#[doc = "PWR_CTL (rw) register accessor: Power Mode Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_ctl`]
module"]
#[doc(alias = "PWR_CTL")]
pub type PwrCtl = crate::Reg<pwr_ctl::PwrCtlSpec>;
#[doc = "Power Mode Control"]
pub mod pwr_ctl;
#[doc = "PWR_HIBERNATE (rw) register accessor: HIBERNATE Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_hibernate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_hibernate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_hibernate`]
module"]
#[doc(alias = "PWR_HIBERNATE")]
pub type PwrHibernate = crate::Reg<pwr_hibernate::PwrHibernateSpec>;
#[doc = "HIBERNATE Mode Register"]
pub mod pwr_hibernate;
#[doc = "PWR_LVD_CTL (rw) register accessor: Low Voltage Detector (LVD) Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_lvd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_lvd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_lvd_ctl`]
module"]
#[doc(alias = "PWR_LVD_CTL")]
pub type PwrLvdCtl = crate::Reg<pwr_lvd_ctl::PwrLvdCtlSpec>;
#[doc = "Low Voltage Detector (LVD) Configuration Register"]
pub mod pwr_lvd_ctl;
#[doc = "PWR_BUCK_CTL (rw) register accessor: Buck Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_buck_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_buck_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_buck_ctl`]
module"]
#[doc(alias = "PWR_BUCK_CTL")]
pub type PwrBuckCtl = crate::Reg<pwr_buck_ctl::PwrBuckCtlSpec>;
#[doc = "Buck Control Register"]
pub mod pwr_buck_ctl;
#[doc = "PWR_BUCK_CTL2 (rw) register accessor: Buck Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_buck_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_buck_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_buck_ctl2`]
module"]
#[doc(alias = "PWR_BUCK_CTL2")]
pub type PwrBuckCtl2 = crate::Reg<pwr_buck_ctl2::PwrBuckCtl2Spec>;
#[doc = "Buck Control Register 2"]
pub mod pwr_buck_ctl2;
#[doc = "PWR_LVD_STATUS (r) register accessor: Low Voltage Detector (LVD) Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_lvd_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_lvd_status`]
module"]
#[doc(alias = "PWR_LVD_STATUS")]
pub type PwrLvdStatus = crate::Reg<pwr_lvd_status::PwrLvdStatusSpec>;
#[doc = "Low Voltage Detector (LVD) Status Register"]
pub mod pwr_lvd_status;
#[doc = "PWR_HIB_DATA (rw) register accessor: HIBERNATE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_hib_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_hib_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_hib_data`]
module"]
#[doc(alias = "PWR_HIB_DATA")]
pub type PwrHibData = crate::Reg<pwr_hib_data::PwrHibDataSpec>;
#[doc = "HIBERNATE Data Register"]
pub mod pwr_hib_data;
#[doc = "WDT_CTL (rw) register accessor: Watchdog Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_ctl`]
module"]
#[doc(alias = "WDT_CTL")]
pub type WdtCtl = crate::Reg<wdt_ctl::WdtCtlSpec>;
#[doc = "Watchdog Counter Control Register"]
pub mod wdt_ctl;
#[doc = "WDT_CNT (rw) register accessor: Watchdog Counter Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_cnt`]
module"]
#[doc(alias = "WDT_CNT")]
pub type WdtCnt = crate::Reg<wdt_cnt::WdtCntSpec>;
#[doc = "Watchdog Counter Count Register"]
pub mod wdt_cnt;
#[doc = "WDT_MATCH (rw) register accessor: Watchdog Counter Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_match::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_match::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdt_match`]
module"]
#[doc(alias = "WDT_MATCH")]
pub type WdtMatch = crate::Reg<wdt_match::WdtMatchSpec>;
#[doc = "Watchdog Counter Match Register"]
pub mod wdt_match;
#[doc = "Multi-Counter Watchdog Timer"]
pub use self::mcwdt_struct::McwdtStruct;
#[doc = r"Cluster"]
#[doc = "Multi-Counter Watchdog Timer"]
pub mod mcwdt_struct;
#[doc = "CLK_DSI_SELECT (rw) register accessor: Clock DSI Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_dsi_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_dsi_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dsi_select`]
module"]
#[doc(alias = "CLK_DSI_SELECT")]
pub type ClkDsiSelect = crate::Reg<clk_dsi_select::ClkDsiSelectSpec>;
#[doc = "Clock DSI Select Register"]
pub mod clk_dsi_select;
#[doc = "CLK_PATH_SELECT (rw) register accessor: Clock Path Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_path_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_path_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_path_select`]
module"]
#[doc(alias = "CLK_PATH_SELECT")]
pub type ClkPathSelect = crate::Reg<clk_path_select::ClkPathSelectSpec>;
#[doc = "Clock Path Select Register"]
pub mod clk_path_select;
#[doc = "CLK_ROOT_SELECT (rw) register accessor: Clock Root Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_root_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_root_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_root_select`]
module"]
#[doc(alias = "CLK_ROOT_SELECT")]
pub type ClkRootSelect = crate::Reg<clk_root_select::ClkRootSelectSpec>;
#[doc = "Clock Root Select Register"]
pub mod clk_root_select;
#[doc = "CLK_SELECT (rw) register accessor: Clock selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_select::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_select::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_select`]
module"]
#[doc(alias = "CLK_SELECT")]
pub type ClkSelect = crate::Reg<clk_select::ClkSelectSpec>;
#[doc = "Clock selection register"]
pub mod clk_select;
#[doc = "CLK_TIMER_CTL (rw) register accessor: Timer Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_timer_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_timer_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_timer_ctl`]
module"]
#[doc(alias = "CLK_TIMER_CTL")]
pub type ClkTimerCtl = crate::Reg<clk_timer_ctl::ClkTimerCtlSpec>;
#[doc = "Timer Clock Control Register"]
pub mod clk_timer_ctl;
#[doc = "CLK_ILO_CONFIG (rw) register accessor: ILO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_ilo_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_ilo_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ilo_config`]
module"]
#[doc(alias = "CLK_ILO_CONFIG")]
pub type ClkIloConfig = crate::Reg<clk_ilo_config::ClkIloConfigSpec>;
#[doc = "ILO Configuration"]
pub mod clk_ilo_config;
#[doc = "CLK_IMO_CONFIG (rw) register accessor: IMO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_imo_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_imo_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_imo_config`]
module"]
#[doc(alias = "CLK_IMO_CONFIG")]
pub type ClkImoConfig = crate::Reg<clk_imo_config::ClkImoConfigSpec>;
#[doc = "IMO Configuration"]
pub mod clk_imo_config;
#[doc = "CLK_OUTPUT_FAST (rw) register accessor: Fast Clock Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_output_fast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_output_fast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_output_fast`]
module"]
#[doc(alias = "CLK_OUTPUT_FAST")]
pub type ClkOutputFast = crate::Reg<clk_output_fast::ClkOutputFastSpec>;
#[doc = "Fast Clock Output Select Register"]
pub mod clk_output_fast;
#[doc = "CLK_OUTPUT_SLOW (rw) register accessor: Slow Clock Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_output_slow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_output_slow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_output_slow`]
module"]
#[doc(alias = "CLK_OUTPUT_SLOW")]
pub type ClkOutputSlow = crate::Reg<clk_output_slow::ClkOutputSlowSpec>;
#[doc = "Slow Clock Output Select Register"]
pub mod clk_output_slow;
#[doc = "CLK_CAL_CNT1 (rw) register accessor: Clock Calibration Counter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cal_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_cal_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cal_cnt1`]
module"]
#[doc(alias = "CLK_CAL_CNT1")]
pub type ClkCalCnt1 = crate::Reg<clk_cal_cnt1::ClkCalCnt1Spec>;
#[doc = "Clock Calibration Counter 1"]
pub mod clk_cal_cnt1;
#[doc = "CLK_CAL_CNT2 (r) register accessor: Clock Calibration Counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_cal_cnt2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cal_cnt2`]
module"]
#[doc(alias = "CLK_CAL_CNT2")]
pub type ClkCalCnt2 = crate::Reg<clk_cal_cnt2::ClkCalCnt2Spec>;
#[doc = "Clock Calibration Counter 2"]
pub mod clk_cal_cnt2;
#[doc = "CLK_ECO_CONFIG (rw) register accessor: ECO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_eco_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_eco_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_eco_config`]
module"]
#[doc(alias = "CLK_ECO_CONFIG")]
pub type ClkEcoConfig = crate::Reg<clk_eco_config::ClkEcoConfigSpec>;
#[doc = "ECO Configuration Register"]
pub mod clk_eco_config;
#[doc = "CLK_ECO_STATUS (r) register accessor: ECO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_eco_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_eco_status`]
module"]
#[doc(alias = "CLK_ECO_STATUS")]
pub type ClkEcoStatus = crate::Reg<clk_eco_status::ClkEcoStatusSpec>;
#[doc = "ECO Status Register"]
pub mod clk_eco_status;
#[doc = "CLK_PILO_CONFIG (rw) register accessor: Precision ILO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pilo_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pilo_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pilo_config`]
module"]
#[doc(alias = "CLK_PILO_CONFIG")]
pub type ClkPiloConfig = crate::Reg<clk_pilo_config::ClkPiloConfigSpec>;
#[doc = "Precision ILO Configuration Register"]
pub mod clk_pilo_config;
#[doc = "CLK_FLL_CONFIG (rw) register accessor: FLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config`]
module"]
#[doc(alias = "CLK_FLL_CONFIG")]
pub type ClkFllConfig = crate::Reg<clk_fll_config::ClkFllConfigSpec>;
#[doc = "FLL Configuration Register"]
pub mod clk_fll_config;
#[doc = "CLK_FLL_CONFIG2 (rw) register accessor: FLL Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config2`]
module"]
#[doc(alias = "CLK_FLL_CONFIG2")]
pub type ClkFllConfig2 = crate::Reg<clk_fll_config2::ClkFllConfig2Spec>;
#[doc = "FLL Configuration Register 2"]
pub mod clk_fll_config2;
#[doc = "CLK_FLL_CONFIG3 (rw) register accessor: FLL Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config3`]
module"]
#[doc(alias = "CLK_FLL_CONFIG3")]
pub type ClkFllConfig3 = crate::Reg<clk_fll_config3::ClkFllConfig3Spec>;
#[doc = "FLL Configuration Register 3"]
pub mod clk_fll_config3;
#[doc = "CLK_FLL_CONFIG4 (rw) register accessor: FLL Configuration Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_config4`]
module"]
#[doc(alias = "CLK_FLL_CONFIG4")]
pub type ClkFllConfig4 = crate::Reg<clk_fll_config4::ClkFllConfig4Spec>;
#[doc = "FLL Configuration Register 4"]
pub mod clk_fll_config4;
#[doc = "CLK_FLL_STATUS (rw) register accessor: FLL Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_fll_status`]
module"]
#[doc(alias = "CLK_FLL_STATUS")]
pub type ClkFllStatus = crate::Reg<clk_fll_status::ClkFllStatusSpec>;
#[doc = "FLL Status Register"]
pub mod clk_fll_status;
#[doc = "CLK_PLL_CONFIG (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll_config`]
module"]
#[doc(alias = "CLK_PLL_CONFIG")]
pub type ClkPllConfig = crate::Reg<clk_pll_config::ClkPllConfigSpec>;
#[doc = "PLL Configuration Register"]
pub mod clk_pll_config;
#[doc = "CLK_PLL_STATUS (rw) register accessor: PLL Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll_status`]
module"]
#[doc(alias = "CLK_PLL_STATUS")]
pub type ClkPllStatus = crate::Reg<clk_pll_status::ClkPllStatusSpec>;
#[doc = "PLL Status Register"]
pub mod clk_pll_status;
#[doc = "SRSS_INTR (rw) register accessor: SRSS Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr`]
module"]
#[doc(alias = "SRSS_INTR")]
pub type SrssIntr = crate::Reg<srss_intr::SrssIntrSpec>;
#[doc = "SRSS Interrupt Register"]
pub mod srss_intr;
#[doc = "SRSS_INTR_SET (rw) register accessor: SRSS Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_set`]
module"]
#[doc(alias = "SRSS_INTR_SET")]
pub type SrssIntrSet = crate::Reg<srss_intr_set::SrssIntrSetSpec>;
#[doc = "SRSS Interrupt Set Register"]
pub mod srss_intr_set;
#[doc = "SRSS_INTR_MASK (rw) register accessor: SRSS Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_mask`]
module"]
#[doc(alias = "SRSS_INTR_MASK")]
pub type SrssIntrMask = crate::Reg<srss_intr_mask::SrssIntrMaskSpec>;
#[doc = "SRSS Interrupt Mask Register"]
pub mod srss_intr_mask;
#[doc = "SRSS_INTR_MASKED (r) register accessor: SRSS Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_masked`]
module"]
#[doc(alias = "SRSS_INTR_MASKED")]
pub type SrssIntrMasked = crate::Reg<srss_intr_masked::SrssIntrMaskedSpec>;
#[doc = "SRSS Interrupt Masked Register"]
pub mod srss_intr_masked;
#[doc = "SRSS_INTR_CFG (rw) register accessor: SRSS Interrupt Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srss_intr_cfg`]
module"]
#[doc(alias = "SRSS_INTR_CFG")]
pub type SrssIntrCfg = crate::Reg<srss_intr_cfg::SrssIntrCfgSpec>;
#[doc = "SRSS Interrupt Configuration Register"]
pub mod srss_intr_cfg;
#[doc = "RES_CAUSE (rw) register accessor: Reset Cause Observation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`res_cause::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res_cause::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res_cause`]
module"]
#[doc(alias = "RES_CAUSE")]
pub type ResCause = crate::Reg<res_cause::ResCauseSpec>;
#[doc = "Reset Cause Observation Register"]
pub mod res_cause;
#[doc = "RES_CAUSE2 (rw) register accessor: Reset Cause Observation Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res_cause2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res_cause2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res_cause2`]
module"]
#[doc(alias = "RES_CAUSE2")]
pub type ResCause2 = crate::Reg<res_cause2::ResCause2Spec>;
#[doc = "Reset Cause Observation Register 2"]
pub mod res_cause2;
#[doc = "PWR_TRIM_REF_CTL (rw) register accessor: Reference Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_ref_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_ref_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_ref_ctl`]
module"]
#[doc(alias = "PWR_TRIM_REF_CTL")]
pub type PwrTrimRefCtl = crate::Reg<pwr_trim_ref_ctl::PwrTrimRefCtlSpec>;
#[doc = "Reference Trim Register"]
pub mod pwr_trim_ref_ctl;
#[doc = "PWR_TRIM_BODOVP_CTL (rw) register accessor: BOD/OVP Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_bodovp_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_bodovp_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_bodovp_ctl`]
module"]
#[doc(alias = "PWR_TRIM_BODOVP_CTL")]
pub type PwrTrimBodovpCtl = crate::Reg<pwr_trim_bodovp_ctl::PwrTrimBodovpCtlSpec>;
#[doc = "BOD/OVP Trim Register"]
pub mod pwr_trim_bodovp_ctl;
#[doc = "CLK_TRIM_CCO_CTL (rw) register accessor: CCO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_cco_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_cco_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_cco_ctl`]
module"]
#[doc(alias = "CLK_TRIM_CCO_CTL")]
pub type ClkTrimCcoCtl = crate::Reg<clk_trim_cco_ctl::ClkTrimCcoCtlSpec>;
#[doc = "CCO Trim Register"]
pub mod clk_trim_cco_ctl;
#[doc = "CLK_TRIM_CCO_CTL2 (rw) register accessor: CCO Trim Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_cco_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_cco_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_cco_ctl2`]
module"]
#[doc(alias = "CLK_TRIM_CCO_CTL2")]
pub type ClkTrimCcoCtl2 = crate::Reg<clk_trim_cco_ctl2::ClkTrimCcoCtl2Spec>;
#[doc = "CCO Trim Register 2"]
pub mod clk_trim_cco_ctl2;
#[doc = "PWR_TRIM_WAKE_CTL (rw) register accessor: Wakeup Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_wake_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_wake_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_wake_ctl`]
module"]
#[doc(alias = "PWR_TRIM_WAKE_CTL")]
pub type PwrTrimWakeCtl = crate::Reg<pwr_trim_wake_ctl::PwrTrimWakeCtlSpec>;
#[doc = "Wakeup Trim Register"]
pub mod pwr_trim_wake_ctl;
#[doc = "PWR_TRIM_LVD_CTL (rw) register accessor: LVD Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_lvd_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_lvd_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_lvd_ctl`]
module"]
#[doc(alias = "PWR_TRIM_LVD_CTL")]
pub type PwrTrimLvdCtl = crate::Reg<pwr_trim_lvd_ctl::PwrTrimLvdCtlSpec>;
#[doc = "LVD Trim Register"]
pub mod pwr_trim_lvd_ctl;
#[doc = "CLK_TRIM_ILO_CTL (rw) register accessor: ILO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_ilo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_ilo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_ilo_ctl`]
module"]
#[doc(alias = "CLK_TRIM_ILO_CTL")]
pub type ClkTrimIloCtl = crate::Reg<clk_trim_ilo_ctl::ClkTrimIloCtlSpec>;
#[doc = "ILO Trim Register"]
pub mod clk_trim_ilo_ctl;
#[doc = "PWR_TRIM_PWRSYS_CTL (rw) register accessor: Power System Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_pwrsys_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_pwrsys_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_trim_pwrsys_ctl`]
module"]
#[doc(alias = "PWR_TRIM_PWRSYS_CTL")]
pub type PwrTrimPwrsysCtl = crate::Reg<pwr_trim_pwrsys_ctl::PwrTrimPwrsysCtlSpec>;
#[doc = "Power System Trim Register"]
pub mod pwr_trim_pwrsys_ctl;
#[doc = "CLK_TRIM_ECO_CTL (rw) register accessor: ECO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_eco_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_eco_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_eco_ctl`]
module"]
#[doc(alias = "CLK_TRIM_ECO_CTL")]
pub type ClkTrimEcoCtl = crate::Reg<clk_trim_eco_ctl::ClkTrimEcoCtlSpec>;
#[doc = "ECO Trim Register"]
pub mod clk_trim_eco_ctl;
#[doc = "CLK_TRIM_PILO_CTL (rw) register accessor: PILO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl`]
module"]
#[doc(alias = "CLK_TRIM_PILO_CTL")]
pub type ClkTrimPiloCtl = crate::Reg<clk_trim_pilo_ctl::ClkTrimPiloCtlSpec>;
#[doc = "PILO Trim Register"]
pub mod clk_trim_pilo_ctl;
#[doc = "CLK_TRIM_PILO_CTL2 (rw) register accessor: PILO Trim Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl2`]
module"]
#[doc(alias = "CLK_TRIM_PILO_CTL2")]
pub type ClkTrimPiloCtl2 = crate::Reg<clk_trim_pilo_ctl2::ClkTrimPiloCtl2Spec>;
#[doc = "PILO Trim Register 2"]
pub mod clk_trim_pilo_ctl2;
#[doc = "CLK_TRIM_PILO_CTL3 (rw) register accessor: PILO Trim Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_trim_pilo_ctl3`]
module"]
#[doc(alias = "CLK_TRIM_PILO_CTL3")]
pub type ClkTrimPiloCtl3 = crate::Reg<clk_trim_pilo_ctl3::ClkTrimPiloCtl3Spec>;
#[doc = "PILO Trim Register 3"]
pub mod clk_trim_pilo_ctl3;
