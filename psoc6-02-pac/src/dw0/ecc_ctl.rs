#[doc = "Register `ECC_CTL` reader"]
pub type R = crate::R<EccCtlSpec>;
#[doc = "Register `ECC_CTL` writer"]
pub type W = crate::W<EccCtlSpec>;
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WordAddrR = crate::FieldReader<u16>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WordAddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WordAddrR {
        WordAddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Specifies the word address where an error will be injected. - On a write transfer to this SRAM word address and when CTL.ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WordAddrW<EccCtlSpec> {
        WordAddrW::new(self, 0)
    }
    #[doc = "Bits 25:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<EccCtlSpec> {
        ParityW::new(self, 25)
    }
}
#[doc = "ECC control\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtlSpec;
impl crate::RegisterSpec for EccCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctl::R`](R) reader structure"]
impl crate::Readable for EccCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctl::W`](W) writer structure"]
impl crate::Writable for EccCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECC_CTL to value 0"]
impl crate::Resettable for EccCtlSpec {
    const RESET_VALUE: u32 = 0;
}
