#[doc = "Register `INTR_RX_MASKED` reader"]
pub type R = crate::R<IntrRxMaskedSpec>;
#[doc = "Field `TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `NOT_EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type NotEmptyR = crate::BitReader;
#[doc = "Field `FULL` reader - Logical and of corresponding request and mask bits."]
pub type FullR = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `BLOCKED` reader - Logical and of corresponding request and mask bits."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `FRAME_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type FrameErrorR = crate::BitReader;
#[doc = "Field `PARITY_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type ParityErrorR = crate::BitReader;
#[doc = "Field `BAUD_DETECT` reader - Logical and of corresponding request and mask bits."]
pub type BaudDetectR = crate::BitReader;
#[doc = "Field `BREAK_DETECT` reader - Logical and of corresponding request and mask bits."]
pub type BreakDetectR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_empty(&self) -> NotEmptyR {
        NotEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn frame_error(&self) -> FrameErrorR {
        FrameErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn parity_error(&self) -> ParityErrorR {
        ParityErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BaudDetectR {
        BaudDetectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn break_detect(&self) -> BreakDetectR {
        BreakDetectR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Receiver interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRxMaskedSpec;
impl crate::RegisterSpec for IntrRxMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx_masked::R`](R) reader structure"]
impl crate::Readable for IntrRxMaskedSpec {}
#[doc = "`reset()` method sets INTR_RX_MASKED to value 0"]
impl crate::Resettable for IntrRxMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
