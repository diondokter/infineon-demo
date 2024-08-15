#[doc = "Register `CRC_CTL` reader"]
pub type R = crate::R<CrcCtlSpec>;
#[doc = "Register `CRC_CTL` writer"]
pub type W = crate::W<CrcCtlSpec>;
#[doc = "Field `DATA_REVERSE` reader - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DataReverseR = crate::BitReader;
#[doc = "Field `DATA_REVERSE` writer - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
pub type DataReverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REM_REVERSE` reader - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type RemReverseR = crate::BitReader;
#[doc = "Field `REM_REVERSE` writer - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
pub type RemReverseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    pub fn data_reverse(&self) -> DataReverseR {
        DataReverseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    pub fn rem_reverse(&self) -> RemReverseR {
        RemReverseR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies the bit order in which a data Byte is processed (reversal is performed after XORing): '0': Most significant bit (bit 1) first. '1': Least significant bit (bit 0) first."]
    #[inline(always)]
    #[must_use]
    pub fn data_reverse(&mut self) -> DataReverseW<CrcCtlSpec> {
        DataReverseW::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies whether the remainder is bit reversed (reversal is performed after XORing): '0': No. '1': Yes."]
    #[inline(always)]
    #[must_use]
    pub fn rem_reverse(&mut self) -> RemReverseW<CrcCtlSpec> {
        RemReverseW::new(self, 8)
    }
}
#[doc = "CRC control\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcCtlSpec;
impl crate::RegisterSpec for CrcCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_ctl::R`](R) reader structure"]
impl crate::Readable for CrcCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_ctl::W`](W) writer structure"]
impl crate::Writable for CrcCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_CTL to value 0"]
impl crate::Resettable for CrcCtlSpec {
    const RESET_VALUE: u32 = 0;
}
