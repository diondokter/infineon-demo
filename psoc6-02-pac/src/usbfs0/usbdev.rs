#[repr(C)]
#[doc = "USB Device"]
#[doc(alias = "USBDEV")]
pub struct Usbdev {
    ep0_dr: [Ep0Dr; 8],
    cr0: Cr0,
    cr1: Cr1,
    sie_ep_int_en: SieEpIntEn,
    sie_ep_int_sr: SieEpIntSr,
    sie_ep1_cnt0: SieEp1Cnt0,
    sie_ep1_cnt1: SieEp1Cnt1,
    sie_ep1_cr0: SieEp1Cr0,
    _reserved8: [u8; 0x04],
    usbio_cr0: UsbioCr0,
    usbio_cr2: UsbioCr2,
    usbio_cr1: UsbioCr1,
    _reserved11: [u8; 0x04],
    dyn_reconfig: DynReconfig,
    _reserved12: [u8; 0x0c],
    sof0: Sof0,
    sof1: Sof1,
    _reserved14: [u8; 0x08],
    sie_ep2_cnt0: SieEp2Cnt0,
    sie_ep2_cnt1: SieEp2Cnt1,
    sie_ep2_cr0: SieEp2Cr0,
    _reserved17: [u8; 0x04],
    osclk_dr0: OsclkDr0,
    osclk_dr1: OsclkDr1,
    _reserved19: [u8; 0x18],
    ep0_cr: Ep0Cr,
    ep0_cnt: Ep0Cnt,
    _reserved21: [u8; 0x08],
    sie_ep3_cnt0: SieEp3Cnt0,
    sie_ep3_cnt1: SieEp3Cnt1,
    sie_ep3_cr0: SieEp3Cr0,
    _reserved24: [u8; 0x34],
    sie_ep4_cnt0: SieEp4Cnt0,
    sie_ep4_cnt1: SieEp4Cnt1,
    sie_ep4_cr0: SieEp4Cr0,
    _reserved27: [u8; 0x34],
    sie_ep5_cnt0: SieEp5Cnt0,
    sie_ep5_cnt1: SieEp5Cnt1,
    sie_ep5_cr0: SieEp5Cr0,
    _reserved30: [u8; 0x34],
    sie_ep6_cnt0: SieEp6Cnt0,
    sie_ep6_cnt1: SieEp6Cnt1,
    sie_ep6_cr0: SieEp6Cr0,
    _reserved33: [u8; 0x34],
    sie_ep7_cnt0: SieEp7Cnt0,
    sie_ep7_cnt1: SieEp7Cnt1,
    sie_ep7_cr0: SieEp7Cr0,
    _reserved36: [u8; 0x34],
    sie_ep8_cnt0: SieEp8Cnt0,
    sie_ep8_cnt1: SieEp8Cnt1,
    sie_ep8_cr0: SieEp8Cr0,
    _reserved39: [u8; 0x04],
    arb_ep1_cfg: ArbEp1Cfg,
    arb_ep1_int_en: ArbEp1IntEn,
    arb_ep1_sr: ArbEp1Sr,
    _reserved42: [u8; 0x04],
    arb_rw1_wa: ArbRw1Wa,
    arb_rw1_wa_msb: ArbRw1WaMsb,
    arb_rw1_ra: ArbRw1Ra,
    arb_rw1_ra_msb: ArbRw1RaMsb,
    arb_rw1_dr: ArbRw1Dr,
    _reserved47: [u8; 0x0c],
    buf_size: BufSize,
    _reserved48: [u8; 0x04],
    ep_active: EpActive,
    ep_type: EpType,
    arb_ep2_cfg: ArbEp2Cfg,
    arb_ep2_int_en: ArbEp2IntEn,
    arb_ep2_sr: ArbEp2Sr,
    _reserved53: [u8; 0x04],
    arb_rw2_wa: ArbRw2Wa,
    arb_rw2_wa_msb: ArbRw2WaMsb,
    arb_rw2_ra: ArbRw2Ra,
    arb_rw2_ra_msb: ArbRw2RaMsb,
    arb_rw2_dr: ArbRw2Dr,
    _reserved58: [u8; 0x0c],
    arb_cfg: ArbCfg,
    usb_clk_en: UsbClkEn,
    arb_int_en: ArbIntEn,
    arb_int_sr: ArbIntSr,
    arb_ep3_cfg: ArbEp3Cfg,
    arb_ep3_int_en: ArbEp3IntEn,
    arb_ep3_sr: ArbEp3Sr,
    _reserved65: [u8; 0x04],
    arb_rw3_wa: ArbRw3Wa,
    arb_rw3_wa_msb: ArbRw3WaMsb,
    arb_rw3_ra: ArbRw3Ra,
    arb_rw3_ra_msb: ArbRw3RaMsb,
    arb_rw3_dr: ArbRw3Dr,
    _reserved70: [u8; 0x0c],
    cwa: Cwa,
    cwa_msb: CwaMsb,
    _reserved72: [u8; 0x08],
    arb_ep4_cfg: ArbEp4Cfg,
    arb_ep4_int_en: ArbEp4IntEn,
    arb_ep4_sr: ArbEp4Sr,
    _reserved75: [u8; 0x04],
    arb_rw4_wa: ArbRw4Wa,
    arb_rw4_wa_msb: ArbRw4WaMsb,
    arb_rw4_ra: ArbRw4Ra,
    arb_rw4_ra_msb: ArbRw4RaMsb,
    arb_rw4_dr: ArbRw4Dr,
    _reserved80: [u8; 0x0c],
    dma_thres: DmaThres,
    dma_thres_msb: DmaThresMsb,
    _reserved82: [u8; 0x08],
    arb_ep5_cfg: ArbEp5Cfg,
    arb_ep5_int_en: ArbEp5IntEn,
    arb_ep5_sr: ArbEp5Sr,
    _reserved85: [u8; 0x04],
    arb_rw5_wa: ArbRw5Wa,
    arb_rw5_wa_msb: ArbRw5WaMsb,
    arb_rw5_ra: ArbRw5Ra,
    arb_rw5_ra_msb: ArbRw5RaMsb,
    arb_rw5_dr: ArbRw5Dr,
    _reserved90: [u8; 0x0c],
    bus_rst_cnt: BusRstCnt,
    _reserved91: [u8; 0x0c],
    arb_ep6_cfg: ArbEp6Cfg,
    arb_ep6_int_en: ArbEp6IntEn,
    arb_ep6_sr: ArbEp6Sr,
    _reserved94: [u8; 0x04],
    arb_rw6_wa: ArbRw6Wa,
    arb_rw6_wa_msb: ArbRw6WaMsb,
    arb_rw6_ra: ArbRw6Ra,
    arb_rw6_ra_msb: ArbRw6RaMsb,
    arb_rw6_dr: ArbRw6Dr,
    _reserved99: [u8; 0x1c],
    arb_ep7_cfg: ArbEp7Cfg,
    arb_ep7_int_en: ArbEp7IntEn,
    arb_ep7_sr: ArbEp7Sr,
    _reserved102: [u8; 0x04],
    arb_rw7_wa: ArbRw7Wa,
    arb_rw7_wa_msb: ArbRw7WaMsb,
    arb_rw7_ra: ArbRw7Ra,
    arb_rw7_ra_msb: ArbRw7RaMsb,
    arb_rw7_dr: ArbRw7Dr,
    _reserved107: [u8; 0x1c],
    arb_ep8_cfg: ArbEp8Cfg,
    arb_ep8_int_en: ArbEp8IntEn,
    arb_ep8_sr: ArbEp8Sr,
    _reserved110: [u8; 0x04],
    arb_rw8_wa: ArbRw8Wa,
    arb_rw8_wa_msb: ArbRw8WaMsb,
    arb_rw8_ra: ArbRw8Ra,
    arb_rw8_ra_msb: ArbRw8RaMsb,
    arb_rw8_dr: ArbRw8Dr,
    _reserved115: [u8; 0x1c],
    mem_data: [MemData; 512],
    _reserved116: [u8; 0x0460],
    sof16: Sof16,
    _reserved117: [u8; 0x1c],
    osclk_dr16: OsclkDr16,
    _reserved118: [u8; 0x018c],
    arb_rw1_wa16: ArbRw1Wa16,
    _reserved119: [u8; 0x04],
    arb_rw1_ra16: ArbRw1Ra16,
    _reserved120: [u8; 0x04],
    arb_rw1_dr16: ArbRw1Dr16,
    _reserved121: [u8; 0x2c],
    arb_rw2_wa16: ArbRw2Wa16,
    _reserved122: [u8; 0x04],
    arb_rw2_ra16: ArbRw2Ra16,
    _reserved123: [u8; 0x04],
    arb_rw2_dr16: ArbRw2Dr16,
    _reserved124: [u8; 0x2c],
    arb_rw3_wa16: ArbRw3Wa16,
    _reserved125: [u8; 0x04],
    arb_rw3_ra16: ArbRw3Ra16,
    _reserved126: [u8; 0x04],
    arb_rw3_dr16: ArbRw3Dr16,
    _reserved127: [u8; 0x0c],
    cwa16: Cwa16,
    _reserved128: [u8; 0x1c],
    arb_rw4_wa16: ArbRw4Wa16,
    _reserved129: [u8; 0x04],
    arb_rw4_ra16: ArbRw4Ra16,
    _reserved130: [u8; 0x04],
    arb_rw4_dr16: ArbRw4Dr16,
    _reserved131: [u8; 0x0c],
    dma_thres16: DmaThres16,
    _reserved132: [u8; 0x1c],
    arb_rw5_wa16: ArbRw5Wa16,
    _reserved133: [u8; 0x04],
    arb_rw5_ra16: ArbRw5Ra16,
    _reserved134: [u8; 0x04],
    arb_rw5_dr16: ArbRw5Dr16,
    _reserved135: [u8; 0x2c],
    arb_rw6_wa16: ArbRw6Wa16,
    _reserved136: [u8; 0x04],
    arb_rw6_ra16: ArbRw6Ra16,
    _reserved137: [u8; 0x04],
    arb_rw6_dr16: ArbRw6Dr16,
    _reserved138: [u8; 0x2c],
    arb_rw7_wa16: ArbRw7Wa16,
    _reserved139: [u8; 0x04],
    arb_rw7_ra16: ArbRw7Ra16,
    _reserved140: [u8; 0x04],
    arb_rw7_dr16: ArbRw7Dr16,
    _reserved141: [u8; 0x2c],
    arb_rw8_wa16: ArbRw8Wa16,
    _reserved142: [u8; 0x04],
    arb_rw8_ra16: ArbRw8Ra16,
    _reserved143: [u8; 0x04],
    arb_rw8_dr16: ArbRw8Dr16,
}
impl Usbdev {
    #[doc = "0x00..0x20 - Control End point EP0 Data Register"]
    #[inline(always)]
    pub const fn ep0_dr(&self, n: usize) -> &Ep0Dr {
        &self.ep0_dr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Control End point EP0 Data Register"]
    #[inline(always)]
    pub fn ep0_dr_iter(&self) -> impl Iterator<Item = &Ep0Dr> {
        self.ep0_dr.iter()
    }
    #[doc = "0x20 - USB control 0 Register"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x24 - USB control 1 Register"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x28 - USB SIE Data Endpoints Interrupt Enable Register"]
    #[inline(always)]
    pub const fn sie_ep_int_en(&self) -> &SieEpIntEn {
        &self.sie_ep_int_en
    }
    #[doc = "0x2c - USB SIE Data Endpoint Interrupt Status"]
    #[inline(always)]
    pub const fn sie_ep_int_sr(&self) -> &SieEpIntSr {
        &self.sie_ep_int_sr
    }
    #[doc = "0x30 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt0(&self) -> &SieEp1Cnt0 {
        &self.sie_ep1_cnt0
    }
    #[doc = "0x34 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep1_cnt1(&self) -> &SieEp1Cnt1 {
        &self.sie_ep1_cnt1
    }
    #[doc = "0x38 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep1_cr0(&self) -> &SieEp1Cr0 {
        &self.sie_ep1_cr0
    }
    #[doc = "0x40 - USBIO Control 0 Register"]
    #[inline(always)]
    pub const fn usbio_cr0(&self) -> &UsbioCr0 {
        &self.usbio_cr0
    }
    #[doc = "0x44 - USBIO control 2 Register"]
    #[inline(always)]
    pub const fn usbio_cr2(&self) -> &UsbioCr2 {
        &self.usbio_cr2
    }
    #[doc = "0x48 - USBIO control 1 Register"]
    #[inline(always)]
    pub const fn usbio_cr1(&self) -> &UsbioCr1 {
        &self.usbio_cr1
    }
    #[doc = "0x50 - USB Dynamic reconfiguration register"]
    #[inline(always)]
    pub const fn dyn_reconfig(&self) -> &DynReconfig {
        &self.dyn_reconfig
    }
    #[doc = "0x60 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof0(&self) -> &Sof0 {
        &self.sof0
    }
    #[doc = "0x64 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof1(&self) -> &Sof1 {
        &self.sof1
    }
    #[doc = "0x70 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt0(&self) -> &SieEp2Cnt0 {
        &self.sie_ep2_cnt0
    }
    #[doc = "0x74 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep2_cnt1(&self) -> &SieEp2Cnt1 {
        &self.sie_ep2_cnt1
    }
    #[doc = "0x78 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep2_cr0(&self) -> &SieEp2Cr0 {
        &self.sie_ep2_cr0
    }
    #[doc = "0x80 - Oscillator lock data register 0"]
    #[inline(always)]
    pub const fn osclk_dr0(&self) -> &OsclkDr0 {
        &self.osclk_dr0
    }
    #[doc = "0x84 - Oscillator lock data register 1"]
    #[inline(always)]
    pub const fn osclk_dr1(&self) -> &OsclkDr1 {
        &self.osclk_dr1
    }
    #[doc = "0xa0 - Endpoint0 control Register"]
    #[inline(always)]
    pub const fn ep0_cr(&self) -> &Ep0Cr {
        &self.ep0_cr
    }
    #[doc = "0xa4 - Endpoint0 count Register"]
    #[inline(always)]
    pub const fn ep0_cnt(&self) -> &Ep0Cnt {
        &self.ep0_cnt
    }
    #[doc = "0xb0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt0(&self) -> &SieEp3Cnt0 {
        &self.sie_ep3_cnt0
    }
    #[doc = "0xb4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep3_cnt1(&self) -> &SieEp3Cnt1 {
        &self.sie_ep3_cnt1
    }
    #[doc = "0xb8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep3_cr0(&self) -> &SieEp3Cr0 {
        &self.sie_ep3_cr0
    }
    #[doc = "0xf0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt0(&self) -> &SieEp4Cnt0 {
        &self.sie_ep4_cnt0
    }
    #[doc = "0xf4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep4_cnt1(&self) -> &SieEp4Cnt1 {
        &self.sie_ep4_cnt1
    }
    #[doc = "0xf8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep4_cr0(&self) -> &SieEp4Cr0 {
        &self.sie_ep4_cr0
    }
    #[doc = "0x130 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt0(&self) -> &SieEp5Cnt0 {
        &self.sie_ep5_cnt0
    }
    #[doc = "0x134 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep5_cnt1(&self) -> &SieEp5Cnt1 {
        &self.sie_ep5_cnt1
    }
    #[doc = "0x138 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep5_cr0(&self) -> &SieEp5Cr0 {
        &self.sie_ep5_cr0
    }
    #[doc = "0x170 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt0(&self) -> &SieEp6Cnt0 {
        &self.sie_ep6_cnt0
    }
    #[doc = "0x174 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep6_cnt1(&self) -> &SieEp6Cnt1 {
        &self.sie_ep6_cnt1
    }
    #[doc = "0x178 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep6_cr0(&self) -> &SieEp6Cr0 {
        &self.sie_ep6_cr0
    }
    #[doc = "0x1b0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt0(&self) -> &SieEp7Cnt0 {
        &self.sie_ep7_cnt0
    }
    #[doc = "0x1b4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep7_cnt1(&self) -> &SieEp7Cnt1 {
        &self.sie_ep7_cnt1
    }
    #[doc = "0x1b8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep7_cr0(&self) -> &SieEp7Cr0 {
        &self.sie_ep7_cr0
    }
    #[doc = "0x1f0 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt0(&self) -> &SieEp8Cnt0 {
        &self.sie_ep8_cnt0
    }
    #[doc = "0x1f4 - Non-control endpoint count register"]
    #[inline(always)]
    pub const fn sie_ep8_cnt1(&self) -> &SieEp8Cnt1 {
        &self.sie_ep8_cnt1
    }
    #[doc = "0x1f8 - Non-control endpoint's control Register"]
    #[inline(always)]
    pub const fn sie_ep8_cr0(&self) -> &SieEp8Cr0 {
        &self.sie_ep8_cr0
    }
    #[doc = "0x200 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_cfg(&self) -> &ArbEp1Cfg {
        &self.arb_ep1_cfg
    }
    #[doc = "0x204 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_int_en(&self) -> &ArbEp1IntEn {
        &self.arb_ep1_int_en
    }
    #[doc = "0x208 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep1_sr(&self) -> &ArbEp1Sr {
        &self.arb_ep1_sr
    }
    #[doc = "0x210 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_wa(&self) -> &ArbRw1Wa {
        &self.arb_rw1_wa
    }
    #[doc = "0x214 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_wa_msb(&self) -> &ArbRw1WaMsb {
        &self.arb_rw1_wa_msb
    }
    #[doc = "0x218 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_ra(&self) -> &ArbRw1Ra {
        &self.arb_rw1_ra
    }
    #[doc = "0x21c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw1_ra_msb(&self) -> &ArbRw1RaMsb {
        &self.arb_rw1_ra_msb
    }
    #[doc = "0x220 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr(&self) -> &ArbRw1Dr {
        &self.arb_rw1_dr
    }
    #[doc = "0x230 - Dedicated Endpoint Buffer Size Register *1"]
    #[inline(always)]
    pub const fn buf_size(&self) -> &BufSize {
        &self.buf_size
    }
    #[doc = "0x238 - Endpoint Active Indication Register *1"]
    #[inline(always)]
    pub const fn ep_active(&self) -> &EpActive {
        &self.ep_active
    }
    #[doc = "0x23c - Endpoint Type (IN/OUT) Indication *1"]
    #[inline(always)]
    pub const fn ep_type(&self) -> &EpType {
        &self.ep_type
    }
    #[doc = "0x240 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_cfg(&self) -> &ArbEp2Cfg {
        &self.arb_ep2_cfg
    }
    #[doc = "0x244 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_int_en(&self) -> &ArbEp2IntEn {
        &self.arb_ep2_int_en
    }
    #[doc = "0x248 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep2_sr(&self) -> &ArbEp2Sr {
        &self.arb_ep2_sr
    }
    #[doc = "0x250 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_wa(&self) -> &ArbRw2Wa {
        &self.arb_rw2_wa
    }
    #[doc = "0x254 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_wa_msb(&self) -> &ArbRw2WaMsb {
        &self.arb_rw2_wa_msb
    }
    #[doc = "0x258 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_ra(&self) -> &ArbRw2Ra {
        &self.arb_rw2_ra
    }
    #[doc = "0x25c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw2_ra_msb(&self) -> &ArbRw2RaMsb {
        &self.arb_rw2_ra_msb
    }
    #[doc = "0x260 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr(&self) -> &ArbRw2Dr {
        &self.arb_rw2_dr
    }
    #[doc = "0x270 - Arbiter Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_cfg(&self) -> &ArbCfg {
        &self.arb_cfg
    }
    #[doc = "0x274 - USB Block Clock Enable Register"]
    #[inline(always)]
    pub const fn usb_clk_en(&self) -> &UsbClkEn {
        &self.usb_clk_en
    }
    #[doc = "0x278 - Arbiter Interrupt Enable *1"]
    #[inline(always)]
    pub const fn arb_int_en(&self) -> &ArbIntEn {
        &self.arb_int_en
    }
    #[doc = "0x27c - Arbiter Interrupt Status *1"]
    #[inline(always)]
    pub const fn arb_int_sr(&self) -> &ArbIntSr {
        &self.arb_int_sr
    }
    #[doc = "0x280 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_cfg(&self) -> &ArbEp3Cfg {
        &self.arb_ep3_cfg
    }
    #[doc = "0x284 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_int_en(&self) -> &ArbEp3IntEn {
        &self.arb_ep3_int_en
    }
    #[doc = "0x288 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep3_sr(&self) -> &ArbEp3Sr {
        &self.arb_ep3_sr
    }
    #[doc = "0x290 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_wa(&self) -> &ArbRw3Wa {
        &self.arb_rw3_wa
    }
    #[doc = "0x294 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_wa_msb(&self) -> &ArbRw3WaMsb {
        &self.arb_rw3_wa_msb
    }
    #[doc = "0x298 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_ra(&self) -> &ArbRw3Ra {
        &self.arb_rw3_ra
    }
    #[doc = "0x29c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw3_ra_msb(&self) -> &ArbRw3RaMsb {
        &self.arb_rw3_ra_msb
    }
    #[doc = "0x2a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr(&self) -> &ArbRw3Dr {
        &self.arb_rw3_dr
    }
    #[doc = "0x2b0 - Common Area Write Address *1"]
    #[inline(always)]
    pub const fn cwa(&self) -> &Cwa {
        &self.cwa
    }
    #[doc = "0x2b4 - Endpoint Read Address value *1"]
    #[inline(always)]
    pub const fn cwa_msb(&self) -> &CwaMsb {
        &self.cwa_msb
    }
    #[doc = "0x2c0 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_cfg(&self) -> &ArbEp4Cfg {
        &self.arb_ep4_cfg
    }
    #[doc = "0x2c4 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_int_en(&self) -> &ArbEp4IntEn {
        &self.arb_ep4_int_en
    }
    #[doc = "0x2c8 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep4_sr(&self) -> &ArbEp4Sr {
        &self.arb_ep4_sr
    }
    #[doc = "0x2d0 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_wa(&self) -> &ArbRw4Wa {
        &self.arb_rw4_wa
    }
    #[doc = "0x2d4 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_wa_msb(&self) -> &ArbRw4WaMsb {
        &self.arb_rw4_wa_msb
    }
    #[doc = "0x2d8 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_ra(&self) -> &ArbRw4Ra {
        &self.arb_rw4_ra
    }
    #[doc = "0x2dc - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw4_ra_msb(&self) -> &ArbRw4RaMsb {
        &self.arb_rw4_ra_msb
    }
    #[doc = "0x2e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr(&self) -> &ArbRw4Dr {
        &self.arb_rw4_dr
    }
    #[doc = "0x2f0 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres(&self) -> &DmaThres {
        &self.dma_thres
    }
    #[doc = "0x2f4 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres_msb(&self) -> &DmaThresMsb {
        &self.dma_thres_msb
    }
    #[doc = "0x300 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_cfg(&self) -> &ArbEp5Cfg {
        &self.arb_ep5_cfg
    }
    #[doc = "0x304 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_int_en(&self) -> &ArbEp5IntEn {
        &self.arb_ep5_int_en
    }
    #[doc = "0x308 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep5_sr(&self) -> &ArbEp5Sr {
        &self.arb_ep5_sr
    }
    #[doc = "0x310 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_wa(&self) -> &ArbRw5Wa {
        &self.arb_rw5_wa
    }
    #[doc = "0x314 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_wa_msb(&self) -> &ArbRw5WaMsb {
        &self.arb_rw5_wa_msb
    }
    #[doc = "0x318 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_ra(&self) -> &ArbRw5Ra {
        &self.arb_rw5_ra
    }
    #[doc = "0x31c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw5_ra_msb(&self) -> &ArbRw5RaMsb {
        &self.arb_rw5_ra_msb
    }
    #[doc = "0x320 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr(&self) -> &ArbRw5Dr {
        &self.arb_rw5_dr
    }
    #[doc = "0x330 - Bus Reset Count Register"]
    #[inline(always)]
    pub const fn bus_rst_cnt(&self) -> &BusRstCnt {
        &self.bus_rst_cnt
    }
    #[doc = "0x340 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_cfg(&self) -> &ArbEp6Cfg {
        &self.arb_ep6_cfg
    }
    #[doc = "0x344 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_int_en(&self) -> &ArbEp6IntEn {
        &self.arb_ep6_int_en
    }
    #[doc = "0x348 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep6_sr(&self) -> &ArbEp6Sr {
        &self.arb_ep6_sr
    }
    #[doc = "0x350 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_wa(&self) -> &ArbRw6Wa {
        &self.arb_rw6_wa
    }
    #[doc = "0x354 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_wa_msb(&self) -> &ArbRw6WaMsb {
        &self.arb_rw6_wa_msb
    }
    #[doc = "0x358 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_ra(&self) -> &ArbRw6Ra {
        &self.arb_rw6_ra
    }
    #[doc = "0x35c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw6_ra_msb(&self) -> &ArbRw6RaMsb {
        &self.arb_rw6_ra_msb
    }
    #[doc = "0x360 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr(&self) -> &ArbRw6Dr {
        &self.arb_rw6_dr
    }
    #[doc = "0x380 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_cfg(&self) -> &ArbEp7Cfg {
        &self.arb_ep7_cfg
    }
    #[doc = "0x384 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_int_en(&self) -> &ArbEp7IntEn {
        &self.arb_ep7_int_en
    }
    #[doc = "0x388 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep7_sr(&self) -> &ArbEp7Sr {
        &self.arb_ep7_sr
    }
    #[doc = "0x390 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_wa(&self) -> &ArbRw7Wa {
        &self.arb_rw7_wa
    }
    #[doc = "0x394 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_wa_msb(&self) -> &ArbRw7WaMsb {
        &self.arb_rw7_wa_msb
    }
    #[doc = "0x398 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_ra(&self) -> &ArbRw7Ra {
        &self.arb_rw7_ra
    }
    #[doc = "0x39c - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw7_ra_msb(&self) -> &ArbRw7RaMsb {
        &self.arb_rw7_ra_msb
    }
    #[doc = "0x3a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr(&self) -> &ArbRw7Dr {
        &self.arb_rw7_dr
    }
    #[doc = "0x3c0 - Endpoint Configuration Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_cfg(&self) -> &ArbEp8Cfg {
        &self.arb_ep8_cfg
    }
    #[doc = "0x3c4 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_int_en(&self) -> &ArbEp8IntEn {
        &self.arb_ep8_int_en
    }
    #[doc = "0x3c8 - Endpoint Interrupt Enable Register *1"]
    #[inline(always)]
    pub const fn arb_ep8_sr(&self) -> &ArbEp8Sr {
        &self.arb_ep8_sr
    }
    #[doc = "0x3d0 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_wa(&self) -> &ArbRw8Wa {
        &self.arb_rw8_wa
    }
    #[doc = "0x3d4 - Endpoint Write Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_wa_msb(&self) -> &ArbRw8WaMsb {
        &self.arb_rw8_wa_msb
    }
    #[doc = "0x3d8 - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_ra(&self) -> &ArbRw8Ra {
        &self.arb_rw8_ra
    }
    #[doc = "0x3dc - Endpoint Read Address value *1, *2"]
    #[inline(always)]
    pub const fn arb_rw8_ra_msb(&self) -> &ArbRw8RaMsb {
        &self.arb_rw8_ra_msb
    }
    #[doc = "0x3e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr(&self) -> &ArbRw8Dr {
        &self.arb_rw8_dr
    }
    #[doc = "0x400..0xc00 - DATA"]
    #[inline(always)]
    pub const fn mem_data(&self, n: usize) -> &MemData {
        &self.mem_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0xc00 - DATA"]
    #[inline(always)]
    pub fn mem_data_iter(&self) -> impl Iterator<Item = &MemData> {
        self.mem_data.iter()
    }
    #[doc = "0x1060 - Start Of Frame Register"]
    #[inline(always)]
    pub const fn sof16(&self) -> &Sof16 {
        &self.sof16
    }
    #[doc = "0x1080 - Oscillator lock data register"]
    #[inline(always)]
    pub const fn osclk_dr16(&self) -> &OsclkDr16 {
        &self.osclk_dr16
    }
    #[doc = "0x1210 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw1_wa16(&self) -> &ArbRw1Wa16 {
        &self.arb_rw1_wa16
    }
    #[doc = "0x1218 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw1_ra16(&self) -> &ArbRw1Ra16 {
        &self.arb_rw1_ra16
    }
    #[doc = "0x1220 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw1_dr16(&self) -> &ArbRw1Dr16 {
        &self.arb_rw1_dr16
    }
    #[doc = "0x1250 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw2_wa16(&self) -> &ArbRw2Wa16 {
        &self.arb_rw2_wa16
    }
    #[doc = "0x1258 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw2_ra16(&self) -> &ArbRw2Ra16 {
        &self.arb_rw2_ra16
    }
    #[doc = "0x1260 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw2_dr16(&self) -> &ArbRw2Dr16 {
        &self.arb_rw2_dr16
    }
    #[doc = "0x1290 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw3_wa16(&self) -> &ArbRw3Wa16 {
        &self.arb_rw3_wa16
    }
    #[doc = "0x1298 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw3_ra16(&self) -> &ArbRw3Ra16 {
        &self.arb_rw3_ra16
    }
    #[doc = "0x12a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw3_dr16(&self) -> &ArbRw3Dr16 {
        &self.arb_rw3_dr16
    }
    #[doc = "0x12b0 - Common Area Write Address"]
    #[inline(always)]
    pub const fn cwa16(&self) -> &Cwa16 {
        &self.cwa16
    }
    #[doc = "0x12d0 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw4_wa16(&self) -> &ArbRw4Wa16 {
        &self.arb_rw4_wa16
    }
    #[doc = "0x12d8 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw4_ra16(&self) -> &ArbRw4Ra16 {
        &self.arb_rw4_ra16
    }
    #[doc = "0x12e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw4_dr16(&self) -> &ArbRw4Dr16 {
        &self.arb_rw4_dr16
    }
    #[doc = "0x12f0 - DMA Burst / Threshold Configuration"]
    #[inline(always)]
    pub const fn dma_thres16(&self) -> &DmaThres16 {
        &self.dma_thres16
    }
    #[doc = "0x1310 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw5_wa16(&self) -> &ArbRw5Wa16 {
        &self.arb_rw5_wa16
    }
    #[doc = "0x1318 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw5_ra16(&self) -> &ArbRw5Ra16 {
        &self.arb_rw5_ra16
    }
    #[doc = "0x1320 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw5_dr16(&self) -> &ArbRw5Dr16 {
        &self.arb_rw5_dr16
    }
    #[doc = "0x1350 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw6_wa16(&self) -> &ArbRw6Wa16 {
        &self.arb_rw6_wa16
    }
    #[doc = "0x1358 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw6_ra16(&self) -> &ArbRw6Ra16 {
        &self.arb_rw6_ra16
    }
    #[doc = "0x1360 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw6_dr16(&self) -> &ArbRw6Dr16 {
        &self.arb_rw6_dr16
    }
    #[doc = "0x1390 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw7_wa16(&self) -> &ArbRw7Wa16 {
        &self.arb_rw7_wa16
    }
    #[doc = "0x1398 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw7_ra16(&self) -> &ArbRw7Ra16 {
        &self.arb_rw7_ra16
    }
    #[doc = "0x13a0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw7_dr16(&self) -> &ArbRw7Dr16 {
        &self.arb_rw7_dr16
    }
    #[doc = "0x13d0 - Endpoint Write Address value *3"]
    #[inline(always)]
    pub const fn arb_rw8_wa16(&self) -> &ArbRw8Wa16 {
        &self.arb_rw8_wa16
    }
    #[doc = "0x13d8 - Endpoint Read Address value *3"]
    #[inline(always)]
    pub const fn arb_rw8_ra16(&self) -> &ArbRw8Ra16 {
        &self.arb_rw8_ra16
    }
    #[doc = "0x13e0 - Endpoint Data Register"]
    #[inline(always)]
    pub const fn arb_rw8_dr16(&self) -> &ArbRw8Dr16 {
        &self.arb_rw8_dr16
    }
}
#[doc = "EP0_DR (rw) register accessor: Control End point EP0 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_dr`]
module"]
#[doc(alias = "EP0_DR")]
pub type Ep0Dr = crate::Reg<ep0_dr::Ep0DrSpec>;
#[doc = "Control End point EP0 Data Register"]
pub mod ep0_dr;
#[doc = "CR0 (rw) register accessor: USB control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "USB control 0 Register"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: USB control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "USB control 1 Register"]
pub mod cr1;
#[doc = "SIE_EP_INT_EN (rw) register accessor: USB SIE Data Endpoints Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep_int_en`]
module"]
#[doc(alias = "SIE_EP_INT_EN")]
pub type SieEpIntEn = crate::Reg<sie_ep_int_en::SieEpIntEnSpec>;
#[doc = "USB SIE Data Endpoints Interrupt Enable Register"]
pub mod sie_ep_int_en;
#[doc = "SIE_EP_INT_SR (rw) register accessor: USB SIE Data Endpoint Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep_int_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep_int_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep_int_sr`]
module"]
#[doc(alias = "SIE_EP_INT_SR")]
pub type SieEpIntSr = crate::Reg<sie_ep_int_sr::SieEpIntSrSpec>;
#[doc = "USB SIE Data Endpoint Interrupt Status"]
pub mod sie_ep_int_sr;
#[doc = "SIE_EP1_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep1_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep1_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cnt0`]
module"]
#[doc(alias = "SIE_EP1_CNT0")]
pub type SieEp1Cnt0 = crate::Reg<sie_ep1_cnt0::SieEp1Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt0;
#[doc = "SIE_EP1_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep1_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep1_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cnt1`]
module"]
#[doc(alias = "SIE_EP1_CNT1")]
pub type SieEp1Cnt1 = crate::Reg<sie_ep1_cnt1::SieEp1Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep1_cnt1;
#[doc = "SIE_EP1_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep1_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep1_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep1_cr0`]
module"]
#[doc(alias = "SIE_EP1_CR0")]
pub type SieEp1Cr0 = crate::Reg<sie_ep1_cr0::SieEp1Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep1_cr0;
#[doc = "USBIO_CR0 (rw) register accessor: USBIO Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr0`]
module"]
#[doc(alias = "USBIO_CR0")]
pub type UsbioCr0 = crate::Reg<usbio_cr0::UsbioCr0Spec>;
#[doc = "USBIO Control 0 Register"]
pub mod usbio_cr0;
#[doc = "USBIO_CR2 (rw) register accessor: USBIO control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr2`]
module"]
#[doc(alias = "USBIO_CR2")]
pub type UsbioCr2 = crate::Reg<usbio_cr2::UsbioCr2Spec>;
#[doc = "USBIO control 2 Register"]
pub mod usbio_cr2;
#[doc = "USBIO_CR1 (rw) register accessor: USBIO control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbio_cr1`]
module"]
#[doc(alias = "USBIO_CR1")]
pub type UsbioCr1 = crate::Reg<usbio_cr1::UsbioCr1Spec>;
#[doc = "USBIO control 1 Register"]
pub mod usbio_cr1;
#[doc = "DYN_RECONFIG (rw) register accessor: USB Dynamic reconfiguration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dyn_reconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dyn_reconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dyn_reconfig`]
module"]
#[doc(alias = "DYN_RECONFIG")]
pub type DynReconfig = crate::Reg<dyn_reconfig::DynReconfigSpec>;
#[doc = "USB Dynamic reconfiguration register"]
pub mod dyn_reconfig;
#[doc = "SOF0 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof0`]
module"]
#[doc(alias = "SOF0")]
pub type Sof0 = crate::Reg<sof0::Sof0Spec>;
#[doc = "Start Of Frame Register"]
pub mod sof0;
#[doc = "SOF1 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof1`]
module"]
#[doc(alias = "SOF1")]
pub type Sof1 = crate::Reg<sof1::Sof1Spec>;
#[doc = "Start Of Frame Register"]
pub mod sof1;
#[doc = "SIE_EP2_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep2_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep2_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cnt0`]
module"]
#[doc(alias = "SIE_EP2_CNT0")]
pub type SieEp2Cnt0 = crate::Reg<sie_ep2_cnt0::SieEp2Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt0;
#[doc = "SIE_EP2_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep2_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep2_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cnt1`]
module"]
#[doc(alias = "SIE_EP2_CNT1")]
pub type SieEp2Cnt1 = crate::Reg<sie_ep2_cnt1::SieEp2Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep2_cnt1;
#[doc = "SIE_EP2_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep2_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep2_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep2_cr0`]
module"]
#[doc(alias = "SIE_EP2_CR0")]
pub type SieEp2Cr0 = crate::Reg<sie_ep2_cr0::SieEp2Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep2_cr0;
#[doc = "OSCLK_DR0 (r) register accessor: Oscillator lock data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr0`]
module"]
#[doc(alias = "OSCLK_DR0")]
pub type OsclkDr0 = crate::Reg<osclk_dr0::OsclkDr0Spec>;
#[doc = "Oscillator lock data register 0"]
pub mod osclk_dr0;
#[doc = "OSCLK_DR1 (r) register accessor: Oscillator lock data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr1`]
module"]
#[doc(alias = "OSCLK_DR1")]
pub type OsclkDr1 = crate::Reg<osclk_dr1::OsclkDr1Spec>;
#[doc = "Oscillator lock data register 1"]
pub mod osclk_dr1;
#[doc = "EP0_CR (rw) register accessor: Endpoint0 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_cr`]
module"]
#[doc(alias = "EP0_CR")]
pub type Ep0Cr = crate::Reg<ep0_cr::Ep0CrSpec>;
#[doc = "Endpoint0 control Register"]
pub mod ep0_cr;
#[doc = "EP0_CNT (rw) register accessor: Endpoint0 count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0_cnt`]
module"]
#[doc(alias = "EP0_CNT")]
pub type Ep0Cnt = crate::Reg<ep0_cnt::Ep0CntSpec>;
#[doc = "Endpoint0 count Register"]
pub mod ep0_cnt;
#[doc = "SIE_EP3_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep3_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep3_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cnt0`]
module"]
#[doc(alias = "SIE_EP3_CNT0")]
pub type SieEp3Cnt0 = crate::Reg<sie_ep3_cnt0::SieEp3Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt0;
#[doc = "SIE_EP3_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep3_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep3_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cnt1`]
module"]
#[doc(alias = "SIE_EP3_CNT1")]
pub type SieEp3Cnt1 = crate::Reg<sie_ep3_cnt1::SieEp3Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep3_cnt1;
#[doc = "SIE_EP3_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep3_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep3_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep3_cr0`]
module"]
#[doc(alias = "SIE_EP3_CR0")]
pub type SieEp3Cr0 = crate::Reg<sie_ep3_cr0::SieEp3Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep3_cr0;
#[doc = "SIE_EP4_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep4_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep4_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cnt0`]
module"]
#[doc(alias = "SIE_EP4_CNT0")]
pub type SieEp4Cnt0 = crate::Reg<sie_ep4_cnt0::SieEp4Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt0;
#[doc = "SIE_EP4_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep4_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep4_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cnt1`]
module"]
#[doc(alias = "SIE_EP4_CNT1")]
pub type SieEp4Cnt1 = crate::Reg<sie_ep4_cnt1::SieEp4Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep4_cnt1;
#[doc = "SIE_EP4_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep4_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep4_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep4_cr0`]
module"]
#[doc(alias = "SIE_EP4_CR0")]
pub type SieEp4Cr0 = crate::Reg<sie_ep4_cr0::SieEp4Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep4_cr0;
#[doc = "SIE_EP5_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep5_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep5_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cnt0`]
module"]
#[doc(alias = "SIE_EP5_CNT0")]
pub type SieEp5Cnt0 = crate::Reg<sie_ep5_cnt0::SieEp5Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt0;
#[doc = "SIE_EP5_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep5_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep5_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cnt1`]
module"]
#[doc(alias = "SIE_EP5_CNT1")]
pub type SieEp5Cnt1 = crate::Reg<sie_ep5_cnt1::SieEp5Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep5_cnt1;
#[doc = "SIE_EP5_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep5_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep5_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep5_cr0`]
module"]
#[doc(alias = "SIE_EP5_CR0")]
pub type SieEp5Cr0 = crate::Reg<sie_ep5_cr0::SieEp5Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep5_cr0;
#[doc = "SIE_EP6_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep6_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep6_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cnt0`]
module"]
#[doc(alias = "SIE_EP6_CNT0")]
pub type SieEp6Cnt0 = crate::Reg<sie_ep6_cnt0::SieEp6Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt0;
#[doc = "SIE_EP6_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep6_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep6_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cnt1`]
module"]
#[doc(alias = "SIE_EP6_CNT1")]
pub type SieEp6Cnt1 = crate::Reg<sie_ep6_cnt1::SieEp6Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep6_cnt1;
#[doc = "SIE_EP6_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep6_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep6_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep6_cr0`]
module"]
#[doc(alias = "SIE_EP6_CR0")]
pub type SieEp6Cr0 = crate::Reg<sie_ep6_cr0::SieEp6Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep6_cr0;
#[doc = "SIE_EP7_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep7_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep7_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cnt0`]
module"]
#[doc(alias = "SIE_EP7_CNT0")]
pub type SieEp7Cnt0 = crate::Reg<sie_ep7_cnt0::SieEp7Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt0;
#[doc = "SIE_EP7_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep7_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep7_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cnt1`]
module"]
#[doc(alias = "SIE_EP7_CNT1")]
pub type SieEp7Cnt1 = crate::Reg<sie_ep7_cnt1::SieEp7Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep7_cnt1;
#[doc = "SIE_EP7_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep7_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep7_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep7_cr0`]
module"]
#[doc(alias = "SIE_EP7_CR0")]
pub type SieEp7Cr0 = crate::Reg<sie_ep7_cr0::SieEp7Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep7_cr0;
#[doc = "SIE_EP8_CNT0 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep8_cnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep8_cnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cnt0`]
module"]
#[doc(alias = "SIE_EP8_CNT0")]
pub type SieEp8Cnt0 = crate::Reg<sie_ep8_cnt0::SieEp8Cnt0Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt0;
#[doc = "SIE_EP8_CNT1 (rw) register accessor: Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep8_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep8_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cnt1`]
module"]
#[doc(alias = "SIE_EP8_CNT1")]
pub type SieEp8Cnt1 = crate::Reg<sie_ep8_cnt1::SieEp8Cnt1Spec>;
#[doc = "Non-control endpoint count register"]
pub mod sie_ep8_cnt1;
#[doc = "SIE_EP8_CR0 (rw) register accessor: Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep8_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep8_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sie_ep8_cr0`]
module"]
#[doc(alias = "SIE_EP8_CR0")]
pub type SieEp8Cr0 = crate::Reg<sie_ep8_cr0::SieEp8Cr0Spec>;
#[doc = "Non-control endpoint's control Register"]
pub mod sie_ep8_cr0;
#[doc = "ARB_EP1_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_cfg`]
module"]
#[doc(alias = "ARB_EP1_CFG")]
pub type ArbEp1Cfg = crate::Reg<arb_ep1_cfg::ArbEp1CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep1_cfg;
#[doc = "ARB_EP1_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep1_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep1_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_int_en`]
module"]
#[doc(alias = "ARB_EP1_INT_EN")]
pub type ArbEp1IntEn = crate::Reg<arb_ep1_int_en::ArbEp1IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_int_en;
#[doc = "ARB_EP1_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep1_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep1_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep1_sr`]
module"]
#[doc(alias = "ARB_EP1_SR")]
pub type ArbEp1Sr = crate::Reg<arb_ep1_sr::ArbEp1SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep1_sr;
#[doc = "ARB_RW1_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa`]
module"]
#[doc(alias = "ARB_RW1_WA")]
pub type ArbRw1Wa = crate::Reg<arb_rw1_wa::ArbRw1WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw1_wa;
#[doc = "ARB_RW1_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa_msb`]
module"]
#[doc(alias = "ARB_RW1_WA_MSB")]
pub type ArbRw1WaMsb = crate::Reg<arb_rw1_wa_msb::ArbRw1WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw1_wa_msb;
#[doc = "ARB_RW1_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra`]
module"]
#[doc(alias = "ARB_RW1_RA")]
pub type ArbRw1Ra = crate::Reg<arb_rw1_ra::ArbRw1RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw1_ra;
#[doc = "ARB_RW1_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra_msb`]
module"]
#[doc(alias = "ARB_RW1_RA_MSB")]
pub type ArbRw1RaMsb = crate::Reg<arb_rw1_ra_msb::ArbRw1RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw1_ra_msb;
#[doc = "ARB_RW1_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_dr`]
module"]
#[doc(alias = "ARB_RW1_DR")]
pub type ArbRw1Dr = crate::Reg<arb_rw1_dr::ArbRw1DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr;
#[doc = "BUF_SIZE (rw) register accessor: Dedicated Endpoint Buffer Size Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf_size`]
module"]
#[doc(alias = "BUF_SIZE")]
pub type BufSize = crate::Reg<buf_size::BufSizeSpec>;
#[doc = "Dedicated Endpoint Buffer Size Register *1"]
pub mod buf_size;
#[doc = "EP_ACTIVE (rw) register accessor: Endpoint Active Indication Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_active::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_active::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_active`]
module"]
#[doc(alias = "EP_ACTIVE")]
pub type EpActive = crate::Reg<ep_active::EpActiveSpec>;
#[doc = "Endpoint Active Indication Register *1"]
pub mod ep_active;
#[doc = "EP_TYPE (rw) register accessor: Endpoint Type (IN/OUT) Indication *1\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_type::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_type::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep_type`]
module"]
#[doc(alias = "EP_TYPE")]
pub type EpType = crate::Reg<ep_type::EpTypeSpec>;
#[doc = "Endpoint Type (IN/OUT) Indication *1"]
pub mod ep_type;
#[doc = "ARB_EP2_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_cfg`]
module"]
#[doc(alias = "ARB_EP2_CFG")]
pub type ArbEp2Cfg = crate::Reg<arb_ep2_cfg::ArbEp2CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep2_cfg;
#[doc = "ARB_EP2_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep2_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep2_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_int_en`]
module"]
#[doc(alias = "ARB_EP2_INT_EN")]
pub type ArbEp2IntEn = crate::Reg<arb_ep2_int_en::ArbEp2IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_int_en;
#[doc = "ARB_EP2_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep2_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep2_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep2_sr`]
module"]
#[doc(alias = "ARB_EP2_SR")]
pub type ArbEp2Sr = crate::Reg<arb_ep2_sr::ArbEp2SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep2_sr;
#[doc = "ARB_RW2_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa`]
module"]
#[doc(alias = "ARB_RW2_WA")]
pub type ArbRw2Wa = crate::Reg<arb_rw2_wa::ArbRw2WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw2_wa;
#[doc = "ARB_RW2_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa_msb`]
module"]
#[doc(alias = "ARB_RW2_WA_MSB")]
pub type ArbRw2WaMsb = crate::Reg<arb_rw2_wa_msb::ArbRw2WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw2_wa_msb;
#[doc = "ARB_RW2_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra`]
module"]
#[doc(alias = "ARB_RW2_RA")]
pub type ArbRw2Ra = crate::Reg<arb_rw2_ra::ArbRw2RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw2_ra;
#[doc = "ARB_RW2_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra_msb`]
module"]
#[doc(alias = "ARB_RW2_RA_MSB")]
pub type ArbRw2RaMsb = crate::Reg<arb_rw2_ra_msb::ArbRw2RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw2_ra_msb;
#[doc = "ARB_RW2_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_dr`]
module"]
#[doc(alias = "ARB_RW2_DR")]
pub type ArbRw2Dr = crate::Reg<arb_rw2_dr::ArbRw2DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr;
#[doc = "ARB_CFG (rw) register accessor: Arbiter Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_cfg`]
module"]
#[doc(alias = "ARB_CFG")]
pub type ArbCfg = crate::Reg<arb_cfg::ArbCfgSpec>;
#[doc = "Arbiter Configuration Register *1"]
pub mod arb_cfg;
#[doc = "USB_CLK_EN (rw) register accessor: USB Block Clock Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usb_clk_en`]
module"]
#[doc(alias = "USB_CLK_EN")]
pub type UsbClkEn = crate::Reg<usb_clk_en::UsbClkEnSpec>;
#[doc = "USB Block Clock Enable Register"]
pub mod usb_clk_en;
#[doc = "ARB_INT_EN (rw) register accessor: Arbiter Interrupt Enable *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_int_en`]
module"]
#[doc(alias = "ARB_INT_EN")]
pub type ArbIntEn = crate::Reg<arb_int_en::ArbIntEnSpec>;
#[doc = "Arbiter Interrupt Enable *1"]
pub mod arb_int_en;
#[doc = "ARB_INT_SR (r) register accessor: Arbiter Interrupt Status *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_int_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_int_sr`]
module"]
#[doc(alias = "ARB_INT_SR")]
pub type ArbIntSr = crate::Reg<arb_int_sr::ArbIntSrSpec>;
#[doc = "Arbiter Interrupt Status *1"]
pub mod arb_int_sr;
#[doc = "ARB_EP3_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep3_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep3_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_cfg`]
module"]
#[doc(alias = "ARB_EP3_CFG")]
pub type ArbEp3Cfg = crate::Reg<arb_ep3_cfg::ArbEp3CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep3_cfg;
#[doc = "ARB_EP3_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep3_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep3_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_int_en`]
module"]
#[doc(alias = "ARB_EP3_INT_EN")]
pub type ArbEp3IntEn = crate::Reg<arb_ep3_int_en::ArbEp3IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_int_en;
#[doc = "ARB_EP3_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep3_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep3_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep3_sr`]
module"]
#[doc(alias = "ARB_EP3_SR")]
pub type ArbEp3Sr = crate::Reg<arb_ep3_sr::ArbEp3SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep3_sr;
#[doc = "ARB_RW3_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa`]
module"]
#[doc(alias = "ARB_RW3_WA")]
pub type ArbRw3Wa = crate::Reg<arb_rw3_wa::ArbRw3WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw3_wa;
#[doc = "ARB_RW3_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa_msb`]
module"]
#[doc(alias = "ARB_RW3_WA_MSB")]
pub type ArbRw3WaMsb = crate::Reg<arb_rw3_wa_msb::ArbRw3WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw3_wa_msb;
#[doc = "ARB_RW3_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra`]
module"]
#[doc(alias = "ARB_RW3_RA")]
pub type ArbRw3Ra = crate::Reg<arb_rw3_ra::ArbRw3RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw3_ra;
#[doc = "ARB_RW3_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra_msb`]
module"]
#[doc(alias = "ARB_RW3_RA_MSB")]
pub type ArbRw3RaMsb = crate::Reg<arb_rw3_ra_msb::ArbRw3RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw3_ra_msb;
#[doc = "ARB_RW3_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_dr`]
module"]
#[doc(alias = "ARB_RW3_DR")]
pub type ArbRw3Dr = crate::Reg<arb_rw3_dr::ArbRw3DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr;
#[doc = "CWA (rw) register accessor: Common Area Write Address *1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa`]
module"]
#[doc(alias = "CWA")]
pub type Cwa = crate::Reg<cwa::CwaSpec>;
#[doc = "Common Area Write Address *1"]
pub mod cwa;
#[doc = "CWA_MSB (rw) register accessor: Endpoint Read Address value *1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa_msb`]
module"]
#[doc(alias = "CWA_MSB")]
pub type CwaMsb = crate::Reg<cwa_msb::CwaMsbSpec>;
#[doc = "Endpoint Read Address value *1"]
pub mod cwa_msb;
#[doc = "ARB_EP4_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep4_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep4_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_cfg`]
module"]
#[doc(alias = "ARB_EP4_CFG")]
pub type ArbEp4Cfg = crate::Reg<arb_ep4_cfg::ArbEp4CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep4_cfg;
#[doc = "ARB_EP4_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep4_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep4_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_int_en`]
module"]
#[doc(alias = "ARB_EP4_INT_EN")]
pub type ArbEp4IntEn = crate::Reg<arb_ep4_int_en::ArbEp4IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_int_en;
#[doc = "ARB_EP4_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep4_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep4_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep4_sr`]
module"]
#[doc(alias = "ARB_EP4_SR")]
pub type ArbEp4Sr = crate::Reg<arb_ep4_sr::ArbEp4SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep4_sr;
#[doc = "ARB_RW4_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa`]
module"]
#[doc(alias = "ARB_RW4_WA")]
pub type ArbRw4Wa = crate::Reg<arb_rw4_wa::ArbRw4WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw4_wa;
#[doc = "ARB_RW4_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa_msb`]
module"]
#[doc(alias = "ARB_RW4_WA_MSB")]
pub type ArbRw4WaMsb = crate::Reg<arb_rw4_wa_msb::ArbRw4WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw4_wa_msb;
#[doc = "ARB_RW4_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra`]
module"]
#[doc(alias = "ARB_RW4_RA")]
pub type ArbRw4Ra = crate::Reg<arb_rw4_ra::ArbRw4RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw4_ra;
#[doc = "ARB_RW4_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra_msb`]
module"]
#[doc(alias = "ARB_RW4_RA_MSB")]
pub type ArbRw4RaMsb = crate::Reg<arb_rw4_ra_msb::ArbRw4RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw4_ra_msb;
#[doc = "ARB_RW4_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_dr`]
module"]
#[doc(alias = "ARB_RW4_DR")]
pub type ArbRw4Dr = crate::Reg<arb_rw4_dr::ArbRw4DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr;
#[doc = "DMA_THRES (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres`]
module"]
#[doc(alias = "DMA_THRES")]
pub type DmaThres = crate::Reg<dma_thres::DmaThresSpec>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres;
#[doc = "DMA_THRES_MSB (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres_msb`]
module"]
#[doc(alias = "DMA_THRES_MSB")]
pub type DmaThresMsb = crate::Reg<dma_thres_msb::DmaThresMsbSpec>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres_msb;
#[doc = "ARB_EP5_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep5_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep5_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_cfg`]
module"]
#[doc(alias = "ARB_EP5_CFG")]
pub type ArbEp5Cfg = crate::Reg<arb_ep5_cfg::ArbEp5CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep5_cfg;
#[doc = "ARB_EP5_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep5_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep5_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_int_en`]
module"]
#[doc(alias = "ARB_EP5_INT_EN")]
pub type ArbEp5IntEn = crate::Reg<arb_ep5_int_en::ArbEp5IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_int_en;
#[doc = "ARB_EP5_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep5_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep5_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep5_sr`]
module"]
#[doc(alias = "ARB_EP5_SR")]
pub type ArbEp5Sr = crate::Reg<arb_ep5_sr::ArbEp5SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep5_sr;
#[doc = "ARB_RW5_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa`]
module"]
#[doc(alias = "ARB_RW5_WA")]
pub type ArbRw5Wa = crate::Reg<arb_rw5_wa::ArbRw5WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw5_wa;
#[doc = "ARB_RW5_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa_msb`]
module"]
#[doc(alias = "ARB_RW5_WA_MSB")]
pub type ArbRw5WaMsb = crate::Reg<arb_rw5_wa_msb::ArbRw5WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw5_wa_msb;
#[doc = "ARB_RW5_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra`]
module"]
#[doc(alias = "ARB_RW5_RA")]
pub type ArbRw5Ra = crate::Reg<arb_rw5_ra::ArbRw5RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw5_ra;
#[doc = "ARB_RW5_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra_msb`]
module"]
#[doc(alias = "ARB_RW5_RA_MSB")]
pub type ArbRw5RaMsb = crate::Reg<arb_rw5_ra_msb::ArbRw5RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw5_ra_msb;
#[doc = "ARB_RW5_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_dr`]
module"]
#[doc(alias = "ARB_RW5_DR")]
pub type ArbRw5Dr = crate::Reg<arb_rw5_dr::ArbRw5DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr;
#[doc = "BUS_RST_CNT (rw) register accessor: Bus Reset Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_rst_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_rst_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus_rst_cnt`]
module"]
#[doc(alias = "BUS_RST_CNT")]
pub type BusRstCnt = crate::Reg<bus_rst_cnt::BusRstCntSpec>;
#[doc = "Bus Reset Count Register"]
pub mod bus_rst_cnt;
#[doc = "ARB_EP6_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep6_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep6_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_cfg`]
module"]
#[doc(alias = "ARB_EP6_CFG")]
pub type ArbEp6Cfg = crate::Reg<arb_ep6_cfg::ArbEp6CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep6_cfg;
#[doc = "ARB_EP6_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep6_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep6_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_int_en`]
module"]
#[doc(alias = "ARB_EP6_INT_EN")]
pub type ArbEp6IntEn = crate::Reg<arb_ep6_int_en::ArbEp6IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_int_en;
#[doc = "ARB_EP6_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep6_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep6_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep6_sr`]
module"]
#[doc(alias = "ARB_EP6_SR")]
pub type ArbEp6Sr = crate::Reg<arb_ep6_sr::ArbEp6SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep6_sr;
#[doc = "ARB_RW6_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa`]
module"]
#[doc(alias = "ARB_RW6_WA")]
pub type ArbRw6Wa = crate::Reg<arb_rw6_wa::ArbRw6WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw6_wa;
#[doc = "ARB_RW6_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa_msb`]
module"]
#[doc(alias = "ARB_RW6_WA_MSB")]
pub type ArbRw6WaMsb = crate::Reg<arb_rw6_wa_msb::ArbRw6WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw6_wa_msb;
#[doc = "ARB_RW6_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra`]
module"]
#[doc(alias = "ARB_RW6_RA")]
pub type ArbRw6Ra = crate::Reg<arb_rw6_ra::ArbRw6RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw6_ra;
#[doc = "ARB_RW6_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra_msb`]
module"]
#[doc(alias = "ARB_RW6_RA_MSB")]
pub type ArbRw6RaMsb = crate::Reg<arb_rw6_ra_msb::ArbRw6RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw6_ra_msb;
#[doc = "ARB_RW6_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_dr`]
module"]
#[doc(alias = "ARB_RW6_DR")]
pub type ArbRw6Dr = crate::Reg<arb_rw6_dr::ArbRw6DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr;
#[doc = "ARB_EP7_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep7_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep7_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_cfg`]
module"]
#[doc(alias = "ARB_EP7_CFG")]
pub type ArbEp7Cfg = crate::Reg<arb_ep7_cfg::ArbEp7CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep7_cfg;
#[doc = "ARB_EP7_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep7_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep7_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_int_en`]
module"]
#[doc(alias = "ARB_EP7_INT_EN")]
pub type ArbEp7IntEn = crate::Reg<arb_ep7_int_en::ArbEp7IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_int_en;
#[doc = "ARB_EP7_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep7_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep7_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep7_sr`]
module"]
#[doc(alias = "ARB_EP7_SR")]
pub type ArbEp7Sr = crate::Reg<arb_ep7_sr::ArbEp7SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep7_sr;
#[doc = "ARB_RW7_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa`]
module"]
#[doc(alias = "ARB_RW7_WA")]
pub type ArbRw7Wa = crate::Reg<arb_rw7_wa::ArbRw7WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw7_wa;
#[doc = "ARB_RW7_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa_msb`]
module"]
#[doc(alias = "ARB_RW7_WA_MSB")]
pub type ArbRw7WaMsb = crate::Reg<arb_rw7_wa_msb::ArbRw7WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw7_wa_msb;
#[doc = "ARB_RW7_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra`]
module"]
#[doc(alias = "ARB_RW7_RA")]
pub type ArbRw7Ra = crate::Reg<arb_rw7_ra::ArbRw7RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw7_ra;
#[doc = "ARB_RW7_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra_msb`]
module"]
#[doc(alias = "ARB_RW7_RA_MSB")]
pub type ArbRw7RaMsb = crate::Reg<arb_rw7_ra_msb::ArbRw7RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw7_ra_msb;
#[doc = "ARB_RW7_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_dr`]
module"]
#[doc(alias = "ARB_RW7_DR")]
pub type ArbRw7Dr = crate::Reg<arb_rw7_dr::ArbRw7DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr;
#[doc = "ARB_EP8_CFG (rw) register accessor: Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep8_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep8_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_cfg`]
module"]
#[doc(alias = "ARB_EP8_CFG")]
pub type ArbEp8Cfg = crate::Reg<arb_ep8_cfg::ArbEp8CfgSpec>;
#[doc = "Endpoint Configuration Register *1"]
pub mod arb_ep8_cfg;
#[doc = "ARB_EP8_INT_EN (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep8_int_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep8_int_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_int_en`]
module"]
#[doc(alias = "ARB_EP8_INT_EN")]
pub type ArbEp8IntEn = crate::Reg<arb_ep8_int_en::ArbEp8IntEnSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_int_en;
#[doc = "ARB_EP8_SR (rw) register accessor: Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep8_sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep8_sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_ep8_sr`]
module"]
#[doc(alias = "ARB_EP8_SR")]
pub type ArbEp8Sr = crate::Reg<arb_ep8_sr::ArbEp8SrSpec>;
#[doc = "Endpoint Interrupt Enable Register *1"]
pub mod arb_ep8_sr;
#[doc = "ARB_RW8_WA (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_wa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_wa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa`]
module"]
#[doc(alias = "ARB_RW8_WA")]
pub type ArbRw8Wa = crate::Reg<arb_rw8_wa::ArbRw8WaSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw8_wa;
#[doc = "ARB_RW8_WA_MSB (rw) register accessor: Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_wa_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_wa_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa_msb`]
module"]
#[doc(alias = "ARB_RW8_WA_MSB")]
pub type ArbRw8WaMsb = crate::Reg<arb_rw8_wa_msb::ArbRw8WaMsbSpec>;
#[doc = "Endpoint Write Address value *1, *2"]
pub mod arb_rw8_wa_msb;
#[doc = "ARB_RW8_RA (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_ra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_ra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra`]
module"]
#[doc(alias = "ARB_RW8_RA")]
pub type ArbRw8Ra = crate::Reg<arb_rw8_ra::ArbRw8RaSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw8_ra;
#[doc = "ARB_RW8_RA_MSB (rw) register accessor: Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_ra_msb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_ra_msb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra_msb`]
module"]
#[doc(alias = "ARB_RW8_RA_MSB")]
pub type ArbRw8RaMsb = crate::Reg<arb_rw8_ra_msb::ArbRw8RaMsbSpec>;
#[doc = "Endpoint Read Address value *1, *2"]
pub mod arb_rw8_ra_msb;
#[doc = "ARB_RW8_DR (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_dr`]
module"]
#[doc(alias = "ARB_RW8_DR")]
pub type ArbRw8Dr = crate::Reg<arb_rw8_dr::ArbRw8DrSpec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr;
#[doc = "MEM_DATA (rw) register accessor: DATA\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_data`]
module"]
#[doc(alias = "MEM_DATA")]
pub type MemData = crate::Reg<mem_data::MemDataSpec>;
#[doc = "DATA"]
pub mod mem_data;
#[doc = "SOF16 (r) register accessor: Start Of Frame Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sof16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sof16`]
module"]
#[doc(alias = "SOF16")]
pub type Sof16 = crate::Reg<sof16::Sof16Spec>;
#[doc = "Start Of Frame Register"]
pub mod sof16;
#[doc = "OSCLK_DR16 (r) register accessor: Oscillator lock data register\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osclk_dr16`]
module"]
#[doc(alias = "OSCLK_DR16")]
pub type OsclkDr16 = crate::Reg<osclk_dr16::OsclkDr16Spec>;
#[doc = "Oscillator lock data register"]
pub mod osclk_dr16;
#[doc = "ARB_RW1_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_wa16`]
module"]
#[doc(alias = "ARB_RW1_WA16")]
pub type ArbRw1Wa16 = crate::Reg<arb_rw1_wa16::ArbRw1Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw1_wa16;
#[doc = "ARB_RW1_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_ra16`]
module"]
#[doc(alias = "ARB_RW1_RA16")]
pub type ArbRw1Ra16 = crate::Reg<arb_rw1_ra16::ArbRw1Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw1_ra16;
#[doc = "ARB_RW1_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw1_dr16`]
module"]
#[doc(alias = "ARB_RW1_DR16")]
pub type ArbRw1Dr16 = crate::Reg<arb_rw1_dr16::ArbRw1Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw1_dr16;
#[doc = "ARB_RW2_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_wa16`]
module"]
#[doc(alias = "ARB_RW2_WA16")]
pub type ArbRw2Wa16 = crate::Reg<arb_rw2_wa16::ArbRw2Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw2_wa16;
#[doc = "ARB_RW2_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_ra16`]
module"]
#[doc(alias = "ARB_RW2_RA16")]
pub type ArbRw2Ra16 = crate::Reg<arb_rw2_ra16::ArbRw2Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw2_ra16;
#[doc = "ARB_RW2_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw2_dr16`]
module"]
#[doc(alias = "ARB_RW2_DR16")]
pub type ArbRw2Dr16 = crate::Reg<arb_rw2_dr16::ArbRw2Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw2_dr16;
#[doc = "ARB_RW3_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_wa16`]
module"]
#[doc(alias = "ARB_RW3_WA16")]
pub type ArbRw3Wa16 = crate::Reg<arb_rw3_wa16::ArbRw3Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw3_wa16;
#[doc = "ARB_RW3_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_ra16`]
module"]
#[doc(alias = "ARB_RW3_RA16")]
pub type ArbRw3Ra16 = crate::Reg<arb_rw3_ra16::ArbRw3Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw3_ra16;
#[doc = "ARB_RW3_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw3_dr16`]
module"]
#[doc(alias = "ARB_RW3_DR16")]
pub type ArbRw3Dr16 = crate::Reg<arb_rw3_dr16::ArbRw3Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw3_dr16;
#[doc = "CWA16 (rw) register accessor: Common Area Write Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwa16`]
module"]
#[doc(alias = "CWA16")]
pub type Cwa16 = crate::Reg<cwa16::Cwa16Spec>;
#[doc = "Common Area Write Address"]
pub mod cwa16;
#[doc = "ARB_RW4_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_wa16`]
module"]
#[doc(alias = "ARB_RW4_WA16")]
pub type ArbRw4Wa16 = crate::Reg<arb_rw4_wa16::ArbRw4Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw4_wa16;
#[doc = "ARB_RW4_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_ra16`]
module"]
#[doc(alias = "ARB_RW4_RA16")]
pub type ArbRw4Ra16 = crate::Reg<arb_rw4_ra16::ArbRw4Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw4_ra16;
#[doc = "ARB_RW4_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw4_dr16`]
module"]
#[doc(alias = "ARB_RW4_DR16")]
pub type ArbRw4Dr16 = crate::Reg<arb_rw4_dr16::ArbRw4Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw4_dr16;
#[doc = "DMA_THRES16 (rw) register accessor: DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_thres16`]
module"]
#[doc(alias = "DMA_THRES16")]
pub type DmaThres16 = crate::Reg<dma_thres16::DmaThres16Spec>;
#[doc = "DMA Burst / Threshold Configuration"]
pub mod dma_thres16;
#[doc = "ARB_RW5_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_wa16`]
module"]
#[doc(alias = "ARB_RW5_WA16")]
pub type ArbRw5Wa16 = crate::Reg<arb_rw5_wa16::ArbRw5Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw5_wa16;
#[doc = "ARB_RW5_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_ra16`]
module"]
#[doc(alias = "ARB_RW5_RA16")]
pub type ArbRw5Ra16 = crate::Reg<arb_rw5_ra16::ArbRw5Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw5_ra16;
#[doc = "ARB_RW5_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw5_dr16`]
module"]
#[doc(alias = "ARB_RW5_DR16")]
pub type ArbRw5Dr16 = crate::Reg<arb_rw5_dr16::ArbRw5Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw5_dr16;
#[doc = "ARB_RW6_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_wa16`]
module"]
#[doc(alias = "ARB_RW6_WA16")]
pub type ArbRw6Wa16 = crate::Reg<arb_rw6_wa16::ArbRw6Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw6_wa16;
#[doc = "ARB_RW6_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_ra16`]
module"]
#[doc(alias = "ARB_RW6_RA16")]
pub type ArbRw6Ra16 = crate::Reg<arb_rw6_ra16::ArbRw6Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw6_ra16;
#[doc = "ARB_RW6_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw6_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw6_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw6_dr16`]
module"]
#[doc(alias = "ARB_RW6_DR16")]
pub type ArbRw6Dr16 = crate::Reg<arb_rw6_dr16::ArbRw6Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw6_dr16;
#[doc = "ARB_RW7_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_wa16`]
module"]
#[doc(alias = "ARB_RW7_WA16")]
pub type ArbRw7Wa16 = crate::Reg<arb_rw7_wa16::ArbRw7Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw7_wa16;
#[doc = "ARB_RW7_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_ra16`]
module"]
#[doc(alias = "ARB_RW7_RA16")]
pub type ArbRw7Ra16 = crate::Reg<arb_rw7_ra16::ArbRw7Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw7_ra16;
#[doc = "ARB_RW7_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw7_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw7_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw7_dr16`]
module"]
#[doc(alias = "ARB_RW7_DR16")]
pub type ArbRw7Dr16 = crate::Reg<arb_rw7_dr16::ArbRw7Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw7_dr16;
#[doc = "ARB_RW8_WA16 (rw) register accessor: Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_wa16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_wa16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_wa16`]
module"]
#[doc(alias = "ARB_RW8_WA16")]
pub type ArbRw8Wa16 = crate::Reg<arb_rw8_wa16::ArbRw8Wa16Spec>;
#[doc = "Endpoint Write Address value *3"]
pub mod arb_rw8_wa16;
#[doc = "ARB_RW8_RA16 (rw) register accessor: Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_ra16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_ra16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_ra16`]
module"]
#[doc(alias = "ARB_RW8_RA16")]
pub type ArbRw8Ra16 = crate::Reg<arb_rw8_ra16::ArbRw8Ra16Spec>;
#[doc = "Endpoint Read Address value *3"]
pub mod arb_rw8_ra16;
#[doc = "ARB_RW8_DR16 (rw) register accessor: Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_dr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_dr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arb_rw8_dr16`]
module"]
#[doc(alias = "ARB_RW8_DR16")]
pub type ArbRw8Dr16 = crate::Reg<arb_rw8_dr16::ArbRw8Dr16Spec>;
#[doc = "Endpoint Data Register"]
pub mod arb_rw8_dr16;
