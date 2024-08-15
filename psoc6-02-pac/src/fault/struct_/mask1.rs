#[doc = "Register `MASK1` reader"]
pub type R = crate::R<Mask1Spec>;
#[doc = "Register `MASK1` writer"]
pub type W = crate::W<Mask1Spec>;
#[doc = "Field `SOURCE` reader - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SourceR = crate::FieldReader<u32>;
#[doc = "Field `SOURCE` writer - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Fault source enables: Bits 31-0: Fault sources 63 to 32."]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<Mask1Spec> {
        SourceW::new(self, 0)
    }
}
#[doc = "Fault mask 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mask1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mask1Spec;
impl crate::RegisterSpec for Mask1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask1::R`](R) reader structure"]
impl crate::Readable for Mask1Spec {}
#[doc = "`write(|w| ..)` method takes [`mask1::W`](W) writer structure"]
impl crate::Writable for Mask1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK1 to value 0"]
impl crate::Resettable for Mask1Spec {
    const RESET_VALUE: u32 = 0;
}
