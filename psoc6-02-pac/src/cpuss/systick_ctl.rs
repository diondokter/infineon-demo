#[doc = "Register `SYSTICK_CTL` reader"]
pub type R = crate::R<SystickCtlSpec>;
#[doc = "Register `SYSTICK_CTL` writer"]
pub type W = crate::W<SystickCtlSpec>;
#[doc = "Field `TENMS` reader - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
pub type TenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `CLOCK_SOURCE` reader - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
pub type ClockSourceR = crate::FieldReader;
#[doc = "Field `CLOCK_SOURCE` writer - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
pub type ClockSourceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKEW` reader - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
pub type SkewR = crate::BitReader;
#[doc = "Field `SKEW` writer - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
pub type SkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOREF` reader - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
pub type NorefR = crate::BitReader;
#[doc = "Field `NOREF` writer - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
pub type NorefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:25 - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    pub fn clock_source(&self) -> ClockSourceR {
        ClockSourceR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the number of clock source cycles (minus 1) that make up 10 ms. E.g., for a 32,768 Hz reference clock, TENMS is 328 - 1 = 327."]
    #[inline(always)]
    #[must_use]
    pub fn tenms(&mut self) -> TenmsW<SystickCtlSpec> {
        TenmsW::new(self, 0)
    }
    #[doc = "Bits 24:25 - Specifies an external clock source: '0': The low frequency clock 'clk_lf' is selected. The precision of this clock depends on whether the low frequency clock source is a SRSS internal RC oscillator (imprecise) or a device external crystal oscillator (precise). '1': The internal main oscillator (IMO) clock 'clk_imo' is selected. The MXS40 platform uses a fixed frequency IMO clock. o '2': The external crystal oscillator (ECO) clock 'clk_eco' is selected. '3': The SRSS 'clk_timer' is selected ('clk_timer' is a divided/gated version of 'clk_hf' or 'clk_imo'). Note: If NOREF is '1', the CLOCK_SOURCE value is NOT used. Note: It is SW's responsibility to provide the correct NOREF, SKEW and TENMS field values for the selected clock source."]
    #[inline(always)]
    #[must_use]
    pub fn clock_source(&mut self) -> ClockSourceW<SystickCtlSpec> {
        ClockSourceW::new(self, 24)
    }
    #[doc = "Bit 30 - Specifies the precision of the clock source and if the TENMS field represents exactly 10 ms (clock source frequency is a multiple of 100 Hz). This affects the suitability of the SysTick timer as a SW real-time clock: '0': Precise. '1': Imprecise."]
    #[inline(always)]
    #[must_use]
    pub fn skew(&mut self) -> SkewW<SystickCtlSpec> {
        SkewW::new(self, 30)
    }
    #[doc = "Bit 31 - Specifies if an external clock source is provided: '0': An external clock source is provided. '1': An external clock source is NOT provided and only the CPU internal clock can be used as SysTick timer clock source."]
    #[inline(always)]
    #[must_use]
    pub fn noref(&mut self) -> NorefW<SystickCtlSpec> {
        NorefW::new(self, 31)
    }
}
#[doc = "SysTick timer control\n\nYou can [`read`](crate::Reg::read) this register and get [`systick_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systick_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystickCtlSpec;
impl crate::RegisterSpec for SystickCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systick_ctl::R`](R) reader structure"]
impl crate::Readable for SystickCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`systick_ctl::W`](W) writer structure"]
impl crate::Writable for SystickCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTICK_CTL to value 0x4000_0147"]
impl crate::Resettable for SystickCtlSpec {
    const RESET_VALUE: u32 = 0x4000_0147;
}
