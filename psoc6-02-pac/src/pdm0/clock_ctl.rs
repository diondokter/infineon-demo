#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<ClockCtlSpec>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<ClockCtlSpec>;
#[doc = "PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkClockDiv {
    #[doc = "0: Divide by 1"]
    Divby1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    Divby2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    Divby3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    Divby4 = 3,
}
impl From<ClkClockDiv> for u8 {
    #[inline(always)]
    fn from(variant: ClkClockDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkClockDiv {
    type Ux = u8;
}
impl crate::IsEnum for ClkClockDiv {}
#[doc = "Field `CLK_CLOCK_DIV` reader - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type ClkClockDivR = crate::FieldReader<ClkClockDiv>;
impl ClkClockDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkClockDiv {
        match self.bits {
            0 => ClkClockDiv::Divby1,
            1 => ClkClockDiv::Divby2,
            2 => ClkClockDiv::Divby3,
            3 => ClkClockDiv::Divby4,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == ClkClockDiv::Divby1
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == ClkClockDiv::Divby2
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == ClkClockDiv::Divby3
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == ClkClockDiv::Divby4
    }
}
#[doc = "Field `CLK_CLOCK_DIV` writer - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
pub type ClkClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkClockDiv, crate::Safe>;
impl<'a, REG> ClkClockDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkClockDiv::Divby1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut crate::W<REG> {
        self.variant(ClkClockDiv::Divby2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut crate::W<REG> {
        self.variant(ClkClockDiv::Divby3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut crate::W<REG> {
        self.variant(ClkClockDiv::Divby4)
    }
}
#[doc = "MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MclkqClockDiv {
    #[doc = "0: Divide by 1"]
    Divby1 = 0,
    #[doc = "1: Divide by 2 (no 50 percent duty cycle)"]
    Divby2 = 1,
    #[doc = "2: Divide by 3 (no 50 percent duty cycle)"]
    Divby3 = 2,
    #[doc = "3: Divide by 4 (no 50 percent duty cycle)"]
    Divby4 = 3,
}
impl From<MclkqClockDiv> for u8 {
    #[inline(always)]
    fn from(variant: MclkqClockDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MclkqClockDiv {
    type Ux = u8;
}
impl crate::IsEnum for MclkqClockDiv {}
#[doc = "Field `MCLKQ_CLOCK_DIV` reader - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MclkqClockDivR = crate::FieldReader<MclkqClockDiv>;
impl MclkqClockDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MclkqClockDiv {
        match self.bits {
            0 => MclkqClockDiv::Divby1,
            1 => MclkqClockDiv::Divby2,
            2 => MclkqClockDiv::Divby3,
            3 => MclkqClockDiv::Divby4,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_divby1(&self) -> bool {
        *self == MclkqClockDiv::Divby1
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby2(&self) -> bool {
        *self == MclkqClockDiv::Divby2
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby3(&self) -> bool {
        *self == MclkqClockDiv::Divby3
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn is_divby4(&self) -> bool {
        *self == MclkqClockDiv::Divby4
    }
}
#[doc = "Field `MCLKQ_CLOCK_DIV` writer - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
pub type MclkqClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 2, MclkqClockDiv, crate::Safe>;
impl<'a, REG> MclkqClockDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divby1(self) -> &'a mut crate::W<REG> {
        self.variant(MclkqClockDiv::Divby1)
    }
    #[doc = "Divide by 2 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby2(self) -> &'a mut crate::W<REG> {
        self.variant(MclkqClockDiv::Divby2)
    }
    #[doc = "Divide by 3 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby3(self) -> &'a mut crate::W<REG> {
        self.variant(MclkqClockDiv::Divby3)
    }
    #[doc = "Divide by 4 (no 50 percent duty cycle)"]
    #[inline(always)]
    pub fn divby4(self) -> &'a mut crate::W<REG> {
        self.variant(MclkqClockDiv::Divby4)
    }
}
#[doc = "Field `CKO_CLOCK_DIV` reader - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CkoClockDivR = crate::FieldReader;
#[doc = "Field `CKO_CLOCK_DIV` writer - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
pub type CkoClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINC_RATE` reader - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SincRateR = crate::FieldReader;
#[doc = "Field `SINC_RATE` writer - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
pub type SincRateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    pub fn clk_clock_div(&self) -> ClkClockDivR {
        ClkClockDivR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    pub fn mclkq_clock_div(&self) -> MclkqClockDivR {
        MclkqClockDivR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    pub fn cko_clock_div(&self) -> CkoClockDivR {
        CkoClockDivR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    pub fn sinc_rate(&self) -> SincRateR {
        SincRateR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDM CLK (FPDM_CLK) (1st divider): This configures a frequency of PDM CLK. The configured frequency is used to operate PDM core. I.e. the frequency is input to MCLKQ_CLOCK_DIV register. Note: configure a frequency of PDM CLK as lower than or equal 50MHz with this divider."]
    #[inline(always)]
    #[must_use]
    pub fn clk_clock_div(&mut self) -> ClkClockDivW<ClockCtlSpec> {
        ClkClockDivW::new(self, 0)
    }
    #[doc = "Bits 4:5 - MCLKQ divider (2nd divider) (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.DIV_MCLKQ)"]
    #[inline(always)]
    #[must_use]
    pub fn mclkq_clock_div(&mut self) -> MclkqClockDivW<ClockCtlSpec> {
        MclkqClockDivW::new(self, 4)
    }
    #[doc = "Bits 8:11 - PDM CKO (FPDM_CKO) clock divider (3rd divider): FPDM_CKO = MCLKQ / (CKO_CLOCK_DIV + 1) Note: To configure '0' to this field is prohibited. (Note: PDM_CKO is configured by MCLKQ_CLOCK_DIV, CLK_CLOCK_DIV and CKO_CLOCK_DIV. ) (Note: These bits are connected to AR36U12.PDM_CORE_CFG.MCLKDIV)"]
    #[inline(always)]
    #[must_use]
    pub fn cko_clock_div(&mut self) -> CkoClockDivW<ClockCtlSpec> {
        CkoClockDivW::new(self, 8)
    }
    #[doc = "Bits 16:22 - SINC Decimation Rate. For details, see the data sheet provided by Archband. Oversampling Ratio = Decimation Rate = 2 X SINC_RATE (Note: These bits are connected to AR36U12.PDM_CORE_CFG.SINC_RATE)"]
    #[inline(always)]
    #[must_use]
    pub fn sinc_rate(&mut self) -> SincRateW<ClockCtlSpec> {
        SincRateW::new(self, 16)
    }
}
#[doc = "Clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtlSpec;
impl crate::RegisterSpec for ClockCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctl::R`](R) reader structure"]
impl crate::Readable for ClockCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctl::W`](W) writer structure"]
impl crate::Writable for ClockCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK_CTL to value 0x0020_0310"]
impl crate::Resettable for ClockCtlSpec {
    const RESET_VALUE: u32 = 0x0020_0310;
}
