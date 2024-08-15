#[doc = "Register `CLK_OUTPUT_SLOW` reader"]
pub type R = crate::R<ClkOutputSlowSpec>;
#[doc = "Register `CLK_OUTPUT_SLOW` writer"]
pub type W = crate::W<ClkOutputSlowSpec>;
#[doc = "Select signal for slow clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlowSel0 {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    Nc = 0,
    #[doc = "1: Internal Low Speed Oscillator (ILO)"]
    Ilo = 1,
    #[doc = "2: Watch-Crystal Oscillator (WCO)"]
    Wco = 2,
    #[doc = "3: Root of the Backup domain clock tree (BAK)"]
    Bak = 3,
    #[doc = "4: Alternate low-frequency clock input to SRSS (ALTLF)"]
    Altlf = 4,
    #[doc = "5: Root of the low-speed clock tree (LFCLK)"]
    Lfclk = 5,
    #[doc = "6: Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    Imo = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    Slpctrl = 7,
    #[doc = "8: Precision Internal Low Speed Oscillator (PILO)"]
    Pilo = 8,
}
impl From<SlowSel0> for u8 {
    #[inline(always)]
    fn from(variant: SlowSel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlowSel0 {
    type Ux = u8;
}
impl crate::IsEnum for SlowSel0 {}
#[doc = "Field `SLOW_SEL0` reader - Select signal for slow clock output #0"]
pub type SlowSel0R = crate::FieldReader<SlowSel0>;
impl SlowSel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SlowSel0> {
        match self.bits {
            0 => Some(SlowSel0::Nc),
            1 => Some(SlowSel0::Ilo),
            2 => Some(SlowSel0::Wco),
            3 => Some(SlowSel0::Bak),
            4 => Some(SlowSel0::Altlf),
            5 => Some(SlowSel0::Lfclk),
            6 => Some(SlowSel0::Imo),
            7 => Some(SlowSel0::Slpctrl),
            8 => Some(SlowSel0::Pilo),
            _ => None,
        }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == SlowSel0::Nc
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == SlowSel0::Ilo
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == SlowSel0::Wco
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn is_bak(&self) -> bool {
        *self == SlowSel0::Bak
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == SlowSel0::Altlf
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == SlowSel0::Lfclk
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == SlowSel0::Imo
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn is_slpctrl(&self) -> bool {
        *self == SlowSel0::Slpctrl
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == SlowSel0::Pilo
    }
}
#[doc = "Field `SLOW_SEL0` writer - Select signal for slow clock output #0"]
pub type SlowSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, SlowSel0>;
impl<'a, REG> SlowSel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Nc)
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Ilo)
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Wco)
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn bak(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Bak)
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Altlf)
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Lfclk)
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Imo)
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn slpctrl(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Slpctrl)
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel0::Pilo)
    }
}
#[doc = "Select signal for slow clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SlowSel1 {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    Nc = 0,
    #[doc = "1: Internal Low Speed Oscillator (ILO)"]
    Ilo = 1,
    #[doc = "2: Watch-Crystal Oscillator (WCO)"]
    Wco = 2,
    #[doc = "3: Root of the Backup domain clock tree (BAK)"]
    Bak = 3,
    #[doc = "4: Alternate low-frequency clock input to SRSS (ALTLF)"]
    Altlf = 4,
    #[doc = "5: Root of the low-speed clock tree (LFCLK)"]
    Lfclk = 5,
    #[doc = "6: Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    Imo = 6,
    #[doc = "7: Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    Slpctrl = 7,
    #[doc = "8: Precision Internal Low Speed Oscillator (PILO)"]
    Pilo = 8,
}
impl From<SlowSel1> for u8 {
    #[inline(always)]
    fn from(variant: SlowSel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SlowSel1 {
    type Ux = u8;
}
impl crate::IsEnum for SlowSel1 {}
#[doc = "Field `SLOW_SEL1` reader - Select signal for slow clock output #1"]
pub type SlowSel1R = crate::FieldReader<SlowSel1>;
impl SlowSel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SlowSel1> {
        match self.bits {
            0 => Some(SlowSel1::Nc),
            1 => Some(SlowSel1::Ilo),
            2 => Some(SlowSel1::Wco),
            3 => Some(SlowSel1::Bak),
            4 => Some(SlowSel1::Altlf),
            5 => Some(SlowSel1::Lfclk),
            6 => Some(SlowSel1::Imo),
            7 => Some(SlowSel1::Slpctrl),
            8 => Some(SlowSel1::Pilo),
            _ => None,
        }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == SlowSel1::Nc
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == SlowSel1::Ilo
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == SlowSel1::Wco
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn is_bak(&self) -> bool {
        *self == SlowSel1::Bak
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == SlowSel1::Altlf
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == SlowSel1::Lfclk
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == SlowSel1::Imo
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn is_slpctrl(&self) -> bool {
        *self == SlowSel1::Slpctrl
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == SlowSel1::Pilo
    }
}
#[doc = "Field `SLOW_SEL1` writer - Select signal for slow clock output #1"]
pub type SlowSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, SlowSel1>;
impl<'a, REG> SlowSel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Nc)
    }
    #[doc = "Internal Low Speed Oscillator (ILO)"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Ilo)
    }
    #[doc = "Watch-Crystal Oscillator (WCO)"]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Wco)
    }
    #[doc = "Root of the Backup domain clock tree (BAK)"]
    #[inline(always)]
    pub fn bak(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Bak)
    }
    #[doc = "Alternate low-frequency clock input to SRSS (ALTLF)"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Altlf)
    }
    #[doc = "Root of the low-speed clock tree (LFCLK)"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Lfclk)
    }
    #[doc = "Internal Main Oscillator (IMO). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Imo)
    }
    #[doc = "Sleep Controller clock (SLPCTRL). This is grouped with the slow clocks so it can be observed during DEEPSLEEP entry/exit."]
    #[inline(always)]
    pub fn slpctrl(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Slpctrl)
    }
    #[doc = "Precision Internal Low Speed Oscillator (PILO)"]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(SlowSel1::Pilo)
    }
}
impl R {
    #[doc = "Bits 0:3 - Select signal for slow clock output #0"]
    #[inline(always)]
    pub fn slow_sel0(&self) -> SlowSel0R {
        SlowSel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select signal for slow clock output #1"]
    #[inline(always)]
    pub fn slow_sel1(&self) -> SlowSel1R {
        SlowSel1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for slow clock output #0"]
    #[inline(always)]
    #[must_use]
    pub fn slow_sel0(&mut self) -> SlowSel0W<ClkOutputSlowSpec> {
        SlowSel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select signal for slow clock output #1"]
    #[inline(always)]
    #[must_use]
    pub fn slow_sel1(&mut self) -> SlowSel1W<ClkOutputSlowSpec> {
        SlowSel1W::new(self, 4)
    }
}
#[doc = "Slow Clock Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_output_slow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_output_slow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkOutputSlowSpec;
impl crate::RegisterSpec for ClkOutputSlowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_output_slow::R`](R) reader structure"]
impl crate::Readable for ClkOutputSlowSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_output_slow::W`](W) writer structure"]
impl crate::Writable for ClkOutputSlowSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_OUTPUT_SLOW to value 0"]
impl crate::Resettable for ClkOutputSlowSpec {
    const RESET_VALUE: u32 = 0;
}
