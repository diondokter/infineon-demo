#[doc = "Register `RX_DATA_FIFO_RD4` reader"]
pub type R = crate::R<RxDataFifoRd4Spec>;
#[doc = "Field `DATA0` reader - RX data (read from RX data FIFO, first byte)."]
pub type Data0R = crate::FieldReader;
#[doc = "Field `DATA1` reader - RX data (read from RX data FIFO, second byte)."]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA2` reader - RX data (read from RX data FIFO, third byte)."]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA3` reader - RX data (read from RX data FIFO, fourth byte)."]
pub type Data3R = crate::FieldReader;
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
    #[doc = "Bits 16:23 - RX data (read from RX data FIFO, third byte)."]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX data (read from RX data FIFO, fourth byte)."]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataFifoRd4Spec;
impl crate::RegisterSpec for RxDataFifoRd4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_rd4::R`](R) reader structure"]
impl crate::Readable for RxDataFifoRd4Spec {}
#[doc = "`reset()` method sets RX_DATA_FIFO_RD4 to value 0"]
impl crate::Resettable for RxDataFifoRd4Spec {
    const RESET_VALUE: u32 = 0;
}
