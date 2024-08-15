#[doc = "Register `TX_DATA_FIFO_STATUS` reader"]
pub type R = crate::R<TxDataFifoStatusSpec>;
#[doc = "Field `USED4` reader - Number of entries that are used in the TX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
pub type Used4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Number of entries that are used in the TX data FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 8\\]."]
    #[inline(always)]
    pub fn used4(&self) -> Used4R {
        Used4R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Transmitter data FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataFifoStatusSpec;
impl crate::RegisterSpec for TxDataFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_fifo_status::R`](R) reader structure"]
impl crate::Readable for TxDataFifoStatusSpec {}
#[doc = "`reset()` method sets TX_DATA_FIFO_STATUS to value 0"]
impl crate::Resettable for TxDataFifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
