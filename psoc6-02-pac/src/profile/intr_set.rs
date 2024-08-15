#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `CNT_OVFLW` reader - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
pub type CntOvflwR = crate::FieldReader<u32>;
#[doc = "Field `CNT_OVFLW` writer - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
pub type CntOvflwW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CntOvflwR {
        CntOvflwR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SW writes a '1' to a bit of this field to set the corresponding bit in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_ovflw(&mut self) -> CntOvflwW<IntrSetSpec> {
        CntOvflwW::new(self, 0)
    }
}
#[doc = "Profile interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
