#[doc = "Register `RX_DATA_FIFO_RD2` reader"]
pub type R = crate::R<RxDataFifoRd2Spec>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO, first byte)."]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA1` reader - RX data (read from RX data FIFO, second byte)."]
pub type Data1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO, first byte)."]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX data (read from RX data FIFO, second byte)."]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataFifoRd2Spec;
impl crate::RegisterSpec for RxDataFifoRd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd2::R`](R) reader structure"]
impl crate::Readable for RxDataFifoRd2Spec {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD2 to value 0"]
impl crate::Resettable for RxDataFifoRd2Spec {
    const RESET_VALUE: u32 = 0;
}
