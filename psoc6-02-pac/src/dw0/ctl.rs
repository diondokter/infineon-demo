#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `ECC_EN` reader - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - Enable ECC checking: '0': Disabled. '1': Enabled."]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC checking: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<CtlSpec> {
        EccEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable parity injection for SRAM. When '1', the parity (ECC_CTL.PARITY) is used when a full 32-bit write is done to the ECC_CTL.WORD_ADDR word address of the SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<CtlSpec> {
        EccInjEnW::new(self, 1)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. Disabling the IP activates the IP's Active logic reset: Active logic and non-retention MMIO registers are reset (retention MMIO registers are not affected). '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x01"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x01;
}
