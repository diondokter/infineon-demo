#[doc = "Register `CLK_FLL_CONFIG2` reader"]
pub type R = crate::R<ClkFllConfig2Spec>;
#[doc = "Register `CLK_FLL_CONFIG2` writer"]
pub type W = crate::W<ClkFllConfig2Spec>;
#[doc = "Field `FLL_REF_DIV` reader - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
pub type FllRefDivR = crate::FieldReader<u16>;
#[doc = "Field `FLL_REF_DIV` writer - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
pub type FllRefDivW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LOCK_TOL` reader - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
pub type LockTolR = crate::FieldReader<u16>;
#[doc = "Field `LOCK_TOL` writer - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
pub type LockTolW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    pub fn fll_ref_div(&self) -> FllRefDivR {
        FllRefDivR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    pub fn lock_tol(&self) -> LockTolR {
        LockTolR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Control bits for reference divider. Set the divide value before enabling the FLL, and do not change it while FLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 8191: divide by 8191"]
    #[inline(always)]
    #[must_use]
    pub fn fll_ref_div(&mut self) -> FllRefDivW<ClkFllConfig2Spec> {
        FllRefDivW::new(self, 0)
    }
    #[doc = "Bits 16:24 - Lock tolerance sets the error threshold for when the FLL output is considered locked to the reference input. A high tolerance can be used to lock more quickly or to track a less accurate source. The tolerance should be set so that the FLL does not unlock under normal conditions. The tolerance is the allowed difference between the count value for the ideal formula and the measured value. 0: tolerate error of 1 count value 1: tolerate error of 2 count values ... 511: tolerate error of 512 count values"]
    #[inline(always)]
    #[must_use]
    pub fn lock_tol(&mut self) -> LockTolW<ClkFllConfig2Spec> {
        LockTolW::new(self, 16)
    }
}
#[doc = "FLL Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkFllConfig2Spec;
impl crate::RegisterSpec for ClkFllConfig2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_config2::R`](R) reader structure"]
impl crate::Readable for ClkFllConfig2Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_config2::W`](W) writer structure"]
impl crate::Writable for ClkFllConfig2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_FLL_CONFIG2 to value 0x0002_0001"]
impl crate::Resettable for ClkFllConfig2Spec {
    const RESET_VALUE: u32 = 0x0002_0001;
}
