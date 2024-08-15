#[doc = "Register `CLK_SELECT` reader"]
pub type R = crate::R<ClkSelectSpec>;
#[doc = "Register `CLK_SELECT` writer"]
pub type W = crate::W<ClkSelectSpec>;
#[doc = "Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LfclkSel {
    #[doc = "0: ILO - Internal Low-speed Oscillator"]
    Ilo = 0,
    #[doc = "1: WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    Wco = 1,
    #[doc = "2: ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    Altlf = 2,
    #[doc = "3: PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    Pilo = 3,
}
impl From<LfclkSel> for u8 {
    #[inline(always)]
    fn from(variant: LfclkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LfclkSel {
    type Ux = u8;
}
impl crate::IsEnum for LfclkSel {}
#[doc = "Field `LFCLK_SEL` reader - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LfclkSelR = crate::FieldReader<LfclkSel>;
impl LfclkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LfclkSel {
        match self.bits {
            0 => LfclkSel::Ilo,
            1 => LfclkSel::Wco,
            2 => LfclkSel::Altlf,
            3 => LfclkSel::Pilo,
            _ => unreachable!(),
        }
    }
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn is_ilo(&self) -> bool {
        *self == LfclkSel::Ilo
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == LfclkSel::Wco
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn is_altlf(&self) -> bool {
        *self == LfclkSel::Altlf
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn is_pilo(&self) -> bool {
        *self == LfclkSel::Pilo
    }
}
#[doc = "Field `LFCLK_SEL` writer - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
pub type LfclkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, LfclkSel, crate::Safe>;
impl<'a, REG> LfclkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ILO - Internal Low-speed Oscillator"]
    #[inline(always)]
    pub fn ilo(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Ilo)
    }
    #[doc = "WCO - Watch-Crystal Oscillator. Requires Backup domain to be present and properly configured (including external watch crystal, if used)."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Wco)
    }
    #[doc = "ALTLF - Alternate Low-Frequency Clock. Capability is product-specific"]
    #[inline(always)]
    pub fn altlf(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Altlf)
    }
    #[doc = "PILO - Precision ILO. If present, it works in DEEPSLEEP and higher modes. Does not work in HIBERNATE mode."]
    #[inline(always)]
    pub fn pilo(self) -> &'a mut crate::W<REG> {
        self.variant(LfclkSel::Pilo)
    }
}
#[doc = "Field `PUMP_SEL` reader - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PumpSelR = crate::FieldReader;
#[doc = "Field `PUMP_SEL` writer - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
pub type PumpSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PumpDiv {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing."]
    NoDiv = 0,
    #[doc = "1: Divide selected clock source by 2"]
    DivBy2 = 1,
    #[doc = "2: Divide selected clock source by 4"]
    DivBy4 = 2,
    #[doc = "3: Divide selected clock source by 8"]
    DivBy8 = 3,
    #[doc = "4: Divide selected clock source by 16"]
    DivBy16 = 4,
}
impl From<PumpDiv> for u8 {
    #[inline(always)]
    fn from(variant: PumpDiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PumpDiv {
    type Ux = u8;
}
impl crate::IsEnum for PumpDiv {}
#[doc = "Field `PUMP_DIV` reader - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PumpDivR = crate::FieldReader<PumpDiv>;
impl PumpDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PumpDiv> {
        match self.bits {
            0 => Some(PumpDiv::NoDiv),
            1 => Some(PumpDiv::DivBy2),
            2 => Some(PumpDiv::DivBy4),
            3 => Some(PumpDiv::DivBy8),
            4 => Some(PumpDiv::DivBy16),
            _ => None,
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == PumpDiv::NoDiv
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == PumpDiv::DivBy2
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == PumpDiv::DivBy4
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == PumpDiv::DivBy8
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn is_div_by_16(&self) -> bool {
        *self == PumpDiv::DivBy16
    }
}
#[doc = "Field `PUMP_DIV` writer - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
pub type PumpDivW<'a, REG> = crate::FieldWriter<'a, REG, 3, PumpDiv>;
impl<'a, REG> PumpDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(PumpDiv::NoDiv)
    }
    #[doc = "Divide selected clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(PumpDiv::DivBy2)
    }
    #[doc = "Divide selected clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(PumpDiv::DivBy4)
    }
    #[doc = "Divide selected clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(PumpDiv::DivBy8)
    }
    #[doc = "Divide selected clock source by 16"]
    #[inline(always)]
    pub fn div_by_16(self) -> &'a mut crate::W<REG> {
        self.variant(PumpDiv::DivBy16)
    }
}
#[doc = "Field `PUMP_ENABLE` reader - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PumpEnableR = crate::BitReader;
#[doc = "Field `PUMP_ENABLE` writer - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
pub type PumpEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    pub fn lfclk_sel(&self) -> LfclkSelR {
        LfclkSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    pub fn pump_sel(&self) -> PumpSelR {
        PumpSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    pub fn pump_div(&self) -> PumpDivR {
        PumpDivR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    pub fn pump_enable(&self) -> PumpEnableR {
        PumpEnableR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select source for LFCLK. Note that not all products support all clock sources. Selecting a clock source that is not supported will result in undefined behavior. Writes to this field are ignored unless the WDT is unlocked using WDT_LOCK register."]
    #[inline(always)]
    #[must_use]
    pub fn lfclk_sel(&mut self) -> LfclkSelW<ClkSelectSpec> {
        LfclkSelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Selects clock PATH&lt;k>, where k=PUMP_SEL. The output of this mux goes to the PUMP_DIV to make PUMPCLK Each product has a specific number of available clock paths. Selecting a path that is not implemented on a product will result in undefined behavior. Note that this is not a glitch free mux."]
    #[inline(always)]
    #[must_use]
    pub fn pump_sel(&mut self) -> PumpSelW<ClkSelectSpec> {
        PumpSelW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Division ratio for PUMPCLK. Uses selected PUMP_SEL clock as the source."]
    #[inline(always)]
    #[must_use]
    pub fn pump_div(&mut self) -> PumpDivW<ClkSelectSpec> {
        PumpDivW::new(self, 12)
    }
    #[doc = "Bit 15 - Enable the pump clock. PUMP_ENABLE and the PUMP_SEL mux are not glitch-free to minimize side-effects, avoid changing the PUMP_SEL and PUMP_DIV while changing PUMP_ENABLE. To change the settings, do the following: 1) If the pump clock is enabled, write PUMP_ENABLE=0 without changing PUMP_SEL and PUMP_DIV. 2) Change PUMP_SEL and PUMP_DIV to desired settings with PUMP_ENABLE=0. 3) Write PUMP_ENABLE=1 without changing PUMP_SEL and PUMP_DIV."]
    #[inline(always)]
    #[must_use]
    pub fn pump_enable(&mut self) -> PumpEnableW<ClkSelectSpec> {
        PumpEnableW::new(self, 15)
    }
}
#[doc = "Clock selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_select::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_select::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSelectSpec;
impl crate::RegisterSpec for ClkSelectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_select::R`](R) reader structure"]
impl crate::Readable for ClkSelectSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_select::W`](W) writer structure"]
impl crate::Writable for ClkSelectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_SELECT to value 0"]
impl crate::Resettable for ClkSelectSpec {
    const RESET_VALUE: u32 = 0;
}
