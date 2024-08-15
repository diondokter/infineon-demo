#[doc = "Register `CLK_TIMER_CTL` reader"]
pub type R = crate::R<ClkTimerCtlSpec>;
#[doc = "Register `CLK_TIMER_CTL` writer"]
pub type W = crate::W<ClkTimerCtlSpec>;
#[doc = "Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerSel {
    #[doc = "0: IMO - Internal Main Oscillator"]
    Imo = 0,
    #[doc = "1: Select the output of the predivider configured by TIMER_HF0_DIV."]
    Hf0Div = 1,
}
impl From<TimerSel> for bool {
    #[inline(always)]
    fn from(variant: TimerSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_SEL` reader - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
pub type TimerSelR = crate::BitReader<TimerSel>;
impl TimerSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerSel {
        match self.bits {
            false => TimerSel::Imo,
            true => TimerSel::Hf0Div,
        }
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn is_imo(&self) -> bool {
        *self == TimerSel::Imo
    }
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    #[inline(always)]
    pub fn is_hf0_div(&self) -> bool {
        *self == TimerSel::Hf0Div
    }
}
#[doc = "Field `TIMER_SEL` writer - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
pub type TimerSelW<'a, REG> = crate::BitWriter<'a, REG, TimerSel>;
impl<'a, REG> TimerSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn imo(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSel::Imo)
    }
    #[doc = "Select the output of the predivider configured by TIMER_HF0_DIV."]
    #[inline(always)]
    pub fn hf0_div(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSel::Hf0Div)
    }
}
#[doc = "Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimerHf0Div {
    #[doc = "0: Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    NoDiv = 0,
    #[doc = "1: Divide HFCLK0 by 2."]
    DivBy2 = 1,
    #[doc = "2: Divide HFCLK0 by 4."]
    DivBy4 = 2,
    #[doc = "3: Divide HFCLK0 by 8."]
    DivBy8 = 3,
}
impl From<TimerHf0Div> for u8 {
    #[inline(always)]
    fn from(variant: TimerHf0Div) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimerHf0Div {
    type Ux = u8;
}
impl crate::IsEnum for TimerHf0Div {}
#[doc = "Field `TIMER_HF0_DIV` reader - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
pub type TimerHf0DivR = crate::FieldReader<TimerHf0Div>;
impl TimerHf0DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerHf0Div {
        match self.bits {
            0 => TimerHf0Div::NoDiv,
            1 => TimerHf0Div::DivBy2,
            2 => TimerHf0Div::DivBy4,
            3 => TimerHf0Div::DivBy8,
            _ => unreachable!(),
        }
    }
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == TimerHf0Div::NoDiv
    }
    #[doc = "Divide HFCLK0 by 2."]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == TimerHf0Div::DivBy2
    }
    #[doc = "Divide HFCLK0 by 4."]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == TimerHf0Div::DivBy4
    }
    #[doc = "Divide HFCLK0 by 8."]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == TimerHf0Div::DivBy8
    }
}
#[doc = "Field `TIMER_HF0_DIV` writer - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
pub type TimerHf0DivW<'a, REG> = crate::FieldWriter<'a, REG, 2, TimerHf0Div, crate::Safe>;
impl<'a, REG> TimerHf0DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Transparent mode, feed through selected clock source w/o dividing or correcting duty cycle."]
    #[inline(always)]
    pub fn no_div(self) -> &'a mut crate::W<REG> {
        self.variant(TimerHf0Div::NoDiv)
    }
    #[doc = "Divide HFCLK0 by 2."]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(TimerHf0Div::DivBy2)
    }
    #[doc = "Divide HFCLK0 by 4."]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(TimerHf0Div::DivBy4)
    }
    #[doc = "Divide HFCLK0 by 8."]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(TimerHf0Div::DivBy8)
    }
}
#[doc = "Field `TIMER_DIV` reader - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
pub type TimerDivR = crate::FieldReader;
#[doc = "Field `TIMER_DIV` writer - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
pub type TimerDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENABLE` reader - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    pub fn timer_sel(&self) -> TimerSelR {
        TimerSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    pub fn timer_hf0_div(&self) -> TimerHf0DivR {
        TimerHf0DivR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    pub fn timer_div(&self) -> TimerDivR {
        TimerDivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select source for TIMERCLK. The output of this mux can be further divided using TIMER_DIV."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TimerSelW<ClkTimerCtlSpec> {
        TimerSelW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Predivider used when HF0_DIV is selected in TIMER_SEL. If HFCLK0 frequency is less than 100MHz and has approximately 50 percent duty cycle, then no division is required (NO_DIV). Otherwise, select a divide ratio of 2, 4, or 8 before selected HF0_DIV as the timer clock."]
    #[inline(always)]
    #[must_use]
    pub fn timer_hf0_div(&mut self) -> TimerHf0DivW<ClkTimerCtlSpec> {
        TimerHf0DivW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Divide selected timer clock source by (1+TIMER_DIV). The output of this divider is TIMERCLK Allows for integer divisions in the range \\[1, 256\\]. Do not change this setting while the timer is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn timer_div(&mut self) -> TimerDivW<ClkTimerCtlSpec> {
        TimerDivW::new(self, 16)
    }
    #[doc = "Bit 31 - Enable for TIMERCLK. 0: TIMERCLK is off 1: TIMERCLK is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkTimerCtlSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Timer Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_timer_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_timer_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTimerCtlSpec;
impl crate::RegisterSpec for ClkTimerCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_timer_ctl::R`](R) reader structure"]
impl crate::Readable for ClkTimerCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_timer_ctl::W`](W) writer structure"]
impl crate::Writable for ClkTimerCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TIMER_CTL to value 0x0007_0000"]
impl crate::Resettable for ClkTimerCtlSpec {
    const RESET_VALUE: u32 = 0x0007_0000;
}
