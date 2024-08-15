#[doc = "Register `CRC_REM_CTL` reader"]
pub type R = crate::R<CrcRemCtlSpec>;
#[doc = "Register `CRC_REM_CTL` writer"]
pub type W = crate::W<CrcRemCtlSpec>;
#[doc = "Field `REM_XOR` reader - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
pub type RemXorR = crate::FieldReader<u32>;
#[doc = "Field `REM_XOR` writer - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
pub type RemXorW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    pub fn rem_xor(&self) -> RemXorR {
        RemXorR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies a mask with which the CRC_LFSR_CTL.LFSR32 register is XOR'd to produce a remainder. The XOR is performed before remainder reversal."]
    #[inline(always)]
    #[must_use]
    pub fn rem_xor(&mut self) -> RemXorW<CrcRemCtlSpec> {
        RemXorW::new(self, 0)
    }
}
#[doc = "CRC remainder control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_rem_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_rem_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcRemCtlSpec;
impl crate::RegisterSpec for CrcRemCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_rem_ctl::R`](R) reader structure"]
impl crate::Readable for CrcRemCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_rem_ctl::W`](W) writer structure"]
impl crate::Writable for CrcRemCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_REM_CTL to value 0"]
impl crate::Resettable for CrcRemCtlSpec {
    const RESET_VALUE: u32 = 0;
}
