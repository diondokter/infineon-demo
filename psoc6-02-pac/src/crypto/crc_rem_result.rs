#[doc = "Register `CRC_REM_RESULT` reader"]
pub type R = crate::R<CrcRemResultSpec>;
#[doc = "Field `REM` reader - Remainder value. The alignment of the remainder depends on CRC_REM_CTL0.REM_REVERSE: '0': the more significant bits (bit 31 and down) contain the remainder. '1': the less significant bits (bit 0 and up) contain the remainder. Note: This field is combinatorially derived from CRC_LFSR_CTL.LFSR32, CRC_REM_CTL0.REM_REVERSE and CRC_REM_CTL1.REM_XOR."]
pub type RemR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Remainder value. The alignment of the remainder depends on CRC_REM_CTL0.REM_REVERSE: '0': the more significant bits (bit 31 and down) contain the remainder. '1': the less significant bits (bit 0 and up) contain the remainder. Note: This field is combinatorially derived from CRC_LFSR_CTL.LFSR32, CRC_REM_CTL0.REM_REVERSE and CRC_REM_CTL1.REM_XOR."]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new(self.bits)
    }
}
#[doc = "CRC remainder result\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_rem_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRemResultSpec;
impl crate::RegisterSpec for CrcRemResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_rem_result::R`](R) reader structure"]
impl crate::Readable for CrcRemResultSpec {}
#[doc = "`reset()` method sets CRC_REM_RESULT to value 0"]
impl crate::Resettable for CrcRemResultSpec {
    const RESET_VALUE: u32 = 0;
}
