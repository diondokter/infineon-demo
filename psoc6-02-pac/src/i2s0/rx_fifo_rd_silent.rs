#[doc = "Register `RX_FIFO_RD_SILENT` reader"]
pub type R = crate::R<RxFifoRdSilentSpec>;
#[doc = "Field `DATA` reader - Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the RX FIFO. Reading a data frame will NOT remove the data frame from the RX FIFO; i.e. behavior is similar to that of a PEEK operation. This field is used only for debugging purposes. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "RX FIFO silent read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd_silent::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoRdSilentSpec;
impl crate::RegisterSpec for RxFifoRdSilentSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_rd_silent::R`](R) reader structure"]
impl crate::Readable for RxFifoRdSilentSpec {}
#[doc = "`reset()` method sets RX_FIFO_RD_SILENT to value 0"]
impl crate::Resettable for RxFifoRdSilentSpec {
    const RESET_VALUE: u32 = 0;
}
