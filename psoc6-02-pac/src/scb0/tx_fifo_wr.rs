#[doc = "Register `TX_FIFO_WR` writer"]
pub type W = crate::W<TxFifoWrSpec>;
#[doc = "Field `DATA` writer - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data frame written into the transmitter FIFO. Behavior is similar to that of a PUSH operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A write to a full TX FIFO sets INTR_TX.OVERFLOW to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<TxFifoWrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Transmitter FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoWrSpec;
impl crate::RegisterSpec for TxFifoWrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_wr::W`](W) writer structure"]
impl crate::Writable for TxFifoWrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_FIFO_WR to value 0"]
impl crate::Resettable for TxFifoWrSpec {
    const RESET_VALUE: u32 = 0;
}
