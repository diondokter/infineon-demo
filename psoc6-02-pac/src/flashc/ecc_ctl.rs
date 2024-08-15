#[doc = "Register `ECC_CTL` reader"]
pub type R = crate::R<EccCtlSpec>;
#[doc = "Register `ECC_CTL` writer"]
pub type W = crate::W<EccCtlSpec>;
#[doc = "Field `WORD_ADDR` reader - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
pub type WordAddrR = crate::FieldReader<u32>;
#[doc = "Field `WORD_ADDR` writer - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
pub type WordAddrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PARITY` reader - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    pub fn word_addr(&self) -> WordAddrR {
        WordAddrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Specifies the word address where an error will be injected. - For cache SRAM ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[25:2\\]. On a FLASH macro refill to this word address and when the corresponding CM0/4_CA_CTL.RAM_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) is injected and stored in the cache. - For FLASH main interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[26:3\\]. On a FLASH main interface read and when FLASH_CTL.MAIN_ECC_INJ_EN bit is '1', the parity (PARITY\\[7:0\\]) replaces the FLASH macro parity (FLASH main interface read path is manipulated). - For FLASH work interface ECC, the word address WORD_ADDR\\[23:0\\]
is device address A\\[24:2\\]. On a FLASH work interface read and when FLASH_CTL.WORK_ECC_INJ_EN bit is '1', the parity (PARITY\\[6:0\\]) replaces the FLASH macro parity (FLASH work interface read path is manipulated)."]
    #[inline(always)]
    #[must_use]
    pub fn word_addr(&mut self) -> WordAddrW<EccCtlSpec> {
        WordAddrW::new(self, 0)
    }
    #[doc = "Bits 24:31 - ECC parity to use for ECC error injection at address WORD_ADDR. - For cache SRAM ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word. - For FLASH main interface ECC, the 8-bit parity PARITY\\[7:0\\]
is for a 64-bit word. - For FLASH work interface ECC, the 7-bit parity PARITY\\[6:0\\]
is for a 32-bit word."]
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
#[doc = "`reset()` method sets ECC_CTL to value 0"]
impl crate::Resettable for EccCtlSpec {
    const RESET_VALUE: u32 = 0;
}
