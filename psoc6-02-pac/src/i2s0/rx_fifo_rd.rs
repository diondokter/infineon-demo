#[doc = "Register `RX_FIFO_RD` reader"]
pub type R = crate::R<RxFifoRdSpec>;
#[doc = "Field `DATA` reader - Data read from the RX FIFO. Reading a data frame will remove the data frame from the RX FIFO; i.e. behavior is similar to that of a POP operation. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data read from the RX FIFO. Reading a data frame will remove the data frame from the RX FIFO; i.e. behavior is similar to that of a POP operation. Notes: - Don't access to this register while RX_FIFO_CTL.CLEAR is '1'. - Two stored data may be not valid after CMD.RX_START is set '1'. Therefore we recommend software discard those data."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "RX FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoRdSpec;
impl crate::RegisterSpec for RxFifoRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_rd::R`](R) reader structure"]
impl crate::Readable for RxFifoRdSpec {}
#[doc = "`reset()` method sets RX_FIFO_RD to value 0"]
impl crate::Resettable for RxFifoRdSpec {
    const RESET_VALUE: u32 = 0;
}
