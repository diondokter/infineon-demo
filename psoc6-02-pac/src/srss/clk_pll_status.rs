#[doc = "Register `CLK_PLL_STATUS[%s]` reader"]
pub type R = crate::R<ClkPllStatusSpec>;
#[doc = "Register `CLK_PLL_STATUS[%s]` writer"]
pub type W = crate::W<ClkPllStatusSpec>;
#[doc = "Field `LOCKED` reader - PLL Lock Indicator"]
pub type LockedR = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` reader - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UnlockOccurredR = crate::BitReader;
#[doc = "Field `UNLOCK_OCCURRED` writer - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub type UnlockOccurredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UnlockOccurredR {
        UnlockOccurredR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    #[must_use]
    pub fn unlock_occurred(&mut self) -> UnlockOccurredW<ClkPllStatusSpec> {
        UnlockOccurredW::new(self, 1)
    }
}
#[doc = "PLL Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pll_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pll_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPllStatusSpec;
impl crate::RegisterSpec for ClkPllStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pll_status::R`](R) reader structure"]
impl crate::Readable for ClkPllStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_pll_status::W`](W) writer structure"]
impl crate::Writable for ClkPllStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PLL_STATUS[%s]
to value 0"]
impl crate::Resettable for ClkPllStatusSpec {
    const RESET_VALUE: u32 = 0;
}
