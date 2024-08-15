#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `SAMPLE` reader - Logical and of corresponding request and mask bits."]
pub type SampleR = crate::BitReader;
#[doc = "Field `INIT` reader - Logical and of corresponding request and mask bits."]
pub type InitR = crate::BitReader;
#[doc = "Field `ADC_RES` reader - Logical and of corresponding request and mask bits."]
pub type AdcResR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn adc_res(&self) -> AdcResR {
        AdcResR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "CSD Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
