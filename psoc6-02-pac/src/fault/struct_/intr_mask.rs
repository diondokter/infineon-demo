#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `FAULT` reader - Mask bit for corresponding field in the INTR register."]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - Mask bit for corresponding field in the INTR register."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<IntrMaskSpec> {
        FaultW::new(self, 0)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
