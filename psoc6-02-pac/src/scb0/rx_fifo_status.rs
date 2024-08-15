#[doc = "Register `RX_FIFO_STATUS` reader"]
pub type R = crate::R<RxFifoStatusSpec>;
#[doc = "Field `USED` reader - Amount of entries in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
pub type UsedR = crate::FieldReader<u16>;
#[doc = "Field `SR_VALID` reader - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
pub type SrValidR = crate::BitReader;
#[doc = "Field `RD_PTR` reader - FIFO read pointer: FIFO location from which a data frame is read."]
pub type RdPtrR = crate::FieldReader;
#[doc = "Field `WR_PTR` reader - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
pub type WrPtrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:8 - Amount of entries in the receiver FIFO. The value of this field ranges from 0 to FF_DATA_NR (EZ_DATA_NR/2)."]
    #[inline(always)]
    pub fn used(&self) -> UsedR {
        UsedR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Indicates whether the RX shift registers holds a (partial) valid data frame ('1') or not ('0'). The shift register can be considered the bottom of the RX FIFO (the data frame is not included in the USED field of the RX FIFO). The shift register is a working register and holds the data frame that is currently being received (when the protocol state machine is receiving a data frame)."]
    #[inline(always)]
    pub fn sr_valid(&self) -> SrValidR {
        SrValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - FIFO read pointer: FIFO location from which a data frame is read."]
    #[inline(always)]
    pub fn rd_ptr(&self) -> RdPtrR {
        RdPtrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIFO write pointer: FIFO location at which a new data frame is written by the hardware."]
    #[inline(always)]
    pub fn wr_ptr(&self) -> WrPtrR {
        WrPtrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Receiver FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
