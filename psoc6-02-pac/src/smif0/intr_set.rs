#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `TR_TX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TrTxReqR = crate::BitReader;
#[doc = "Field `TR_TX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TrTxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RX_REQ` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TrRxReqR = crate::BitReader;
#[doc = "Field `TR_RX_REQ` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TrRxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type XipAlignmentErrorR = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type XipAlignmentErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxCmdFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxCmdFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxDataFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type TxDataFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxDataFifoUnderflowR = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RxDataFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TrTxReqR {
        TrTxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TrRxReqR {
        TrRxReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XipAlignmentErrorR {
        XipAlignmentErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TxCmdFifoOverflowR {
        TxCmdFifoOverflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TxDataFifoOverflowR {
        TxDataFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RxDataFifoUnderflowR {
        RxDataFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TrTxReqW<IntrSetSpec> {
        TrTxReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TrRxReqW<IntrSetSpec> {
        TrRxReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XipAlignmentErrorW<IntrSetSpec> {
        XipAlignmentErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TxCmdFifoOverflowW<IntrSetSpec> {
        TxCmdFifoOverflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TxDataFifoOverflowW<IntrSetSpec> {
        TxDataFifoOverflowW::new(self, 4)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_fifo_underflow(&mut self) -> RxDataFifoUnderflowW<IntrSetSpec> {
        RxDataFifoUnderflowW::new(self, 5)
    }
}
#[doc = "Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
