#[doc = "Register `CQDQS` reader"]
pub type R = crate::R<CqdqsSpec>;
#[doc = "Field `DQS` reader - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Device has marked task N as ready for execution - Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
pub type DqsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Device Queue Status Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Device has marked task N as ready for execution - Bit-N(0): Task-N is not ready for execution. This task could be pending in device or not submitted. Host controller updates this register with response of the Device Queue Status command."]
    #[inline(always)]
    pub fn dqs(&self) -> DqsR {
        DqsR::new(self.bits)
    }
}
#[doc = "Device queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqdqs::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqdqsSpec;
impl crate::RegisterSpec for CqdqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqdqs::R`](R) reader structure"]
impl crate::Readable for CqdqsSpec {}
#[doc = "`reset()` method sets CQDQS to value 0"]
impl crate::Resettable for CqdqsSpec {
    const RESET_VALUE: u32 = 0;
}
