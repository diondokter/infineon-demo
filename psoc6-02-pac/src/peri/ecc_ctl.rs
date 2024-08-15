#[doc = "Register `ECC_CTL` reader"]
pub type R = crate::R<EccCtlSpec>;
#[doc = "Register `ECC_CTL` writer"]
pub type W = crate::W<EccCtlSpec>;
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WordAddrR = crate::FieldReader<u16>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
pub type WordAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR."]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    pub fn word_addr(&self) -> WordAddrR {
        WordAddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Specifies the word address where the parity is injected. - On a 32-bit write access to this SRAM address and when ECC_INJ_EN bit is '1', the parity (PARITY) is injected."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WordAddrW<EccCtlSpec> {
        WordAddrW::new(self, 0)
    }
    #[doc = "Bit 16 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<EccCtlSpec> {
        EccEnW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable error injection for PERI protection structure SRAM. When '1', the parity (PARITY) is used when a write is done to the WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<EccCtlSpec> {
        EccInjEnW::new(self, 18)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<EccCtlSpec> {
        ParityW::new(self, 24)
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
#[doc = "`reset()` method sets ECC_CTL to value 0x0001_0000"]
impl crate::Resettable for EccCtlSpec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
