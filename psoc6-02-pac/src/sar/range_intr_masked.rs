#[doc = "Register `RANGE_INTR_MASKED` reader"]
pub type R = crate::R<RangeIntrMaskedSpec>;
#[doc = "Field `RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type RangeMaskedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn range_masked(&self) -> RangeMaskedR {
        RangeMaskedR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Range interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeIntrMaskedSpec;
impl crate::RegisterSpec for RangeIntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr_masked::R`](R) reader structure"]
impl crate::Readable for RangeIntrMaskedSpec {}
#[doc = "`reset()` method sets RANGE_INTR_MASKED to value 0"]
impl crate::Resettable for RangeIntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
