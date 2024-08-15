#[doc = "Register `DIV_16_5_CTL[%s]` reader"]
pub type R = crate::R<Div16_5CtlSpec>;
#[doc = "Register `DIV_16_5_CTL[%s]` writer"]
pub type W = crate::W<Div16_5CtlSpec>;
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub type EnR = crate::BitReader;
#[doc = "Field `FRAC5_DIV` reader - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Frac5DivR = crate::FieldReader;
#[doc = "Field `FRAC5_DIV` writer - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Frac5DivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT16_DIV` reader - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int16DivR = crate::FieldReader<u16>;
#[doc = "Field `INT16_DIV` writer - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int16DivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn frac5_div(&self) -> Frac5DivR {
        Frac5DivR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:23 - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int16_div(&self) -> Int16DivR {
        Int16DivR::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn frac5_div(&mut self) -> Frac5DivW<Div16_5CtlSpec> {
        Frac5DivW::new(self, 3)
    }
    #[doc = "Bits 8:23 - Integer division by (1+INT16_DIV). Allows for integer divisions in the range \\[1, 65,536\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 65,536 31/32\\]
in 1/32 increments. For the generation of a divided clock, the division range is restricted to \\[2, 65,536 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 65,536\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int16_div(&mut self) -> Int16DivW<Div16_5CtlSpec> {
        Int16DivW::new(self, 8)
    }
}
#[doc = "Divider control (for 16.5 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_16_5_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_16_5_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Div16_5CtlSpec;
impl crate::RegisterSpec for Div16_5CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_16_5_ctl::R`](R) reader structure"]
impl crate::Readable for Div16_5CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`div_16_5_ctl::W`](W) writer structure"]
impl crate::Writable for Div16_5CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_16_5_CTL[%s]
to value 0"]
impl crate::Resettable for Div16_5CtlSpec {
    const RESET_VALUE: u32 = 0;
}
