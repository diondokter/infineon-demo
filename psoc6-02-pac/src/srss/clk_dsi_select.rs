#[doc = "Register `CLK_DSI_SELECT[%s]` reader"]
pub type R = crate::R<ClkDsiSelectSpec>;
#[doc = "Register `CLK_DSI_SELECT[%s]` writer"]
pub type W = crate::W<ClkDsiSelectSpec>;
#[doc = "Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_PATH_SELECT register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DsiMux {
    #[doc = "0: DSI0 - dsi_out\\[0\\]"]
    DsiOut0 = 0,
    #[doc = "1: DSI1 - dsi_out\\[1\\]"]
    DsiOut1 = 1,
    #[doc = "2: DSI2 - dsi_out\\[2\\]"]
    DsiOut2 = 2,
    #[doc = "3: DSI3 - dsi_out\\[3\\]"]
    DsiOut3 = 3,
    #[doc = "4: DSI4 - dsi_out\\[4\\]"]
    DsiOut4 = 4,
    #[doc = "5: DSI5 - dsi_out\\[5\\]"]
    DsiOut5 = 5,
    #[doc = "6: DSI6 - dsi_out\\[6\\]"]
    DsiOut6 = 6,
    #[doc = "7: DSI7 - dsi_out\\[7\\]"]
    DsiOut7 = 7,
    #[doc = "8: DSI8 - dsi_out\\[8\\]"]
    DsiOut8 = 8,
    #[doc = "9: DSI9 - dsi_out\\[9\\]"]
    DsiOut9 = 9,
    #[doc = "10: DSI10 - dsi_out\\[10\\]"]
    DsiOut10 = 10,
    #[doc = "11: DSI11 - dsi_out\\[11\\]"]
    DsiOut11 = 11,
    #[doc = "12: DSI12 - dsi_out\\[12\\]"]
    DsiOut12 = 12,
    #[doc = "13: DSI13 - dsi_out\\[13\\]"]
    DsiOut13 = 13,
    #[doc = "14: DSI14 - dsi_out\\[14\\]"]
    DsiOut14 = 14,
    #[doc = "15: DSI15 - dsi_out\\[15\\]"]
    DsiOut15 = 15,
    #[doc = "16: ILO - Internal Low-speed Oscillator"]
    Ilo = 16,
    #[doc = "17: WCO - Watch-Crystal Oscillator"]
    Wco = 17,
    #[doc = "18: ALTLF - Alternate Low-Frequency Clock"]
    Altlf = 18,
    #[doc = "19: PILO - Precision Internal Low-speed Oscillator"]
    Pilo = 19,
}
impl From<DsiMux> for u8 {
    #[inline(always)]
    fn from(variant: DsiMux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DsiMux {
    type Ux = u8;
}
impl crate::IsEnum for DsiMux {}
#[doc = "Field `DSI_MUX` reader - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_PATH_SELECT register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
pub type DsiMuxR = crate::FieldReader<DsiMux>;
impl DsiMuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DsiMux> {
        match self.bits {
            0 => Some(DsiMux::DsiOut0),
            1 => Some(DsiMux::DsiOut1),
            2 => Some(DsiMux::DsiOut2),
            3 => Some(DsiMux::DsiOut3),
            4 => Some(DsiMux::DsiOut4),
            5 => Some(DsiMux::DsiOut5),
            6 => Some(DsiMux::DsiOut6),
            7 => Some(DsiMux::DsiOut7),
            8 => Some(DsiMux::DsiOut8),
            9 => Some(DsiMux::DsiOut9),
            10 => Some(DsiMux::DsiOut10),
            11 => Some(DsiMux::DsiOut11),
            12 => Some(DsiMux::DsiOut12),
            13 => Some(DsiMux::DsiOut13),
            14 => Some(DsiMux::DsiOut14),
            15 => Some(DsiMux::DsiOut15),
            16 => Some(DsiMux::Ilo),
            17 => Some(DsiMux::Wco),
            18 => Some(DsiMux::Altlf),
            19 => Some(DsiMux::Pilo),
            _ => None,
        }
    }
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    #[inline(always)]
    pub fn is_dsi_out0(&self) -> bool {
        *self == DsiMux::DsiOut0
    }
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    #[inline(always)]
    pub fn is_dsi_out1(&self) -> bool {
        *self == DsiMux::DsiOut1
    }
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    #[inline(always)]
    pub fn is_dsi_out2(&self) -> bool {
        *self == DsiMux::DsiOut2
    }
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    #[inline(always)]
    pub fn is_dsi_out3(&self) -> bool {
        *self == DsiMux::DsiOut3
    }
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    #[inline(always)]
    pub fn is_dsi_out4(&self) -> bool {
        *self == DsiMux::DsiOut4
    }
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    #[inline(always)]
    pub fn is_dsi_out5(&self) -> bool {
        *self == DsiMux::DsiOut5
    }
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    #[inline(always)]
    pub fn is_dsi_out6(&self) -> bool {
        *self == DsiMux::DsiOut6
    }
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    #[inline(always)]
    pub fn is_dsi_out7(&self) -> bool {
        *self == DsiMux::DsiOut7
    }
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    #[inline(always)]
    pub fn is_dsi_out8(&self) -> bool {
        *self == DsiMux::DsiOut8
    }
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    #[inline(always)]
    pub fn is_dsi_out9(&self) -> bool {
        *self == DsiMux::DsiOut9
    }
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    #[inline(always)]
    pub fn is_dsi_out10(&self) -> bool {
        *self == DsiMux::DsiOut10
    }
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    #[inline(always)]
    pub fn is_dsi_out11(&self) -> bool {
        *self == DsiMux::DsiOut11
    }
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    #[inline(always)]
    pub fn is_dsi_out12(&self) -> bool {
        *self == DsiMux::DsiOut12
    }
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    #[inline(always)]
    pub fn is_dsi_out13(&self) -> bool {
        *self == DsiMux::DsiOut13
    }
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    #[inline(always)]
    pub fn is_dsi_out14(&self) -> bool {
        *self == DsiMux::DsiOut14
    }
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    #[inline(always)]
    pub fn is_dsi_out15(&self) -> bool {
        *self == DsiMux::DsiOut15
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == DsiMux::Ilo
    }
    #[doc = "WCO - Watch-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == DsiMux::Wco
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == DsiMux::Altlf
    }
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == DsiMux::Pilo
    }
}
#[doc = "Field `DSI_MUX` writer - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_PATH_SELECT register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
pub type DsiMuxW<'a, REG> = crate::FieldWriter<'a, REG, 5, DsiMux>;
impl<'a, REG> DsiMuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DSI0 - dsi_out\\[0\\]"]
    #[inline(always)]
    pub fn dsi_out0(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut0)
    }
    #[doc = "DSI1 - dsi_out\\[1\\]"]
    #[inline(always)]
    pub fn dsi_out1(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut1)
    }
    #[doc = "DSI2 - dsi_out\\[2\\]"]
    #[inline(always)]
    pub fn dsi_out2(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut2)
    }
    #[doc = "DSI3 - dsi_out\\[3\\]"]
    #[inline(always)]
    pub fn dsi_out3(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut3)
    }
    #[doc = "DSI4 - dsi_out\\[4\\]"]
    #[inline(always)]
    pub fn dsi_out4(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut4)
    }
    #[doc = "DSI5 - dsi_out\\[5\\]"]
    #[inline(always)]
    pub fn dsi_out5(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut5)
    }
    #[doc = "DSI6 - dsi_out\\[6\\]"]
    #[inline(always)]
    pub fn dsi_out6(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut6)
    }
    #[doc = "DSI7 - dsi_out\\[7\\]"]
    #[inline(always)]
    pub fn dsi_out7(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut7)
    }
    #[doc = "DSI8 - dsi_out\\[8\\]"]
    #[inline(always)]
    pub fn dsi_out8(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut8)
    }
    #[doc = "DSI9 - dsi_out\\[9\\]"]
    #[inline(always)]
    pub fn dsi_out9(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut9)
    }
    #[doc = "DSI10 - dsi_out\\[10\\]"]
    #[inline(always)]
    pub fn dsi_out10(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut10)
    }
    #[doc = "DSI11 - dsi_out\\[11\\]"]
    #[inline(always)]
    pub fn dsi_out11(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut11)
    }
    #[doc = "DSI12 - dsi_out\\[12\\]"]
    #[inline(always)]
    pub fn dsi_out12(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut12)
    }
    #[doc = "DSI13 - dsi_out\\[13\\]"]
    #[inline(always)]
    pub fn dsi_out13(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut13)
    }
    #[doc = "DSI14 - dsi_out\\[14\\]"]
    #[inline(always)]
    pub fn dsi_out14(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut14)
    }
    #[doc = "DSI15 - dsi_out\\[15\\]"]
    #[inline(always)]
    pub fn dsi_out15(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::DsiOut15)
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::Ilo)
    }
    #[doc = "WCO - Watch-Crystal Oscillator"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::Wco)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::Altlf)
    }
    #[doc = "PILO - Precision Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(DsiMux::Pilo)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_PATH_SELECT register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    pub fn dsi_mux(&self) -> DsiMuxR {
        DsiMuxR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects a DSI source or low frequency clock for use in a clock path. The output of this mux can be selected for clock PATH&lt;i> using CLK_PATH_SELECT register. Using the output of this mux as HFCLK source will result in undefined behavior. It can be used to clocks to DSI or to the reference inputs of FLL/PLL, subject to the frequency limits of those circuits. This mux is not glitch free, so do not change the selection while it is an actively selected clock."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_mux(&mut self) -> DsiMuxW<ClkDsiSelectSpec> {
        DsiMuxW::new(self, 0)
    }
}
#[doc = "Clock DSI Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_dsi_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_dsi_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDsiSelectSpec;
impl crate::RegisterSpec for ClkDsiSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_dsi_select::R`](R) reader structure"]
impl crate::Readable for ClkDsiSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_dsi_select::W`](W) writer structure"]
impl crate::Writable for ClkDsiSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_DSI_SELECT[%s]
to value 0"]
impl crate::Resettable for ClkDsiSelectSpec {
    const RESET_VALUE: u32 = 0;
}
