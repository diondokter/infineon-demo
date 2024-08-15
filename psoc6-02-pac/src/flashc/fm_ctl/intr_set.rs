#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `TIMER_EXPIRED` reader - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TimerExpiredR = crate::BitReader;
#[doc = "Field `TIMER_EXPIRED` writer - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
pub type TimerExpiredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn timer_expired(&self) -> TimerExpiredR {
        TimerExpiredR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write INTR_SET field with '1' to set corresponding INTR field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn timer_expired(&mut self) -> TimerExpiredW<IntrSetSpec> {
        TimerExpiredW::new(self, 0)
    }
}
#[doc = "Interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
