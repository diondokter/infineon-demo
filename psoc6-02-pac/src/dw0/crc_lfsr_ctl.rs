#[doc = "Register `CRC_LFSR_CTL` reader"]
pub type R = crate::R<CrcLfsrCtlSpec>;
#[doc = "Register `CRC_LFSR_CTL` writer"]
pub type W = crate::W<CrcLfsrCtlSpec>;
#[doc = "Field `LFSR32` reader - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
pub type Lfsr32R = crate::FieldReader<u32>;
#[doc = "Field `LFSR32` writer - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
pub type Lfsr32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    pub fn lfsr32(&self) -> Lfsr32R {
        Lfsr32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to implement CRC. This register needs to be initialized by SW to provide the CRC seed value. The seed value should be aligned such that the more significant bits (bit 31 and down) contain the seed value and the less significant bits (bit 0 and up) contain padding '0's. Note that SW can write this field. This functionality can be used prevent information leakage (through either CRC_LFSR_CTL or CRC_REM_RESULT)."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr32(&mut self) -> Lfsr32W<CrcLfsrCtlSpec> {
        Lfsr32W::new(self, 0)
    }
}
#[doc = "CRC LFSR control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_lfsr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_lfsr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcLfsrCtlSpec;
impl crate::RegisterSpec for CrcLfsrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_lfsr_ctl::R`](R) reader structure"]
impl crate::Readable for CrcLfsrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_lfsr_ctl::W`](W) writer structure"]
impl crate::Writable for CrcLfsrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_LFSR_CTL to value 0"]
impl crate::Resettable for CrcLfsrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
