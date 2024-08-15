#[doc = "Register `CLK_PLL_CONFIG[%s]` reader"]
pub type R = crate::R<ClkPllConfigSpec>;
#[doc = "Register `CLK_PLL_CONFIG[%s]` writer"]
pub type W = crate::W<ClkPllConfigSpec>;
#[doc = "Field `FEEDBACK_DIV` reader - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FeedbackDivR = crate::FieldReader;
#[doc = "Field `FEEDBACK_DIV` writer - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
pub type FeedbackDivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REFERENCE_DIV` reader - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type ReferenceDivR = crate::FieldReader;
#[doc = "Field `REFERENCE_DIV` writer - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
pub type ReferenceDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OutputDivR = crate::FieldReader;
#[doc = "Field `OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OutputDivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PLL_LF_MODE` reader - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PllLfModeR = crate::BitReader;
#[doc = "Field `PLL_LF_MODE` writer - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
pub type PllLfModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BypassSel {
    #[doc = "0: Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    Auto = 0,
    #[doc = "1: Same as AUTO"]
    Auto1 = 1,
    #[doc = "2: Select PLL reference input (bypass mode). Ignores lock indicator"]
    PllRef = 2,
    #[doc = "3: Select PLL output. Ignores lock indicator."]
    PllOut = 3,
}
impl From<BypassSel> for u8 {
    #[inline(always)]
    fn from(variant: BypassSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BypassSel {
    type Ux = u8;
}
impl crate::IsEnum for BypassSel {}
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BypassSelR = crate::FieldReader<BypassSel>;
impl BypassSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BypassSel {
        match self.bits {
            0 => BypassSel::Auto,
            1 => BypassSel::Auto1,
            2 => BypassSel::PllRef,
            3 => BypassSel::PllOut,
            _ => unreachable!(),
        }
    }
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BypassSel::Auto
    }
    #[doc = "Same as AUTO"]
    #[inline(always)]
    pub fn is_auto1(&self) -> bool {
        *self == BypassSel::Auto1
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn is_pll_ref(&self) -> bool {
        *self == BypassSel::PllRef
    }
    #[doc = "Select PLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn is_pll_out(&self) -> bool {
        *self == BypassSel::PllOut
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
pub type BypassSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, BypassSel, crate::Safe>;
impl<'a, REG> BypassSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::Auto)
    }
    #[doc = "Same as AUTO"]
    #[inline(always)]
    pub fn auto1(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::Auto1)
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn pll_ref(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::PllRef)
    }
    #[doc = "Select PLL output. Ignores lock indicator."]
    #[inline(always)]
    pub fn pll_out(self) -> &'a mut crate::W<REG> {
        self.variant(BypassSel::PllOut)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&self) -> FeedbackDivR {
        FeedbackDivR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&self) -> ReferenceDivR {
        ReferenceDivR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&self) -> OutputDivR {
        OutputDivR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    pub fn pll_lf_mode(&self) -> PllLfModeR {
        PllLfModeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BypassSelR {
        BypassSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-21: illegal (undefined behavior) 22: divide by 22 ... 112: divide by 112 >112: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn feedback_div(&mut self) -> FeedbackDivW<ClkPllConfigSpec> {
        FeedbackDivW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 20: divide by 20 others: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn reference_div(&mut self) -> ReferenceDivW<ClkPllConfigSpec> {
        ReferenceDivW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn output_div(&mut self) -> OutputDivW<ClkPllConfigSpec> {
        OutputDivW::new(self, 16)
    }
    #[doc = "Bit 27 - VCO frequency range selection. Configure this bit according to the targeted VCO frequency. Do not change this setting while the PLL is enabled. 0: VCO frequency is \\[200MHz, 400MHz\\]
1: VCO frequency is \\[170MHz, 200MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn pll_lf_mode(&mut self) -> PllLfModeW<ClkPllConfigSpec> {
        PllLfModeW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_sel(&mut self) -> BypassSelW<ClkPllConfigSpec> {
        BypassSelW::new(self, 28)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. To disable the PLL, first deselect it using .BYPASS_SEL=PLL_REF, wait at least six PLL clock cycles, and then disable it with .ENABLE=0. Fpll = (FEEDBACK_DIV) * (Fref / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled 1: Block is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkPllConfigSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPllConfigSpec;
impl crate::RegisterSpec for ClkPllConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll_config::R`](R) reader structure"]
impl crate::Readable for ClkPllConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_pll_config::W`](W) writer structure"]
impl crate::Writable for ClkPllConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PLL_CONFIG[%s]
to value 0x0002_0116"]
impl crate::Resettable for ClkPllConfigSpec {
    const RESET_VALUE: u32 = 0x0002_0116;
}
