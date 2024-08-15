#[doc = "Register `CRC_DATA_CTL` reader"]
pub type R = crate::R<CrcDataCtlSpec>;
#[doc = "Register `CRC_DATA_CTL` writer"]
pub type W = crate::W<CrcDataCtlSpec>;
#[doc = "Field `DATA_XOR` reader - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DataXorR = crate::FieldReader;
#[doc = "Field `DATA_XOR` writer - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
pub type DataXorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    pub fn data_xor(&self) -> DataXorR {
        DataXorR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies a byte mask with which each data byte is XOR'd. The XOR is performed before data reversal."]
    #[inline(always)]
    #[must_use]
    pub fn data_xor(&mut self) -> DataXorW<CrcDataCtlSpec> {
        DataXorW::new(self, 0)
    }
}
#[doc = "CRC data control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_data_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcDataCtlSpec;
impl crate::RegisterSpec for CrcDataCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_data_ctl::R`](R) reader structure"]
impl crate::Readable for CrcDataCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_data_ctl::W`](W) writer structure"]
impl crate::Writable for CrcDataCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_DATA_CTL to value 0"]
impl crate::Resettable for CrcDataCtlSpec {
    const RESET_VALUE: u32 = 0;
}
