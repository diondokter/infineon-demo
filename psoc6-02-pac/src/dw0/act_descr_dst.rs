#[doc = "Register `ACT_DESCR_DST` reader"]
pub type R = crate::R<ActDescrDstSpec>;
#[doc = "Field `DATA` reader - Copy of DESCR_DST of the currently active descriptor. Base address of destination location. Note: For a CRC transfer descriptor, this field should be programmed with the address of the CRC_LFSR_CTL register. The calculated CRC LFSR state is written to this address (through the CRYPTO AHB-Lite master interface) when the input trigger is processed. The write transfer will be submitted to the CPUSS and PERI protection schemes."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_DST of the currently active descriptor. Base address of destination location. Note: For a CRC transfer descriptor, this field should be programmed with the address of the CRC_LFSR_CTL register. The calculated CRC LFSR state is written to this address (through the CRYPTO AHB-Lite master interface) when the input trigger is processed. The write transfer will be submitted to the CPUSS and PERI protection schemes."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Active descriptor destination\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrDstSpec;
impl crate::RegisterSpec for ActDescrDstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_dst::R`](R) reader structure"]
impl crate::Readable for ActDescrDstSpec {}
#[doc = "`reset()` method sets ACT_DESCR_DST to value 0"]
impl crate::Resettable for ActDescrDstSpec {
    const RESET_VALUE: u32 = 0;
}
