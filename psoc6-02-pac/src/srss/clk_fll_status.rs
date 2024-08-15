#[doc = "Register `CLK_FLL_STATUS` reader"]
pub type R = crate::R<ClkFllStatusSpec>;
#[doc = "Register `CLK_FLL_STATUS` writer"]
pub type W = crate::W<ClkFllStatusSpec>;
#[doc = "Field `LOCKED` reader - FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
pub type LockedR = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` reader - N/A"]
pub type UnlockOccurredR = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` writer - N/A"]
pub type UnlockOccurredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCO_READY` reader - This indicates that the CCO is internally settled and ready to use."]
pub type CcoReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FLL Lock Indicator. LOCKED is high when FLL is within CLK_FLL_CONFIG2.LOCK_TOL. If FLL is outside LOCK_TOL, LOCKED goes low. Note that this can happen during normal operation, if FLL needs to recalculate due to a change in the reference clock, change in voltage, or change in temperature."]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UnlockOccurredR {
        UnlockOccurredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This indicates that the CCO is internally settled and ready to use."]
    #[inline(always)]
    pub fn cco_ready(&self) -> CcoReadyR {
        CcoReadyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn unlock_occurred(&mut self) -> UnlockOccurredW<ClkFllStatusSpec> {
        UnlockOccurredW::new(self, 1)
    }
}
#[doc = "FLL Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_fll_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_fll_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkFllStatusSpec;
impl crate::RegisterSpec for ClkFllStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_fll_status::R`](R) reader structure"]
impl crate::Readable for ClkFllStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_fll_status::W`](W) writer structure"]
impl crate::Writable for ClkFllStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_FLL_STATUS to value 0"]
impl crate::Resettable for ClkFllStatusSpec {
    const RESET_VALUE: u32 = 0;
}
