#[doc = "Register `RX_FIFO_STATUS` reader"]
pub type R = crate::R<RxFifoStatusSpec>;
#[doc = "Field `USED` reader - Number of entries in the RX FIFO. The field value is in the range \\[0, 255\\]. When this is zero, the RX FIFO is empty."]
pub type UsedR = crate::FieldReader;
#[doc = "Field `RD_PTR` reader - RX FIFO read pointer: RX FIFO location from which a data frame is read by the host.This field is used only for debugging purposes."]
pub type RdPtrR = crate::FieldReader;
#[doc = "Field `WR_PTR` reader - RX FIFO write pointer: RX FIFO location at which a new data frame is written by the hardware.This field is used only for debugging purposes."]
pub type WrPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of entries in the RX FIFO. The field value is in the range \\[0, 255\\]. When this is zero, the RX FIFO is empty."]
    #[inline(always)]
    pub fn used(&self) -> UsedR {
        UsedR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX FIFO read pointer: RX FIFO location from which a data frame is read by the host.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RdPtrR {
        RdPtrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - RX FIFO write pointer: RX FIFO location at which a new data frame is written by the hardware.This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WrPtrR {
        WrPtrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoStatusSpec;
impl crate::RegisterSpec for RxFifoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_status::R`](R) reader structure"]
impl crate::Readable for RxFifoStatusSpec {}
#[doc = "`reset()` method sets RX_FIFO_STATUS to value 0"]
impl crate::Resettable for RxFifoStatusSpec {
    const RESET_VALUE: u32 = 0;
}
