#[doc = "Register `PENDING0` reader"]
pub type R = crate::R<Pending0Spec>;
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. Bit 4: DMA controller MPU. ... Bit 15: DAP MPU. Bit 16: CM4 system bus MPU. Bit 17: CM4 code bus MPU (for non FLASH controller accesses). Bit 18: CM4 code bus MPU (for FLASH controller accesses)."]
pub type SourceR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0: CM0 MPU. Bit 1: CRYPTO MPU. Bit 2: DW 0 MPU. Bit 3: DW 1 MPU. Bit 4: DMA controller MPU. ... Bit 15: DAP MPU. Bit 16: CM4 system bus MPU. Bit 17: CM4 code bus MPU (for non FLASH controller accesses). Bit 18: CM4 code bus MPU (for FLASH controller accesses)."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits)
    }
}
#[doc = "Fault pending 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pending0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending0Spec;
impl crate::RegisterSpec for Pending0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending0::R`](R) reader structure"]
impl crate::Readable for Pending0Spec {}
#[doc = "`reset()` method sets PENDING0 to value 0"]
impl crate::Resettable for Pending0Spec {
    const RESET_VALUE: u32 = 0;
}
