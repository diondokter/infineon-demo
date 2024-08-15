#[doc = "Register `TX_DATA_FIFO_WR2` writer"]
pub type W = crate::W<TxDataFifoWr2Spec>;
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO, first byte)."]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - TX data (written to TX data FIFO, second byte)."]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<TxDataFifoWr2Spec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<TxDataFifoWr2Spec> {
        Data1W::new(self, 8)
    }
}
#[doc = "Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataFifoWr2Spec;
impl crate::RegisterSpec for TxDataFifoWr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_wr2::W`](W) writer structure"]
impl crate::Writable for TxDataFifoWr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR2 to value 0"]
impl crate::Resettable for TxDataFifoWr2Spec {
    const RESET_VALUE: u32 = 0;
}
