#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `CNT_OVFLW` reader - Mask bit for corresponding field in the INTR register."]
pub type CntOvflwR = crate::FieldReader<u32>;
#[doc = "Field `CNT_OVFLW` writer - Mask bit for corresponding field in the INTR register."]
pub type CntOvflwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CntOvflwR {
        CntOvflwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask bit for corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovflw(&mut self) -> CntOvflwW<IntrMaskSpec> {
        CntOvflwW::new(self, 0)
    }
}
#[doc = "Profile interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
