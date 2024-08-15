#[doc = "Register `TX_DATA_FIFO_WR4` writer"]
pub type W = crate::W<TxDataFifoWr4Spec>;
#[doc = "Field `DATA0` writer - TX data (written to TX data FIFO, first byte)."]
pub type Data0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` writer - TX data (written to TX data FIFO, second byte)."]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` writer - TX data (written to TX data FIFO, third byte)."]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` writer - TX data (written to TX data FIFO, fourth byte)."]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data (written to TX data FIFO, first byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> Data0W<TxDataFifoWr4Spec> {
        Data0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX data (written to TX data FIFO, second byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<TxDataFifoWr4Spec> {
        Data1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - TX data (written to TX data FIFO, third byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> Data2W<TxDataFifoWr4Spec> {
        Data2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - TX data (written to TX data FIFO, fourth byte)."]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> Data3W<TxDataFifoWr4Spec> {
        Data3W::new(self, 24)
    }
}
#[doc = "Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr4::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataFifoWr4Spec;
impl crate::RegisterSpec for TxDataFifoWr4Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_wr4::W`](W) writer structure"]
impl crate::Writable for TxDataFifoWr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_WR4 to value 0"]
impl crate::Resettable for TxDataFifoWr4Spec {
    const RESET_VALUE: u32 = 0;
}
