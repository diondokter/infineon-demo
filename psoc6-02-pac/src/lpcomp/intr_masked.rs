#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `COMP0_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type Comp0MaskedR = crate::BitReader;
#[doc = "Field `COMP1_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type Comp1MaskedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp0_masked(&self) -> Comp0MaskedR {
        Comp0MaskedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn comp1_masked(&self) -> Comp1MaskedR {
        Comp1MaskedR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "LPCOMP Interrupt request masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
