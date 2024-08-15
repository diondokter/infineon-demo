#[doc = "Register `CM4_CLOCK_CTL` reader"]
pub type R = crate::R<Cm4ClockCtlSpec>;
#[doc = "Register `CM4_CLOCK_CTL` writer"]
pub type W = crate::W<Cm4ClockCtlSpec>;
#[doc = "Field `FAST_INT_DIV` reader - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FastIntDivR = crate::FieldReader;
#[doc = "Field `FAST_INT_DIV` writer - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type FastIntDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn fast_int_div(&self) -> FastIntDivR {
        FastIntDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies the fast clock divider (from the high frequency clock 'clk_hf' to the peripheral clock 'clk_fast'). Integer division by (1+FAST_INT_DIV). Allows for integer divisions in the range \\[1, 256\\]
(FAST_INT_DIV is in the range \\[0, 255\\]). Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn fast_int_div(&mut self) -> FastIntDivW<Cm4ClockCtlSpec> {
        FastIntDivW::new(self, 8)
    }
}
#[doc = "CM4 clock control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_clock_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_clock_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4ClockCtlSpec;
impl crate::RegisterSpec for Cm4ClockCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_clock_ctl::R`](R) reader structure"]
impl crate::Readable for Cm4ClockCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_clock_ctl::W`](W) writer structure"]
impl crate::Writable for Cm4ClockCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_CLOCK_CTL to value 0"]
impl crate::Resettable for Cm4ClockCtlSpec {
    const RESET_VALUE: u32 = 0;
}
