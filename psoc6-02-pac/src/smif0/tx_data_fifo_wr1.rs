#[doc = "Register `TX_DATA_FIFO_WR1` writer"]
pub type W = crate::W<TxDataFifoWr1Spec>;
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO)."]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<TxDataFifoWr1Spec> {
        Data0W::new(self, 0)
    }
}
#[doc = "Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataFifoWr1Spec;
impl crate::RegisterSpec for TxDataFifoWr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_wr1::W`](W) writer structure"]
impl crate::Writable for TxDataFifoWr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR1 to value 0"]
impl crate::Resettable for TxDataFifoWr1Spec {
    const RESET_VALUE: u32 = 0;
}
