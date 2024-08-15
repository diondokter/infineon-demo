#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `TR_TX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TrTxReqR = crate::BitReader;
#[doc = "Field `TR_RX_REQ` reader - Logical and of corresponding request and mask bits."]
pub type TrRxReqR = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type XipAlignmentErrorR = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TxCmdFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type TxDataFifoOverflowR = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Logical and of corresponding request and mask bits."]
pub type RxDataFifoUnderflowR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TrTxReqR {
        TrTxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TrRxReqR {
        TrRxReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XipAlignmentErrorR {
        XipAlignmentErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TxCmdFifoOverflowR {
        TxCmdFifoOverflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TxDataFifoOverflowR {
        TxDataFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RxDataFifoUnderflowR {
        RxDataFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
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
