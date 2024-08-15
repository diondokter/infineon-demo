#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `TR_TX_REQ` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TrTxReqR = crate::BitReader;
#[doc = "Field `TR_TX_REQ` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TrTxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RX_REQ` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TrRxReqR = crate::BitReader;
#[doc = "Field `TR_RX_REQ` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TrRxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type XipAlignmentErrorR = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type XipAlignmentErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TxCmdFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TxCmdFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type TxDataFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type TxDataFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RxDataFifoUnderflowR = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RxDataFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TrTxReqR {
        TrTxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TrRxReqR {
        TrRxReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XipAlignmentErrorR {
        XipAlignmentErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TxCmdFifoOverflowR {
        TxCmdFifoOverflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TxDataFifoOverflowR {
        TxDataFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RxDataFifoUnderflowR {
        RxDataFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TrTxReqW<IntrMaskSpec> {
        TrTxReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TrRxReqW<IntrMaskSpec> {
        TrRxReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XipAlignmentErrorW<IntrMaskSpec> {
        XipAlignmentErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TxCmdFifoOverflowW<IntrMaskSpec> {
        TxCmdFifoOverflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TxDataFifoOverflowW<IntrMaskSpec> {
        TxDataFifoOverflowW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_fifo_underflow(&mut self) -> RxDataFifoUnderflowW<IntrMaskSpec> {
        RxDataFifoUnderflowW::new(self, 5)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
