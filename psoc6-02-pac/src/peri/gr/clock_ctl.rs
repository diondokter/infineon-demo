#[doc = "Register `CLOCK_CTL` reader"]
pub type R = crate::R<ClockCtlSpec>;
#[doc = "Register `CLOCK_CTL` writer"]
pub type W = crate::W<ClockCtlSpec>;
#[doc = "Field `INT8_DIV` reader - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int8DivR = crate::FieldReader;
#[doc = "Field `INT8_DIV` writer - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int8DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int8_div(&self) -> Int8DivR {
        Int8DivR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Specifies a group clock divider (from the peripheral clock 'clk_peri' to the group clock 'clk_group\\[3/4/5/...15\\]'). Integer division by (1+INT8_DIV). Allows for integer divisions in the range \\[1, 256\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int8_div(&mut self) -> Int8DivW<ClockCtlSpec> {
        Int8DivW::new(self, 8)
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
#[doc = "`reset()` method sets CLOCK_CTL to value 0"]
impl crate::Resettable for ClockCtlSpec {
    const RESET_VALUE: u32 = 0;
}
