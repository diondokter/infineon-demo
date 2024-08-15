#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `CNT_OVFLW` reader - Logical and of corresponding INTR and INTR_MASK fields."]
pub type CntOvflwR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn cnt_ovflw(&self) -> CntOvflwR {
        CntOvflwR::new(self.bits)
    }
}
#[doc = "Profile interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskedSpec;
impl crate::RegisterSpec for IntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for IntrMaskedSpec {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for IntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
