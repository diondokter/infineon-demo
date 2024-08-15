#[doc = "Register `MASK2` reader"]
pub type R = crate::R<Mask2Spec>;
#[doc = "Register `MASK2` writer"]
pub type W = crate::W<Mask2Spec>;
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SourceR = crate::FieldReader<u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 95 to 64."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<Mask2Spec> {
        SourceW::new(self, 0)
    }
}
#[doc = "Fault mask 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mask2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mask2Spec;
impl crate::RegisterSpec for Mask2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask2::R`](R) reader structure"]
impl crate::Readable for Mask2Spec {}
#[doc = "`write(|w| ..)` method takes [`mask2::W`](W) writer structure"]
impl crate::Writable for Mask2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK2 to value 0"]
impl crate::Resettable for Mask2Spec {
    const RESET_VALUE: u32 = 0;
}
