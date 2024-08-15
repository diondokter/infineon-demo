#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `ALARM1` reader - Logical and of corresponding request and mask bits."]
pub type Alarm1R = crate::BitReader;
#[doc = "Field `ALARM2` reader - Logical and of corresponding request and mask bits."]
pub type Alarm2R = crate::BitReader;
#[doc = "Field `CENTURY` reader - Logical and of corresponding request and mask bits."]
pub type CenturyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn alarm1(&self) -> Alarm1R {
        Alarm1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn alarm2(&self) -> Alarm2R {
        Alarm2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn century(&self) -> CenturyR {
        CenturyR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
