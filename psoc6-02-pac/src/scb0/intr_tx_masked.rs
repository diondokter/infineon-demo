#[doc = "Register `INTR_TX_MASKED` reader"]
pub type R = crate::R<IntrTxMaskedSpec>;
#[doc = "Field `TRIGGER` reader - Logical and of corresponding request and mask bits."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `NOT_FULL` reader - Logical and of corresponding request and mask bits."]
pub type NotFullR = crate::BitReader;
#[doc = "Field `EMPTY` reader - Logical and of corresponding request and mask bits."]
pub type EmptyR = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `BLOCKED` reader - Logical and of corresponding request and mask bits."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `UART_NACK` reader - Logical and of corresponding request and mask bits."]
pub type UartNackR = crate::BitReader;
#[doc = "Field `UART_DONE` reader - Logical and of corresponding request and mask bits."]
pub type UartDoneR = crate::BitReader;
#[doc = "Field `UART_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type UartArbLostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn not_full(&self) -> NotFullR {
        NotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn uart_nack(&self) -> UartNackR {
        UartNackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_done(&self) -> UartDoneR {
        UartDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UartArbLostR {
        UartArbLostR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Transmitter interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrTxMaskedSpec;
impl crate::RegisterSpec for IntrTxMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_tx_masked::R`](R) reader structure"]
impl crate::Readable for IntrTxMaskedSpec {}
#[doc = "`reset()` method sets INTR_TX_MASKED to value 0"]
impl crate::Resettable for IntrTxMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
