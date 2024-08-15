#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `CH` reader - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
pub type ChR = crate::BitReader;
#[doc = "Field `CH` writer - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to '1', when event (as specified by CH_STATUS.INTR_CAUSE) is detected. Write INTR.CH field with '1', to clear bit. Write INTR_SET.CH field with '1', to set bit."]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> ChW<IntrSpec> {
        ChW::new(self, 0)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
