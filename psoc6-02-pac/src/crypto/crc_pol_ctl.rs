#[doc = "Register `CRC_POL_CTL` reader"]
pub type R = crate::R<CrcPolCtlSpec>;
#[doc = "Register `CRC_POL_CTL` writer"]
pub type W = crate::W<CrcPolCtlSpec>;
#[doc = "Field `POLYNOMIAL` reader - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
pub type PolynomialR = crate::FieldReader<u32>;
#[doc = "Field `POLYNOMIAL` writer - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
pub type PolynomialW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    pub fn polynomial(&self) -> PolynomialR {
        PolynomialR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC polynomial. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned/shifted such that the more significant bits (bit 31 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's. Some frequently used polynomials: - CRC32: POLYNOMIAL is 0x04c11db7 (x^32 + x^26 + x^23 + x^22 + x^16 + x^12 + x^11 + x^10 + x^8 + x^7 + x^5 + x^4 + x^2 + x + 1). - CRC16: POLYNOMIAL is 0x80050000 (x^16 + x^15 + x^2 + 1, shifted by 16 bit positions). - CRC16 CCITT: POLYNOMIAL is 0x10210000 (x^16 + x^12 + x^5 + 1, shifted by 16 bit positions)."]
    #[inline(always)]
    #[must_use]
    pub fn polynomial(&mut self) -> PolynomialW<CrcPolCtlSpec> {
        PolynomialW::new(self, 0)
    }
}
#[doc = "CRC polynomial control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pol_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pol_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcPolCtlSpec;
impl crate::RegisterSpec for CrcPolCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pol_ctl::R`](R) reader structure"]
impl crate::Readable for CrcPolCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_pol_ctl::W`](W) writer structure"]
impl crate::Writable for CrcPolCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_POL_CTL to value 0"]
impl crate::Resettable for CrcPolCtlSpec {
    const RESET_VALUE: u32 = 0;
}
