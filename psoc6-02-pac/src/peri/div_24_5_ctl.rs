#[doc = "Register `DIV_24_5_CTL[%s]` reader"]
pub type R = crate::R<Div24_5CtlSpec>;
#[doc = "Register `DIV_24_5_CTL[%s]` writer"]
pub type W = crate::W<Div24_5CtlSpec>;
#[doc = "Field `EN` reader - Divider enabled. HW sets this field to '1' as a result of an ENABLE command. HW sets this field to '0' as a result on a DISABLE command. Note that this field is retained. As a result, the divider does NOT have to be re-enabled after transitioning from DeepSleep to Active power mode."]
pub type EnR = crate::BitReader;
#[doc = "Field `FRAC5_DIV` reader - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Frac5DivR = crate::FieldReader;
#[doc = "Field `FRAC5_DIV` writer - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Frac5DivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `INT24_DIV` reader - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int24DivR = crate::FieldReader<u32>;
#[doc = "Field `INT24_DIV` writer - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
pub type Int24DivW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
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
    #[doc = "Bits 8:31 - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    pub fn int24_div(&self) -> Int24DivR {
        Int24DivR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:7 - Fractional division by (FRAC5_DIV/32). Allows for fractional divisions in the range \\[0, 31/32\\]. Note that fractional division results in clock jitter as some clock periods may be 1 'clk_peri' cycle longer than other clock periods. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn frac5_div(&mut self) -> Frac5DivW<Div24_5CtlSpec> {
        Frac5DivW::new(self, 3)
    }
    #[doc = "Bits 8:31 - Integer division by (1+INT24_DIV). Allows for integer divisions in the range \\[1, 16,777,216\\]. Note: combined with fractional division, this divider type allows for a division in the range \\[1, 16,777,216 31/32\\]
in 1/32 increments. For the generation of a divided clock, the integer division range is restricted to \\[2, 16,777,216 31/32\\]. For the generation of a 50/50 percent duty cycle divided clock, the division range is restricted to \\[2, 16,777,216\\]. Note that this field is retained. However, the counter that is used to implement the division is not and will be initialized by HW to '0' when transitioning from DeepSleep to Active power mode."]
    #[inline(always)]
    #[must_use]
    pub fn int24_div(&mut self) -> Int24DivW<Div24_5CtlSpec> {
        Int24DivW::new(self, 8)
    }
}
#[doc = "Divider control (for 24.5 divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`div_24_5_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div_24_5_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Div24_5CtlSpec;
impl crate::RegisterSpec for Div24_5CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_24_5_ctl::R`](R) reader structure"]
impl crate::Readable for Div24_5CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`div_24_5_ctl::W`](W) writer structure"]
impl crate::Writable for Div24_5CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_24_5_CTL[%s]
to value 0"]
impl crate::Resettable for Div24_5CtlSpec {
    const RESET_VALUE: u32 = 0;
}
