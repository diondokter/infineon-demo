#[doc = "Register `TX_CMD_FIFO_STATUS` reader"]
pub type R = crate::R<TxCmdFifoStatusSpec>;
#[doc = "Field `USED3` reader - Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
pub type Used3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Number of entries that are used in the TX command FIFO (available in both XIP_MODE and MMIO_MODE). Legal range: \\[0, 4\\]."]
    #[inline(always)]
    pub fn used3(&self) -> Used3R {
        Used3R::new((self.bits & 7) as u8)
    }
}
#[doc = "Transmitter command FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cmd_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCmdFifoStatusSpec;
impl crate::RegisterSpec for TxCmdFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cmd_fifo_status::R`](R) reader structure"]
impl crate::Readable for TxCmdFifoStatusSpec {}
#[doc = "`reset()` method sets TX_CMD_FIFO_STATUS to value 0"]
impl crate::Resettable for TxCmdFifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
