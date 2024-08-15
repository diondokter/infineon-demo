#[doc = "Register `CM0_CLOCK_CTL` reader"]
pub type R = crate::R<Cm0ClockCtlSpec>;
#[doc = "Register `CM0_CLOCK_CTL` writer"]
pub type W = crate::W<Cm0ClockCtlSpec>;
#[doc = "Field `SLOW_INT_DIV` reader - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type SlowIntDivR = crate::FieldReader;
#[doc = "Field `SLOW_INT_DIV` writer - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type SlowIntDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PERI_INT_DIV` reader - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi &lt;= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
pub type PeriIntDivR = crate::FieldReader;
#[doc = "Field `PERI_INT_DIV` writer - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi &lt;= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
pub type PeriIntDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn slow_int_div(&self) -> SlowIntDivR {
        SlowIntDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi &lt;= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    pub fn peri_int_div(&self) -> PeriIntDivR {
        PeriIntDivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the slow clock divider (from the peripheral clock 'clk_peri' to the slow clock 'clk_slow'). Integer division by (1+SLOW_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(SLOW_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn slow_int_div(&mut self) -> SlowIntDivW<Cm0ClockCtlSpec> {
        SlowIntDivW::new(self, 8)
    }
    #[doc = "Bits 24:31 - Specifies the peripheral clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_peri'). Integer division by (1+PERI_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(PERI_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode. Note that Fperi &lt;= Fperi_max. Fperi_max is likely to be smaller than Fhf_max. In other words, if Fhf = Fhf_max, PERI_INT_DIV should not be set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn peri_int_div(&mut self) -> PeriIntDivW<Cm0ClockCtlSpec> {
        PeriIntDivW::new(self, 24)
    }
}
#[doc = "CM0+ clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_clock_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_clock_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0ClockCtlSpec;
impl crate::RegisterSpec for Cm0ClockCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_clock_ctl::R`](R) reader structure"]
impl crate::Readable for Cm0ClockCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_clock_ctl::W`](W) writer structure"]
impl crate::Writable for Cm0ClockCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_CLOCK_CTL to value 0"]
impl crate::Resettable for Cm0ClockCtlSpec {
    const RESET_VALUE: u32 = 0;
}
