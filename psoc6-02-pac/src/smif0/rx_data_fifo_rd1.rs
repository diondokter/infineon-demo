#[doc = "Register `RX_DATA_FIFO_RD1` reader"]
pub type R = crate::R<RxDataFifoRd1Spec>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO)."]
pub type Data0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data (read from RX data FIFO)."]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataFifoRd1Spec;
impl crate::RegisterSpec for RxDataFifoRd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd1::R`](R) reader structure"]
impl crate::Readable for RxDataFifoRd1Spec {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD1 to value 0"]
impl crate::Resettable for RxDataFifoRd1Spec {
    const RESET_VALUE: u32 = 0;
}
