#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    identity: Identity,
    cm4_status: Cm4Status,
    cm4_clock_ctl: Cm4ClockCtl,
    cm4_ctl: Cm4Ctl,
    _reserved4: [u8; 0xf0],
    cm4_int0_status: Cm4Int0Status,
    cm4_int1_status: Cm4Int1Status,
    cm4_int2_status: Cm4Int2Status,
    cm4_int3_status: Cm4Int3Status,
    cm4_int4_status: Cm4Int4Status,
    cm4_int5_status: Cm4Int5Status,
    cm4_int6_status: Cm4Int6Status,
    cm4_int7_status: Cm4Int7Status,
    _reserved12: [u8; 0xe0],
    cm4_vector_table_base: Cm4VectorTableBase,
    _reserved13: [u8; 0x3c],
    cm4_nmi_ctl: [Cm4NmiCtl; 4],
    _reserved14: [u8; 0xb0],
    udb_pwr_ctl: UdbPwrCtl,
    udb_pwr_delay_ctl: UdbPwrDelayCtl,
    _reserved16: [u8; 0x0cf8],
    cm0_ctl: Cm0Ctl,
    cm0_status: Cm0Status,
    cm0_clock_ctl: Cm0ClockCtl,
    _reserved19: [u8; 0xf4],
    cm0_int0_status: Cm0Int0Status,
    cm0_int1_status: Cm0Int1Status,
    cm0_int2_status: Cm0Int2Status,
    cm0_int3_status: Cm0Int3Status,
    cm0_int4_status: Cm0Int4Status,
    cm0_int5_status: Cm0Int5Status,
    cm0_int6_status: Cm0Int6Status,
    cm0_int7_status: Cm0Int7Status,
    cm0_vector_table_base: Cm0VectorTableBase,
    _reserved28: [u8; 0x1c],
    cm0_nmi_ctl: [Cm0NmiCtl; 4],
    _reserved29: [u8; 0xb0],
    cm4_pwr_ctl: Cm4PwrCtl,
    cm4_pwr_delay_ctl: Cm4PwrDelayCtl,
    _reserved31: [u8; 0xf8],
    ram0_ctl0: Ram0Ctl0,
    ram0_status: Ram0Status,
    _reserved33: [u8; 0x38],
    ram0_pwr_macro_ctl: [Ram0PwrMacroCtl; 16],
    ram1_ctl0: Ram1Ctl0,
    ram1_status: Ram1Status,
    ram1_pwr_ctl: Ram1PwrCtl,
    _reserved37: [u8; 0x14],
    ram2_ctl0: Ram2Ctl0,
    ram2_status: Ram2Status,
    ram2_pwr_ctl: Ram2PwrCtl,
    _reserved40: [u8; 0x14],
    ram_pwr_delay_ctl: RamPwrDelayCtl,
    rom_ctl: RomCtl,
    ecc_ctl: EccCtl,
    _reserved43: [u8; 0x34],
    product_id: ProductId,
    _reserved44: [u8; 0x0c],
    dp_status: DpStatus,
    ap_ctl: ApCtl,
    _reserved46: [u8; 0xe8],
    buff_ctl: BuffCtl,
    _reserved47: [u8; 0xfc],
    systick_ctl: SystickCtl,
    _reserved48: [u8; 0x0100],
    mbist_stat: MbistStat,
    _reserved49: [u8; 0xf8],
    cal_sup_set: CalSupSet,
    cal_sup_clr: CalSupClr,
    _reserved51: [u8; 0x07f8],
    cm0_pc_ctl: Cm0PcCtl,
    _reserved52: [u8; 0x3c],
    cm0_pc0_handler: Cm0Pc0Handler,
    cm0_pc1_handler: Cm0Pc1Handler,
    cm0_pc2_handler: Cm0Pc2Handler,
    cm0_pc3_handler: Cm0Pc3Handler,
    _reserved56: [u8; 0x74],
    protection: Protection,
    _reserved57: [u8; 0x38],
    trim_rom_ctl: TrimRomCtl,
    trim_ram_ctl: TrimRamCtl,
    _reserved59: [u8; 0x5ef8],
    cm0_system_int_ctl: [Cm0SystemIntCtl; 1023],
    _reserved60: [u8; 0x1004],
    cm4_system_int_ctl: [Cm4SystemIntCtl; 1023],
}
impl RegisterBlock {
    #[doc = "0x00 - Identity"]
    #[inline(always)]
    pub const fn identity(&self) -> &Identity {
        &self.identity
    }
    #[doc = "0x04 - CM4 status"]
    #[inline(always)]
    pub const fn cm4_status(&self) -> &Cm4Status {
        &self.cm4_status
    }
    #[doc = "0x08 - CM4 clock control"]
    #[inline(always)]
    pub const fn cm4_clock_ctl(&self) -> &Cm4ClockCtl {
        &self.cm4_clock_ctl
    }
    #[doc = "0x0c - CM4 control"]
    #[inline(always)]
    pub const fn cm4_ctl(&self) -> &Cm4Ctl {
        &self.cm4_ctl
    }
    #[doc = "0x100 - CM4 interrupt 0 status"]
    #[inline(always)]
    pub const fn cm4_int0_status(&self) -> &Cm4Int0Status {
        &self.cm4_int0_status
    }
    #[doc = "0x104 - CM4 interrupt 1 status"]
    #[inline(always)]
    pub const fn cm4_int1_status(&self) -> &Cm4Int1Status {
        &self.cm4_int1_status
    }
    #[doc = "0x108 - CM4 interrupt 2 status"]
    #[inline(always)]
    pub const fn cm4_int2_status(&self) -> &Cm4Int2Status {
        &self.cm4_int2_status
    }
    #[doc = "0x10c - CM4 interrupt 3 status"]
    #[inline(always)]
    pub const fn cm4_int3_status(&self) -> &Cm4Int3Status {
        &self.cm4_int3_status
    }
    #[doc = "0x110 - CM4 interrupt 4 status"]
    #[inline(always)]
    pub const fn cm4_int4_status(&self) -> &Cm4Int4Status {
        &self.cm4_int4_status
    }
    #[doc = "0x114 - CM4 interrupt 5 status"]
    #[inline(always)]
    pub const fn cm4_int5_status(&self) -> &Cm4Int5Status {
        &self.cm4_int5_status
    }
    #[doc = "0x118 - CM4 interrupt 6 status"]
    #[inline(always)]
    pub const fn cm4_int6_status(&self) -> &Cm4Int6Status {
        &self.cm4_int6_status
    }
    #[doc = "0x11c - CM4 interrupt 7 status"]
    #[inline(always)]
    pub const fn cm4_int7_status(&self) -> &Cm4Int7Status {
        &self.cm4_int7_status
    }
    #[doc = "0x200 - CM4 vector table base"]
    #[inline(always)]
    pub const fn cm4_vector_table_base(&self) -> &Cm4VectorTableBase {
        &self.cm4_vector_table_base
    }
    #[doc = "0x240..0x250 - CM4 NMI control"]
    #[inline(always)]
    pub const fn cm4_nmi_ctl(&self, n: usize) -> &Cm4NmiCtl {
        &self.cm4_nmi_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x250 - CM4 NMI control"]
    #[inline(always)]
    pub fn cm4_nmi_ctl_iter(&self) -> impl Iterator<Item = &Cm4NmiCtl> {
        self.cm4_nmi_ctl.iter()
    }
    #[doc = "0x300 - UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_ctl(&self) -> &UdbPwrCtl {
        &self.udb_pwr_ctl
    }
    #[doc = "0x304 - UDB power control"]
    #[inline(always)]
    pub const fn udb_pwr_delay_ctl(&self) -> &UdbPwrDelayCtl {
        &self.udb_pwr_delay_ctl
    }
    #[doc = "0x1000 - CM0+ control"]
    #[inline(always)]
    pub const fn cm0_ctl(&self) -> &Cm0Ctl {
        &self.cm0_ctl
    }
    #[doc = "0x1004 - CM0+ status"]
    #[inline(always)]
    pub const fn cm0_status(&self) -> &Cm0Status {
        &self.cm0_status
    }
    #[doc = "0x1008 - CM0+ clock control"]
    #[inline(always)]
    pub const fn cm0_clock_ctl(&self) -> &Cm0ClockCtl {
        &self.cm0_clock_ctl
    }
    #[doc = "0x1100 - CM0+ interrupt 0 status"]
    #[inline(always)]
    pub const fn cm0_int0_status(&self) -> &Cm0Int0Status {
        &self.cm0_int0_status
    }
    #[doc = "0x1104 - CM0+ interrupt 1 status"]
    #[inline(always)]
    pub const fn cm0_int1_status(&self) -> &Cm0Int1Status {
        &self.cm0_int1_status
    }
    #[doc = "0x1108 - CM0+ interrupt 2 status"]
    #[inline(always)]
    pub const fn cm0_int2_status(&self) -> &Cm0Int2Status {
        &self.cm0_int2_status
    }
    #[doc = "0x110c - CM0+ interrupt 3 status"]
    #[inline(always)]
    pub const fn cm0_int3_status(&self) -> &Cm0Int3Status {
        &self.cm0_int3_status
    }
    #[doc = "0x1110 - CM0+ interrupt 4 status"]
    #[inline(always)]
    pub const fn cm0_int4_status(&self) -> &Cm0Int4Status {
        &self.cm0_int4_status
    }
    #[doc = "0x1114 - CM0+ interrupt 5 status"]
    #[inline(always)]
    pub const fn cm0_int5_status(&self) -> &Cm0Int5Status {
        &self.cm0_int5_status
    }
    #[doc = "0x1118 - CM0+ interrupt 6 status"]
    #[inline(always)]
    pub const fn cm0_int6_status(&self) -> &Cm0Int6Status {
        &self.cm0_int6_status
    }
    #[doc = "0x111c - CM0+ interrupt 7 status"]
    #[inline(always)]
    pub const fn cm0_int7_status(&self) -> &Cm0Int7Status {
        &self.cm0_int7_status
    }
    #[doc = "0x1120 - CM0+ vector table base"]
    #[inline(always)]
    pub const fn cm0_vector_table_base(&self) -> &Cm0VectorTableBase {
        &self.cm0_vector_table_base
    }
    #[doc = "0x1140..0x1150 - CM0+ NMI control"]
    #[inline(always)]
    pub const fn cm0_nmi_ctl(&self, n: usize) -> &Cm0NmiCtl {
        &self.cm0_nmi_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1140..0x1150 - CM0+ NMI control"]
    #[inline(always)]
    pub fn cm0_nmi_ctl_iter(&self) -> impl Iterator<Item = &Cm0NmiCtl> {
        self.cm0_nmi_ctl.iter()
    }
    #[doc = "0x1200 - CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_ctl(&self) -> &Cm4PwrCtl {
        &self.cm4_pwr_ctl
    }
    #[doc = "0x1204 - CM4 power control"]
    #[inline(always)]
    pub const fn cm4_pwr_delay_ctl(&self) -> &Cm4PwrDelayCtl {
        &self.cm4_pwr_delay_ctl
    }
    #[doc = "0x1300 - RAM 0 control"]
    #[inline(always)]
    pub const fn ram0_ctl0(&self) -> &Ram0Ctl0 {
        &self.ram0_ctl0
    }
    #[doc = "0x1304 - RAM 0 status"]
    #[inline(always)]
    pub const fn ram0_status(&self) -> &Ram0Status {
        &self.ram0_status
    }
    #[doc = "0x1340..0x1380 - RAM 0 power control"]
    #[inline(always)]
    pub const fn ram0_pwr_macro_ctl(&self, n: usize) -> &Ram0PwrMacroCtl {
        &self.ram0_pwr_macro_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1340..0x1380 - RAM 0 power control"]
    #[inline(always)]
    pub fn ram0_pwr_macro_ctl_iter(&self) -> impl Iterator<Item = &Ram0PwrMacroCtl> {
        self.ram0_pwr_macro_ctl.iter()
    }
    #[doc = "0x1380 - RAM 1 control"]
    #[inline(always)]
    pub const fn ram1_ctl0(&self) -> &Ram1Ctl0 {
        &self.ram1_ctl0
    }
    #[doc = "0x1384 - RAM 1 status"]
    #[inline(always)]
    pub const fn ram1_status(&self) -> &Ram1Status {
        &self.ram1_status
    }
    #[doc = "0x1388 - RAM 1 power control"]
    #[inline(always)]
    pub const fn ram1_pwr_ctl(&self) -> &Ram1PwrCtl {
        &self.ram1_pwr_ctl
    }
    #[doc = "0x13a0 - RAM 2 control"]
    #[inline(always)]
    pub const fn ram2_ctl0(&self) -> &Ram2Ctl0 {
        &self.ram2_ctl0
    }
    #[doc = "0x13a4 - RAM 2 status"]
    #[inline(always)]
    pub const fn ram2_status(&self) -> &Ram2Status {
        &self.ram2_status
    }
    #[doc = "0x13a8 - RAM 2 power control"]
    #[inline(always)]
    pub const fn ram2_pwr_ctl(&self) -> &Ram2PwrCtl {
        &self.ram2_pwr_ctl
    }
    #[doc = "0x13c0 - Power up delay used for all SRAM power domains"]
    #[inline(always)]
    pub const fn ram_pwr_delay_ctl(&self) -> &RamPwrDelayCtl {
        &self.ram_pwr_delay_ctl
    }
    #[doc = "0x13c4 - ROM control"]
    #[inline(always)]
    pub const fn rom_ctl(&self) -> &RomCtl {
        &self.rom_ctl
    }
    #[doc = "0x13c8 - ECC control"]
    #[inline(always)]
    pub const fn ecc_ctl(&self) -> &EccCtl {
        &self.ecc_ctl
    }
    #[doc = "0x1400 - Product identifier and version (same as CoreSight RomTables)"]
    #[inline(always)]
    pub const fn product_id(&self) -> &ProductId {
        &self.product_id
    }
    #[doc = "0x1410 - Debug port status"]
    #[inline(always)]
    pub const fn dp_status(&self) -> &DpStatus {
        &self.dp_status
    }
    #[doc = "0x1414 - Access port control"]
    #[inline(always)]
    pub const fn ap_ctl(&self) -> &ApCtl {
        &self.ap_ctl
    }
    #[doc = "0x1500 - Buffer control"]
    #[inline(always)]
    pub const fn buff_ctl(&self) -> &BuffCtl {
        &self.buff_ctl
    }
    #[doc = "0x1600 - SysTick timer control"]
    #[inline(always)]
    pub const fn systick_ctl(&self) -> &SystickCtl {
        &self.systick_ctl
    }
    #[doc = "0x1704 - Memory BIST status"]
    #[inline(always)]
    pub const fn mbist_stat(&self) -> &MbistStat {
        &self.mbist_stat
    }
    #[doc = "0x1800 - Calibration support set and read"]
    #[inline(always)]
    pub const fn cal_sup_set(&self) -> &CalSupSet {
        &self.cal_sup_set
    }
    #[doc = "0x1804 - Calibration support clear and reset"]
    #[inline(always)]
    pub const fn cal_sup_clr(&self) -> &CalSupClr {
        &self.cal_sup_clr
    }
    #[doc = "0x2000 - CM0+ protection context control"]
    #[inline(always)]
    pub const fn cm0_pc_ctl(&self) -> &Cm0PcCtl {
        &self.cm0_pc_ctl
    }
    #[doc = "0x2040 - CM0+ protection context 0 handler"]
    #[inline(always)]
    pub const fn cm0_pc0_handler(&self) -> &Cm0Pc0Handler {
        &self.cm0_pc0_handler
    }
    #[doc = "0x2044 - CM0+ protection context 1 handler"]
    #[inline(always)]
    pub const fn cm0_pc1_handler(&self) -> &Cm0Pc1Handler {
        &self.cm0_pc1_handler
    }
    #[doc = "0x2048 - CM0+ protection context 2 handler"]
    #[inline(always)]
    pub const fn cm0_pc2_handler(&self) -> &Cm0Pc2Handler {
        &self.cm0_pc2_handler
    }
    #[doc = "0x204c - CM0+ protection context 3 handler"]
    #[inline(always)]
    pub const fn cm0_pc3_handler(&self) -> &Cm0Pc3Handler {
        &self.cm0_pc3_handler
    }
    #[doc = "0x20c4 - Protection status"]
    #[inline(always)]
    pub const fn protection(&self) -> &Protection {
        &self.protection
    }
    #[doc = "0x2100 - ROM trim control"]
    #[inline(always)]
    pub const fn trim_rom_ctl(&self) -> &TrimRomCtl {
        &self.trim_rom_ctl
    }
    #[doc = "0x2104 - RAM trim control"]
    #[inline(always)]
    pub const fn trim_ram_ctl(&self) -> &TrimRamCtl {
        &self.trim_ram_ctl
    }
    #[doc = "0x8000..0x8ffc - CM0+ system interrupt control"]
    #[inline(always)]
    pub const fn cm0_system_int_ctl(&self, n: usize) -> &Cm0SystemIntCtl {
        &self.cm0_system_int_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8000..0x8ffc - CM0+ system interrupt control"]
    #[inline(always)]
    pub fn cm0_system_int_ctl_iter(&self) -> impl Iterator<Item = &Cm0SystemIntCtl> {
        self.cm0_system_int_ctl.iter()
    }
    #[doc = "0xa000..0xaffc - CM4 system interrupt control"]
    #[inline(always)]
    pub const fn cm4_system_int_ctl(&self, n: usize) -> &Cm4SystemIntCtl {
        &self.cm4_system_int_ctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa000..0xaffc - CM4 system interrupt control"]
    #[inline(always)]
    pub fn cm4_system_int_ctl_iter(&self) -> impl Iterator<Item = &Cm4SystemIntCtl> {
        self.cm4_system_int_ctl.iter()
    }
}
#[doc = "IDENTITY (r) register accessor: Identity\n\nYou can [`read`](crate::Reg::read) this register and get [`identity::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@identity`]
module"]
#[doc(alias = "IDENTITY")]
pub type Identity = crate::Reg<identity::IdentitySpec>;
#[doc = "Identity"]
pub mod identity;
#[doc = "CM4_STATUS (r) register accessor: CM4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_status`]
module"]
#[doc(alias = "CM4_STATUS")]
pub type Cm4Status = crate::Reg<cm4_status::Cm4StatusSpec>;
#[doc = "CM4 status"]
pub mod cm4_status;
#[doc = "CM4_CLOCK_CTL (rw) register accessor: CM4 clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_clock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_clock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_clock_ctl`]
module"]
#[doc(alias = "CM4_CLOCK_CTL")]
pub type Cm4ClockCtl = crate::Reg<cm4_clock_ctl::Cm4ClockCtlSpec>;
#[doc = "CM4 clock control"]
pub mod cm4_clock_ctl;
#[doc = "CM4_CTL (rw) register accessor: CM4 control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_ctl`]
module"]
#[doc(alias = "CM4_CTL")]
pub type Cm4Ctl = crate::Reg<cm4_ctl::Cm4CtlSpec>;
#[doc = "CM4 control"]
pub mod cm4_ctl;
#[doc = "CM4_INT0_STATUS (r) register accessor: CM4 interrupt 0 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int0_status`]
module"]
#[doc(alias = "CM4_INT0_STATUS")]
pub type Cm4Int0Status = crate::Reg<cm4_int0_status::Cm4Int0StatusSpec>;
#[doc = "CM4 interrupt 0 status"]
pub mod cm4_int0_status;
#[doc = "CM4_INT1_STATUS (r) register accessor: CM4 interrupt 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int1_status`]
module"]
#[doc(alias = "CM4_INT1_STATUS")]
pub type Cm4Int1Status = crate::Reg<cm4_int1_status::Cm4Int1StatusSpec>;
#[doc = "CM4 interrupt 1 status"]
pub mod cm4_int1_status;
#[doc = "CM4_INT2_STATUS (r) register accessor: CM4 interrupt 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int2_status`]
module"]
#[doc(alias = "CM4_INT2_STATUS")]
pub type Cm4Int2Status = crate::Reg<cm4_int2_status::Cm4Int2StatusSpec>;
#[doc = "CM4 interrupt 2 status"]
pub mod cm4_int2_status;
#[doc = "CM4_INT3_STATUS (r) register accessor: CM4 interrupt 3 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int3_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int3_status`]
module"]
#[doc(alias = "CM4_INT3_STATUS")]
pub type Cm4Int3Status = crate::Reg<cm4_int3_status::Cm4Int3StatusSpec>;
#[doc = "CM4 interrupt 3 status"]
pub mod cm4_int3_status;
#[doc = "CM4_INT4_STATUS (r) register accessor: CM4 interrupt 4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int4_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int4_status`]
module"]
#[doc(alias = "CM4_INT4_STATUS")]
pub type Cm4Int4Status = crate::Reg<cm4_int4_status::Cm4Int4StatusSpec>;
#[doc = "CM4 interrupt 4 status"]
pub mod cm4_int4_status;
#[doc = "CM4_INT5_STATUS (r) register accessor: CM4 interrupt 5 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int5_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int5_status`]
module"]
#[doc(alias = "CM4_INT5_STATUS")]
pub type Cm4Int5Status = crate::Reg<cm4_int5_status::Cm4Int5StatusSpec>;
#[doc = "CM4 interrupt 5 status"]
pub mod cm4_int5_status;
#[doc = "CM4_INT6_STATUS (r) register accessor: CM4 interrupt 6 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int6_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int6_status`]
module"]
#[doc(alias = "CM4_INT6_STATUS")]
pub type Cm4Int6Status = crate::Reg<cm4_int6_status::Cm4Int6StatusSpec>;
#[doc = "CM4 interrupt 6 status"]
pub mod cm4_int6_status;
#[doc = "CM4_INT7_STATUS (r) register accessor: CM4 interrupt 7 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int7_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_int7_status`]
module"]
#[doc(alias = "CM4_INT7_STATUS")]
pub type Cm4Int7Status = crate::Reg<cm4_int7_status::Cm4Int7StatusSpec>;
#[doc = "CM4 interrupt 7 status"]
pub mod cm4_int7_status;
#[doc = "CM4_VECTOR_TABLE_BASE (rw) register accessor: CM4 vector table base\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_vector_table_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_vector_table_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_vector_table_base`]
module"]
#[doc(alias = "CM4_VECTOR_TABLE_BASE")]
pub type Cm4VectorTableBase = crate::Reg<cm4_vector_table_base::Cm4VectorTableBaseSpec>;
#[doc = "CM4 vector table base"]
pub mod cm4_vector_table_base;
#[doc = "CM4_NMI_CTL (rw) register accessor: CM4 NMI control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_nmi_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_nmi_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_nmi_ctl`]
module"]
#[doc(alias = "CM4_NMI_CTL")]
pub type Cm4NmiCtl = crate::Reg<cm4_nmi_ctl::Cm4NmiCtlSpec>;
#[doc = "CM4 NMI control"]
pub mod cm4_nmi_ctl;
#[doc = "UDB_PWR_CTL (rw) register accessor: UDB power control\n\nYou can [`read`](crate::Reg::read) this register and get [`udb_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udb_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udb_pwr_ctl`]
module"]
#[doc(alias = "UDB_PWR_CTL")]
pub type UdbPwrCtl = crate::Reg<udb_pwr_ctl::UdbPwrCtlSpec>;
#[doc = "UDB power control"]
pub mod udb_pwr_ctl;
#[doc = "UDB_PWR_DELAY_CTL (rw) register accessor: UDB power control\n\nYou can [`read`](crate::Reg::read) this register and get [`udb_pwr_delay_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udb_pwr_delay_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udb_pwr_delay_ctl`]
module"]
#[doc(alias = "UDB_PWR_DELAY_CTL")]
pub type UdbPwrDelayCtl = crate::Reg<udb_pwr_delay_ctl::UdbPwrDelayCtlSpec>;
#[doc = "UDB power control"]
pub mod udb_pwr_delay_ctl;
#[doc = "CM0_CTL (rw) register accessor: CM0+ control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_ctl`]
module"]
#[doc(alias = "CM0_CTL")]
pub type Cm0Ctl = crate::Reg<cm0_ctl::Cm0CtlSpec>;
#[doc = "CM0+ control"]
pub mod cm0_ctl;
#[doc = "CM0_STATUS (r) register accessor: CM0+ status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_status`]
module"]
#[doc(alias = "CM0_STATUS")]
pub type Cm0Status = crate::Reg<cm0_status::Cm0StatusSpec>;
#[doc = "CM0+ status"]
pub mod cm0_status;
#[doc = "CM0_CLOCK_CTL (rw) register accessor: CM0+ clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_clock_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_clock_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_clock_ctl`]
module"]
#[doc(alias = "CM0_CLOCK_CTL")]
pub type Cm0ClockCtl = crate::Reg<cm0_clock_ctl::Cm0ClockCtlSpec>;
#[doc = "CM0+ clock control"]
pub mod cm0_clock_ctl;
#[doc = "CM0_INT0_STATUS (r) register accessor: CM0+ interrupt 0 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int0_status`]
module"]
#[doc(alias = "CM0_INT0_STATUS")]
pub type Cm0Int0Status = crate::Reg<cm0_int0_status::Cm0Int0StatusSpec>;
#[doc = "CM0+ interrupt 0 status"]
pub mod cm0_int0_status;
#[doc = "CM0_INT1_STATUS (r) register accessor: CM0+ interrupt 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int1_status`]
module"]
#[doc(alias = "CM0_INT1_STATUS")]
pub type Cm0Int1Status = crate::Reg<cm0_int1_status::Cm0Int1StatusSpec>;
#[doc = "CM0+ interrupt 1 status"]
pub mod cm0_int1_status;
#[doc = "CM0_INT2_STATUS (r) register accessor: CM0+ interrupt 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int2_status`]
module"]
#[doc(alias = "CM0_INT2_STATUS")]
pub type Cm0Int2Status = crate::Reg<cm0_int2_status::Cm0Int2StatusSpec>;
#[doc = "CM0+ interrupt 2 status"]
pub mod cm0_int2_status;
#[doc = "CM0_INT3_STATUS (r) register accessor: CM0+ interrupt 3 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int3_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int3_status`]
module"]
#[doc(alias = "CM0_INT3_STATUS")]
pub type Cm0Int3Status = crate::Reg<cm0_int3_status::Cm0Int3StatusSpec>;
#[doc = "CM0+ interrupt 3 status"]
pub mod cm0_int3_status;
#[doc = "CM0_INT4_STATUS (r) register accessor: CM0+ interrupt 4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int4_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int4_status`]
module"]
#[doc(alias = "CM0_INT4_STATUS")]
pub type Cm0Int4Status = crate::Reg<cm0_int4_status::Cm0Int4StatusSpec>;
#[doc = "CM0+ interrupt 4 status"]
pub mod cm0_int4_status;
#[doc = "CM0_INT5_STATUS (r) register accessor: CM0+ interrupt 5 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int5_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int5_status`]
module"]
#[doc(alias = "CM0_INT5_STATUS")]
pub type Cm0Int5Status = crate::Reg<cm0_int5_status::Cm0Int5StatusSpec>;
#[doc = "CM0+ interrupt 5 status"]
pub mod cm0_int5_status;
#[doc = "CM0_INT6_STATUS (r) register accessor: CM0+ interrupt 6 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int6_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int6_status`]
module"]
#[doc(alias = "CM0_INT6_STATUS")]
pub type Cm0Int6Status = crate::Reg<cm0_int6_status::Cm0Int6StatusSpec>;
#[doc = "CM0+ interrupt 6 status"]
pub mod cm0_int6_status;
#[doc = "CM0_INT7_STATUS (r) register accessor: CM0+ interrupt 7 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int7_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_int7_status`]
module"]
#[doc(alias = "CM0_INT7_STATUS")]
pub type Cm0Int7Status = crate::Reg<cm0_int7_status::Cm0Int7StatusSpec>;
#[doc = "CM0+ interrupt 7 status"]
pub mod cm0_int7_status;
#[doc = "CM0_VECTOR_TABLE_BASE (rw) register accessor: CM0+ vector table base\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_vector_table_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_vector_table_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_vector_table_base`]
module"]
#[doc(alias = "CM0_VECTOR_TABLE_BASE")]
pub type Cm0VectorTableBase = crate::Reg<cm0_vector_table_base::Cm0VectorTableBaseSpec>;
#[doc = "CM0+ vector table base"]
pub mod cm0_vector_table_base;
#[doc = "CM0_NMI_CTL (rw) register accessor: CM0+ NMI control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_nmi_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_nmi_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_nmi_ctl`]
module"]
#[doc(alias = "CM0_NMI_CTL")]
pub type Cm0NmiCtl = crate::Reg<cm0_nmi_ctl::Cm0NmiCtlSpec>;
#[doc = "CM0+ NMI control"]
pub mod cm0_nmi_ctl;
#[doc = "CM4_PWR_CTL (rw) register accessor: CM4 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_pwr_ctl`]
module"]
#[doc(alias = "CM4_PWR_CTL")]
pub type Cm4PwrCtl = crate::Reg<cm4_pwr_ctl::Cm4PwrCtlSpec>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_ctl;
#[doc = "CM4_PWR_DELAY_CTL (rw) register accessor: CM4 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_pwr_delay_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_pwr_delay_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_pwr_delay_ctl`]
module"]
#[doc(alias = "CM4_PWR_DELAY_CTL")]
pub type Cm4PwrDelayCtl = crate::Reg<cm4_pwr_delay_ctl::Cm4PwrDelayCtlSpec>;
#[doc = "CM4 power control"]
pub mod cm4_pwr_delay_ctl;
#[doc = "RAM0_CTL0 (rw) register accessor: RAM 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_ctl0`]
module"]
#[doc(alias = "RAM0_CTL0")]
pub type Ram0Ctl0 = crate::Reg<ram0_ctl0::Ram0Ctl0Spec>;
#[doc = "RAM 0 control"]
pub mod ram0_ctl0;
#[doc = "RAM0_STATUS (r) register accessor: RAM 0 status\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_status`]
module"]
#[doc(alias = "RAM0_STATUS")]
pub type Ram0Status = crate::Reg<ram0_status::Ram0StatusSpec>;
#[doc = "RAM 0 status"]
pub mod ram0_status;
#[doc = "RAM0_PWR_MACRO_CTL (rw) register accessor: RAM 0 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_pwr_macro_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_pwr_macro_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram0_pwr_macro_ctl`]
module"]
#[doc(alias = "RAM0_PWR_MACRO_CTL")]
pub type Ram0PwrMacroCtl = crate::Reg<ram0_pwr_macro_ctl::Ram0PwrMacroCtlSpec>;
#[doc = "RAM 0 power control"]
pub mod ram0_pwr_macro_ctl;
#[doc = "RAM1_CTL0 (rw) register accessor: RAM 1 control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_ctl0`]
module"]
#[doc(alias = "RAM1_CTL0")]
pub type Ram1Ctl0 = crate::Reg<ram1_ctl0::Ram1Ctl0Spec>;
#[doc = "RAM 1 control"]
pub mod ram1_ctl0;
#[doc = "RAM1_STATUS (r) register accessor: RAM 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_status`]
module"]
#[doc(alias = "RAM1_STATUS")]
pub type Ram1Status = crate::Reg<ram1_status::Ram1StatusSpec>;
#[doc = "RAM 1 status"]
pub mod ram1_status;
#[doc = "RAM1_PWR_CTL (rw) register accessor: RAM 1 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram1_pwr_ctl`]
module"]
#[doc(alias = "RAM1_PWR_CTL")]
pub type Ram1PwrCtl = crate::Reg<ram1_pwr_ctl::Ram1PwrCtlSpec>;
#[doc = "RAM 1 power control"]
pub mod ram1_pwr_ctl;
#[doc = "RAM2_CTL0 (rw) register accessor: RAM 2 control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2_ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2_ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_ctl0`]
module"]
#[doc(alias = "RAM2_CTL0")]
pub type Ram2Ctl0 = crate::Reg<ram2_ctl0::Ram2Ctl0Spec>;
#[doc = "RAM 2 control"]
pub mod ram2_ctl0;
#[doc = "RAM2_STATUS (r) register accessor: RAM 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_status`]
module"]
#[doc(alias = "RAM2_STATUS")]
pub type Ram2Status = crate::Reg<ram2_status::Ram2StatusSpec>;
#[doc = "RAM 2 status"]
pub mod ram2_status;
#[doc = "RAM2_PWR_CTL (rw) register accessor: RAM 2 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2_pwr_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram2_pwr_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram2_pwr_ctl`]
module"]
#[doc(alias = "RAM2_PWR_CTL")]
pub type Ram2PwrCtl = crate::Reg<ram2_pwr_ctl::Ram2PwrCtlSpec>;
#[doc = "RAM 2 power control"]
pub mod ram2_pwr_ctl;
#[doc = "RAM_PWR_DELAY_CTL (rw) register accessor: Power up delay used for all SRAM power domains\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pwr_delay_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pwr_delay_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_pwr_delay_ctl`]
module"]
#[doc(alias = "RAM_PWR_DELAY_CTL")]
pub type RamPwrDelayCtl = crate::Reg<ram_pwr_delay_ctl::RamPwrDelayCtlSpec>;
#[doc = "Power up delay used for all SRAM power domains"]
pub mod ram_pwr_delay_ctl;
#[doc = "ROM_CTL (rw) register accessor: ROM control\n\nYou can [`read`](crate::Reg::read) this register and get [`rom_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rom_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rom_ctl`]
module"]
#[doc(alias = "ROM_CTL")]
pub type RomCtl = crate::Reg<rom_ctl::RomCtlSpec>;
#[doc = "ROM control"]
pub mod rom_ctl;
#[doc = "ECC_CTL (rw) register accessor: ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctl`]
module"]
#[doc(alias = "ECC_CTL")]
pub type EccCtl = crate::Reg<ecc_ctl::EccCtlSpec>;
#[doc = "ECC control"]
pub mod ecc_ctl;
#[doc = "PRODUCT_ID (r) register accessor: Product identifier and version (same as CoreSight RomTables)\n\nYou can [`read`](crate::Reg::read) this register and get [`product_id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@product_id`]
module"]
#[doc(alias = "PRODUCT_ID")]
pub type ProductId = crate::Reg<product_id::ProductIdSpec>;
#[doc = "Product identifier and version (same as CoreSight RomTables)"]
pub mod product_id;
#[doc = "DP_STATUS (r) register accessor: Debug port status\n\nYou can [`read`](crate::Reg::read) this register and get [`dp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp_status`]
module"]
#[doc(alias = "DP_STATUS")]
pub type DpStatus = crate::Reg<dp_status::DpStatusSpec>;
#[doc = "Debug port status"]
pub mod dp_status;
#[doc = "AP_CTL (rw) register accessor: Access port control\n\nYou can [`read`](crate::Reg::read) this register and get [`ap_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ap_ctl`]
module"]
#[doc(alias = "AP_CTL")]
pub type ApCtl = crate::Reg<ap_ctl::ApCtlSpec>;
#[doc = "Access port control"]
pub mod ap_ctl;
#[doc = "BUFF_CTL (rw) register accessor: Buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`buff_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff_ctl`]
module"]
#[doc(alias = "BUFF_CTL")]
pub type BuffCtl = crate::Reg<buff_ctl::BuffCtlSpec>;
#[doc = "Buffer control"]
pub mod buff_ctl;
#[doc = "SYSTICK_CTL (rw) register accessor: SysTick timer control\n\nYou can [`read`](crate::Reg::read) this register and get [`systick_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systick_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systick_ctl`]
module"]
#[doc(alias = "SYSTICK_CTL")]
pub type SystickCtl = crate::Reg<systick_ctl::SystickCtlSpec>;
#[doc = "SysTick timer control"]
pub mod systick_ctl;
#[doc = "MBIST_STAT (r) register accessor: Memory BIST status\n\nYou can [`read`](crate::Reg::read) this register and get [`mbist_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mbist_stat`]
module"]
#[doc(alias = "MBIST_STAT")]
pub type MbistStat = crate::Reg<mbist_stat::MbistStatSpec>;
#[doc = "Memory BIST status"]
pub mod mbist_stat;
#[doc = "CAL_SUP_SET (rw) register accessor: Calibration support set and read\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_sup_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_sup_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_sup_set`]
module"]
#[doc(alias = "CAL_SUP_SET")]
pub type CalSupSet = crate::Reg<cal_sup_set::CalSupSetSpec>;
#[doc = "Calibration support set and read"]
pub mod cal_sup_set;
#[doc = "CAL_SUP_CLR (rw) register accessor: Calibration support clear and reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_sup_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_sup_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal_sup_clr`]
module"]
#[doc(alias = "CAL_SUP_CLR")]
pub type CalSupClr = crate::Reg<cal_sup_clr::CalSupClrSpec>;
#[doc = "Calibration support clear and reset"]
pub mod cal_sup_clr;
#[doc = "CM0_PC_CTL (rw) register accessor: CM0+ protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc_ctl`]
module"]
#[doc(alias = "CM0_PC_CTL")]
pub type Cm0PcCtl = crate::Reg<cm0_pc_ctl::Cm0PcCtlSpec>;
#[doc = "CM0+ protection context control"]
pub mod cm0_pc_ctl;
#[doc = "CM0_PC0_HANDLER (rw) register accessor: CM0+ protection context 0 handler\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc0_handler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc0_handler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc0_handler`]
module"]
#[doc(alias = "CM0_PC0_HANDLER")]
pub type Cm0Pc0Handler = crate::Reg<cm0_pc0_handler::Cm0Pc0HandlerSpec>;
#[doc = "CM0+ protection context 0 handler"]
pub mod cm0_pc0_handler;
#[doc = "CM0_PC1_HANDLER (rw) register accessor: CM0+ protection context 1 handler\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc1_handler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc1_handler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc1_handler`]
module"]
#[doc(alias = "CM0_PC1_HANDLER")]
pub type Cm0Pc1Handler = crate::Reg<cm0_pc1_handler::Cm0Pc1HandlerSpec>;
#[doc = "CM0+ protection context 1 handler"]
pub mod cm0_pc1_handler;
#[doc = "CM0_PC2_HANDLER (rw) register accessor: CM0+ protection context 2 handler\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc2_handler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc2_handler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc2_handler`]
module"]
#[doc(alias = "CM0_PC2_HANDLER")]
pub type Cm0Pc2Handler = crate::Reg<cm0_pc2_handler::Cm0Pc2HandlerSpec>;
#[doc = "CM0+ protection context 2 handler"]
pub mod cm0_pc2_handler;
#[doc = "CM0_PC3_HANDLER (rw) register accessor: CM0+ protection context 3 handler\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc3_handler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc3_handler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_pc3_handler`]
module"]
#[doc(alias = "CM0_PC3_HANDLER")]
pub type Cm0Pc3Handler = crate::Reg<cm0_pc3_handler::Cm0Pc3HandlerSpec>;
#[doc = "CM0+ protection context 3 handler"]
pub mod cm0_pc3_handler;
#[doc = "PROTECTION (rw) register accessor: Protection status\n\nYou can [`read`](crate::Reg::read) this register and get [`protection::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`protection::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@protection`]
module"]
#[doc(alias = "PROTECTION")]
pub type Protection = crate::Reg<protection::ProtectionSpec>;
#[doc = "Protection status"]
pub mod protection;
#[doc = "TRIM_ROM_CTL (rw) register accessor: ROM trim control\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_rom_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_rom_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_rom_ctl`]
module"]
#[doc(alias = "TRIM_ROM_CTL")]
pub type TrimRomCtl = crate::Reg<trim_rom_ctl::TrimRomCtlSpec>;
#[doc = "ROM trim control"]
pub mod trim_rom_ctl;
#[doc = "TRIM_RAM_CTL (rw) register accessor: RAM trim control\n\nYou can [`read`](crate::Reg::read) this register and get [`trim_ram_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim_ram_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trim_ram_ctl`]
module"]
#[doc(alias = "TRIM_RAM_CTL")]
pub type TrimRamCtl = crate::Reg<trim_ram_ctl::TrimRamCtlSpec>;
#[doc = "RAM trim control"]
pub mod trim_ram_ctl;
#[doc = "CM0_SYSTEM_INT_CTL (rw) register accessor: CM0+ system interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_system_int_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_system_int_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm0_system_int_ctl`]
module"]
#[doc(alias = "CM0_SYSTEM_INT_CTL")]
pub type Cm0SystemIntCtl = crate::Reg<cm0_system_int_ctl::Cm0SystemIntCtlSpec>;
#[doc = "CM0+ system interrupt control"]
pub mod cm0_system_int_ctl;
#[doc = "CM4_SYSTEM_INT_CTL (rw) register accessor: CM4 system interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_system_int_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_system_int_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm4_system_int_ctl`]
module"]
#[doc(alias = "CM4_SYSTEM_INT_CTL")]
pub type Cm4SystemIntCtl = crate::Reg<cm4_system_int_ctl::Cm4SystemIntCtlSpec>;
#[doc = "CM4 system interrupt control"]
pub mod cm4_system_int_ctl;
