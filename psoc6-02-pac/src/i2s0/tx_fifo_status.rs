#[doc = "Register `TX_FIFO_STATUS` reader"]
pub type R = crate::R<TxFifoStatusSpec>;
#[doc = "Field `USED` reader - Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
pub type UsedR = crate::FieldReader<u16>;
#[doc = "Field `RD_PTR` reader - TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
pub type RdPtrR = crate::FieldReader;
#[doc = "Field `WR_PTR` reader - TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
pub type WrPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Number of entries in the TX FIFO. The field value is in the range \\[0, 256\\]."]
    #[inline(always)]
    pub fn used(&self) -> UsedR {
        UsedR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - TX FIFO read pointer: FIFO location from which a data frame is read by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RdPtrR {
        RdPtrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TX FIFO write pointer: FIFO location at which a new data frame is written by the host. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WrPtrR {
        WrPtrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "TX FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoStatusSpec;
impl crate::RegisterSpec for TxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_status::R`](R) reader structure"]
impl crate::Readable for TxFifoStatusSpec {}
#[doc = "`reset()` method sets TX_FIFO_STATUS to value 0"]
impl crate::Resettable for TxFifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
