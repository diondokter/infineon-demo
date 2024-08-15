#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `RELEASE` reader - Logical and of corresponding request and mask bits."]
pub type ReleaseR = crate::FieldReader<u16>;
#[doc = "Field `NOTIFY` reader - Logical and of corresponding INTR and INTR_MASK fields."]
pub type NotifyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn release(&self) -> ReleaseR {
        ReleaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Logical and of corresponding INTR and INTR_MASK fields."]
    #[inline(always)]
    pub fn notify(&self) -> NotifyR {
        NotifyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
