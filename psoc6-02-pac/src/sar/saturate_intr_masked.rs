#[doc = "Register `SATURATE_INTR_MASKED` reader"]
pub type R = crate::R<SaturateIntrMaskedSpec>;
#[doc = "Field `SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type SaturateMaskedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn saturate_masked(&self) -> SaturateMaskedR {
        SaturateMaskedR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Saturate interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaturateIntrMaskedSpec;
impl crate::RegisterSpec for SaturateIntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr_masked::R`](R) reader structure"]
impl crate::Readable for SaturateIntrMaskedSpec {}
#[doc = "`reset()` method sets SATURATE_INTR_MASKED to value 0"]
impl crate::Resettable for SaturateIntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
