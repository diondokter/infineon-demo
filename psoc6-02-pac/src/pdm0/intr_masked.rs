#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `RX_TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type RxTriggerR = crate::BitReader;
#[doc = "Field `RX_NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type RxNotEmptyR = crate::BitReader;
#[doc = "Field `RX_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RxOverflowR = crate::BitReader;
#[doc = "Field `RX_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RxUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 16 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_trigger(&self) -> RxTriggerR {
        RxTriggerR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_not_empty(&self) -> RxNotEmptyR {
        RxNotEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_overflow(&self) -> RxOverflowR {
        RxOverflowR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_underflow(&self) -> RxUnderflowR {
        RxUnderflowR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
