#[doc = "Register `RX_DATA_FIFO_STATUS` reader"]
pub type R = crate::R<RxDataFifoStatusSpec>;
#[doc = "Field `USED4` reader - Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
pub type Used4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of entries that are used in the RX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn used4(&self) -> Used4R {
        Used4R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Receiver data FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxDataFifoStatusSpec;
impl crate::RegisterSpec for RxDataFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_fifo_status::R`](R) reader structure"]
impl crate::Readable for RxDataFifoStatusSpec {}
#[doc = "`reset()` method sets RX_DATA_FIFO_STATUS to value 0"]
impl crate::Resettable for RxDataFifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
