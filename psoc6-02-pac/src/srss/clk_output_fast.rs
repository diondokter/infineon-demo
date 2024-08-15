#[doc = "Register `CLK_OUTPUT_FAST` reader"]
pub type R = crate::R<ClkOutputFastSpec>;
#[doc = "Register `CLK_OUTPUT_FAST` writer"]
pub type W = crate::W<ClkOutputFastSpec>;
#[doc = "Select signal for fast clock output #0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FastSel0 {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    Nc = 0,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    Eco = 1,
    #[doc = "2: External clock input (EXTCLK)"]
    Extclk = 2,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    Althf = 3,
    #[doc = "4: Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    Timerclk = 4,
    #[doc = "5: Selects the clock path chosen by PATH_SEL0 field"]
    PathSel0 = 5,
    #[doc = "6: Selects the output of the HFCLK_SEL0 mux"]
    HfclkSel0 = 6,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    SlowSel0 = 7,
}
impl From<FastSel0> for u8 {
    #[inline(always)]
    fn from(variant: FastSel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FastSel0 {
    type Ux = u8;
}
impl crate::IsEnum for FastSel0 {}
#[doc = "Field `FAST_SEL0` reader - Select signal for fast clock output #0"]
pub type FastSel0R = crate::FieldReader<FastSel0>;
impl FastSel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FastSel0> {
        match self.bits {
            0 => Some(FastSel0::Nc),
            1 => Some(FastSel0::Eco),
            2 => Some(FastSel0::Extclk),
            3 => Some(FastSel0::Althf),
            4 => Some(FastSel0::Timerclk),
            5 => Some(FastSel0::PathSel0),
            6 => Some(FastSel0::HfclkSel0),
            7 => Some(FastSel0::SlowSel0),
            _ => None,
        }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == FastSel0::Nc
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == FastSel0::Eco
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == FastSel0::Extclk
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == FastSel0::Althf
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn is_timerclk(&self) -> bool {
        *self == FastSel0::Timerclk
    }
    #[doc = "Selects the clock path chosen by PATH_SEL0 field"]
    #[inline(always)]
    pub fn is_path_sel0(&self) -> bool {
        *self == FastSel0::PathSel0
    }
    #[doc = "Selects the output of the HFCLK_SEL0 mux"]
    #[inline(always)]
    pub fn is_hfclk_sel0(&self) -> bool {
        *self == FastSel0::HfclkSel0
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    #[inline(always)]
    pub fn is_slow_sel0(&self) -> bool {
        *self == FastSel0::SlowSel0
    }
}
#[doc = "Field `FAST_SEL0` writer - Select signal for fast clock output #0"]
pub type FastSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, FastSel0>;
impl<'a, REG> FastSel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL0 and HFCLK_SEL0."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::Nc)
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::Eco)
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::Extclk)
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::Althf)
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn timerclk(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::Timerclk)
    }
    #[doc = "Selects the clock path chosen by PATH_SEL0 field"]
    #[inline(always)]
    pub fn path_sel0(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::PathSel0)
    }
    #[doc = "Selects the output of the HFCLK_SEL0 mux"]
    #[inline(always)]
    pub fn hfclk_sel0(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::HfclkSel0)
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL0"]
    #[inline(always)]
    pub fn slow_sel0(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel0::SlowSel0)
    }
}
#[doc = "Field `PATH_SEL0` reader - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PathSel0R = crate::FieldReader;
#[doc = "Field `PATH_SEL0` writer - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PathSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFCLK_SEL0` reader - Selects a HFCLK tree for use in fast clock output #0"]
pub type HfclkSel0R = crate::FieldReader;
#[doc = "Field `HFCLK_SEL0` writer - Selects a HFCLK tree for use in fast clock output #0"]
pub type HfclkSel0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Select signal for fast clock output #1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FastSel1 {
    #[doc = "0: Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    Nc = 0,
    #[doc = "1: External Crystal Oscillator (ECO)"]
    Eco = 1,
    #[doc = "2: External clock input (EXTCLK)"]
    Extclk = 2,
    #[doc = "3: Alternate High-Frequency (ALTHF) clock input to SRSS"]
    Althf = 3,
    #[doc = "4: Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    Timerclk = 4,
    #[doc = "5: Selects the clock path chosen by PATH_SEL1 field"]
    PathSel1 = 5,
    #[doc = "6: Selects the output of the HFCLK_SEL1 mux"]
    HfclkSel1 = 6,
    #[doc = "7: Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    SlowSel1 = 7,
}
impl From<FastSel1> for u8 {
    #[inline(always)]
    fn from(variant: FastSel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FastSel1 {
    type Ux = u8;
}
impl crate::IsEnum for FastSel1 {}
#[doc = "Field `FAST_SEL1` reader - Select signal for fast clock output #1"]
pub type FastSel1R = crate::FieldReader<FastSel1>;
impl FastSel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FastSel1> {
        match self.bits {
            0 => Some(FastSel1::Nc),
            1 => Some(FastSel1::Eco),
            2 => Some(FastSel1::Extclk),
            3 => Some(FastSel1::Althf),
            4 => Some(FastSel1::Timerclk),
            5 => Some(FastSel1::PathSel1),
            6 => Some(FastSel1::HfclkSel1),
            7 => Some(FastSel1::SlowSel1),
            _ => None,
        }
    }
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == FastSel1::Nc
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn is_eco(&self) -> bool {
        *self == FastSel1::Eco
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == FastSel1::Extclk
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn is_althf(&self) -> bool {
        *self == FastSel1::Althf
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn is_timerclk(&self) -> bool {
        *self == FastSel1::Timerclk
    }
    #[doc = "Selects the clock path chosen by PATH_SEL1 field"]
    #[inline(always)]
    pub fn is_path_sel1(&self) -> bool {
        *self == FastSel1::PathSel1
    }
    #[doc = "Selects the output of the HFCLK_SEL1 mux"]
    #[inline(always)]
    pub fn is_hfclk_sel1(&self) -> bool {
        *self == FastSel1::HfclkSel1
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    #[inline(always)]
    pub fn is_slow_sel1(&self) -> bool {
        *self == FastSel1::SlowSel1
    }
}
#[doc = "Field `FAST_SEL1` writer - Select signal for fast clock output #1"]
pub type FastSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4, FastSel1>;
impl<'a, REG> FastSel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled - output is 0. For power savings, clocks are blocked before entering any muxes, including PATH_SEL1 and HFCLK_SEL1."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::Nc)
    }
    #[doc = "External Crystal Oscillator (ECO)"]
    #[inline(always)]
    pub fn eco(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::Eco)
    }
    #[doc = "External clock input (EXTCLK)"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::Extclk)
    }
    #[doc = "Alternate High-Frequency (ALTHF) clock input to SRSS"]
    #[inline(always)]
    pub fn althf(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::Althf)
    }
    #[doc = "Timer clock. It is grouped with the fast clocks because it may be a gated version of a fast clock, and therefore may have a short high pulse."]
    #[inline(always)]
    pub fn timerclk(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::Timerclk)
    }
    #[doc = "Selects the clock path chosen by PATH_SEL1 field"]
    #[inline(always)]
    pub fn path_sel1(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::PathSel1)
    }
    #[doc = "Selects the output of the HFCLK_SEL1 mux"]
    #[inline(always)]
    pub fn hfclk_sel1(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::HfclkSel1)
    }
    #[doc = "Selects the output of CLK_OUTPUT_SLOW.SLOW_SEL1"]
    #[inline(always)]
    pub fn slow_sel1(self) -> &'a mut crate::W<REG> {
        self.variant(FastSel1::SlowSel1)
    }
}
#[doc = "Field `PATH_SEL1` reader - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PathSel1R = crate::FieldReader;
#[doc = "Field `PATH_SEL1` writer - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
pub type PathSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HFCLK_SEL1` reader - Selects a HFCLK tree for use in fast clock output #1 logic"]
pub type HfclkSel1R = crate::FieldReader;
#[doc = "Field `HFCLK_SEL1` writer - Selects a HFCLK tree for use in fast clock output #1 logic"]
pub type HfclkSel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select signal for fast clock output #0"]
    #[inline(always)]
    pub fn fast_sel0(&self) -> FastSel0R {
        FastSel0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel0(&self) -> PathSel0R {
        PathSel0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    pub fn hfclk_sel0(&self) -> HfclkSel0R {
        HfclkSel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Select signal for fast clock output #1"]
    #[inline(always)]
    pub fn fast_sel1(&self) -> FastSel1R {
        FastSel1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    pub fn path_sel1(&self) -> PathSel1R {
        PathSel1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    pub fn hfclk_sel1(&self) -> HfclkSel1R {
        HfclkSel1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select signal for fast clock output #0"]
    #[inline(always)]
    #[must_use]
    pub fn fast_sel0(&mut self) -> FastSel0W<ClkOutputFastSpec> {
        FastSel0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selects a clock path to use in fast clock output #0 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    #[must_use]
    pub fn path_sel0(&mut self) -> PathSel0W<ClkOutputFastSpec> {
        PathSel0W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Selects a HFCLK tree for use in fast clock output #0"]
    #[inline(always)]
    #[must_use]
    pub fn hfclk_sel0(&mut self) -> HfclkSel0W<ClkOutputFastSpec> {
        HfclkSel0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Select signal for fast clock output #1"]
    #[inline(always)]
    #[must_use]
    pub fn fast_sel1(&mut self) -> FastSel1W<ClkOutputFastSpec> {
        FastSel1W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Selects a clock path to use in fast clock output #1 logic. 0: FLL output 1-15: PLL output on path1-path15 (if available)"]
    #[inline(always)]
    #[must_use]
    pub fn path_sel1(&mut self) -> PathSel1W<ClkOutputFastSpec> {
        PathSel1W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Selects a HFCLK tree for use in fast clock output #1 logic"]
    #[inline(always)]
    #[must_use]
    pub fn hfclk_sel1(&mut self) -> HfclkSel1W<ClkOutputFastSpec> {
        HfclkSel1W::new(self, 24)
    }
}
#[doc = "Fast Clock Output Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_output_fast::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_output_fast::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkOutputFastSpec;
impl crate::RegisterSpec for ClkOutputFastSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_output_fast::R`](R) reader structure"]
impl crate::Readable for ClkOutputFastSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_output_fast::W`](W) writer structure"]
impl crate::Writable for ClkOutputFastSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_OUTPUT_FAST to value 0"]
impl crate::Resettable for ClkOutputFastSpec {
    const RESET_VALUE: u32 = 0;
}
