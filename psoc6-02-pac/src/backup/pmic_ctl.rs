#[doc = "Register `PMIC_CTL` reader"]
pub type R = crate::R<PmicCtlSpec>;
#[doc = "Register `PMIC_CTL` writer"]
pub type W = crate::W<PmicCtlSpec>;
#[doc = "Field `UNLOCK` reader - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
pub type UnlockR = crate::FieldReader;
#[doc = "Field `UNLOCK` writer - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
pub type UnlockW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `POLARITY` reader - N/A"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - N/A"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_EN_OUTEN` reader - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
pub type PmicEnOutenR = crate::BitReader;
#[doc = "Field `PMIC_EN_OUTEN` writer - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
pub type PmicEnOutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_ALWAYSEN` reader - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
pub type PmicAlwaysenR = crate::BitReader;
#[doc = "Field `PMIC_ALWAYSEN` writer - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
pub type PmicAlwaysenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMIC_EN` reader - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
pub type PmicEnR = crate::BitReader;
#[doc = "Field `PMIC_EN` writer - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
pub type PmicEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 29 - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    pub fn pmic_en_outen(&self) -> PmicEnOutenR {
        PmicEnOutenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    pub fn pmic_alwaysen(&self) -> PmicAlwaysenR {
        PmicAlwaysenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    pub fn pmic_en(&self) -> PmicEnR {
        PmicEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for PMIC to be disabled. When the UNLOCK code is not present: writes to PMIC_EN field are ignored and the hardware ignores the value in PMIC_EN. Do not change PMIC_EN in the same write cycle as setting/clearing the UNLOCK code; do these in separate write cycles."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UnlockW<PmicCtlSpec> {
        UnlockW::new(self, 8)
    }
    #[doc = "Bit 16 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<PmicCtlSpec> {
        PolarityW::new(self, 16)
    }
    #[doc = "Bit 29 - Output enable for the output driver in the PMIC_EN pad. 0: Output pad is tristate for PMIC_EN pin. This can allow this pin to be used for another purpose. Tristate condition is kept only if the UNLOCK key (0x3A) is present 1: Output pad is enabled for PMIC_EN pin."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_en_outen(&mut self) -> PmicEnOutenW<PmicCtlSpec> {
        PmicEnOutenW::new(self, 29)
    }
    #[doc = "Bit 30 - Override normal PMIC controls to prevent accidentally turning off the PMIC by errant firmware. 0: Normal operation, PMIC_EN and PMIC_OUTEN work as described 1: PMIC_EN and PMIC_OUTEN are ignored and the output pad is forced enabled. Note: This bit is a write-once bit until the next backup reset."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_alwaysen(&mut self) -> PmicAlwaysenW<PmicCtlSpec> {
        PmicAlwaysenW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable for external PMIC that supplies vddd (if present). This bit will only clear if UNLOCK was written correctly in a previous write operation and PMIC_ALWAYSEN=0. When PMIC_EN=0, the system functions normally until vddd is no longer present (OFF w/Backup mode). Firmware can set this bit, if it does so before vddd is actually removed. This bit is also set by any RTC alarm or PMIC pin wakeup event regardless of UNLOCK setting."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_en(&mut self) -> PmicEnW<PmicCtlSpec> {
        PmicEnW::new(self, 31)
    }
}
#[doc = "PMIC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmic_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmic_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmicCtlSpec;
impl crate::RegisterSpec for PmicCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmic_ctl::R`](R) reader structure"]
impl crate::Readable for PmicCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pmic_ctl::W`](W) writer structure"]
impl crate::Writable for PmicCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMIC_CTL to value 0xa000_0000"]
impl crate::Resettable for PmicCtlSpec {
    const RESET_VALUE: u32 = 0xa000_0000;
}
