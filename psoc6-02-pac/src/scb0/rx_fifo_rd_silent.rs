#[doc = "Register `RX_FIFO_RD_SILENT` reader"]
pub type R = crate::R<RxFifoRdSilentSpec>;
#[doc = "Field `DATA` reader - Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
pub type DataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data read from the receiver FIFO. Reading a data frame will NOT remove the data frame from the FIFO; i.e. behavior is similar to that of a PEEK operation. Note that when CTRL.BYTE_MODE is '1', only DATA\\[7:0\\]
are used. A read from an empty RX FIFO sets INTR_RX.UNDERFLOW to '1'."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receiver FIFO read silent\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd_silent::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
