#[doc = "Register `PWR_HIBERNATE` reader"]
pub type R = crate::R<PwrHibernateSpec>;
#[doc = "Register `PWR_HIBERNATE` writer"]
pub type W = crate::W<PwrHibernateSpec>;
#[doc = "Field `TOKEN` reader - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type TokenR = crate::FieldReader;
#[doc = "Field `TOKEN` writer - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type TokenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UNLOCK` reader - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub type UnlockR = crate::FieldReader;
#[doc = "Field `UNLOCK` writer - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub type UnlockW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FREEZE` reader - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HIBALARM` reader - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub type MaskHibalarmR = crate::BitReader;
#[doc = "Field `MASK_HIBALARM` writer - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub type MaskHibalarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK_HIBWDT` reader - When set, HIBERNATE will wakeup if WDT matches"]
pub type MaskHibwdtR = crate::BitReader;
#[doc = "Field `MASK_HIBWDT` writer - When set, HIBERNATE will wakeup if WDT matches"]
pub type MaskHibwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY_HIBPIN` reader - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub type PolarityHibpinR = crate::FieldReader;
#[doc = "Field `POLARITY_HIBPIN` writer - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub type PolarityHibpinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MASK_HIBPIN` reader - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub type MaskHibpinR = crate::FieldReader;
#[doc = "Field `MASK_HIBPIN` writer - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub type MaskHibpinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HIBERNATE_DISABLE` reader - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub type HibernateDisableR = crate::BitReader;
#[doc = "Field `HIBERNATE_DISABLE` writer - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub type HibernateDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIBERNATE` reader - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub type HibernateR = crate::BitReader;
#[doc = "Field `HIBERNATE` writer - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub type HibernateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&self) -> TokenR {
        TokenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&self) -> UnlockR {
        UnlockR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&self) -> MaskHibalarmR {
        MaskHibalarmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&self) -> MaskHibwdtR {
        MaskHibwdtR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&self) -> PolarityHibpinR {
        PolarityHibpinR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&self) -> MaskHibpinR {
        MaskHibpinR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&self) -> HibernateDisableR {
        HibernateDisableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&self) -> HibernateR {
        HibernateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    #[must_use]
    pub fn token(&mut self) -> TokenW<PwrHibernateSpec> {
        TokenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UnlockW<PwrHibernateSpec> {
        UnlockW::new(self, 8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<PwrHibernateSpec> {
        FreezeW::new(self, 17)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibalarm(&mut self) -> MaskHibalarmW<PwrHibernateSpec> {
        MaskHibalarmW::new(self, 18)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibwdt(&mut self) -> MaskHibwdtW<PwrHibernateSpec> {
        MaskHibwdtW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    #[must_use]
    pub fn polarity_hibpin(&mut self) -> PolarityHibpinW<PwrHibernateSpec> {
        PolarityHibpinW::new(self, 20)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    #[must_use]
    pub fn mask_hibpin(&mut self) -> MaskHibpinW<PwrHibernateSpec> {
        MaskHibpinW::new(self, 24)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    #[must_use]
    pub fn hibernate_disable(&mut self) -> HibernateDisableW<PwrHibernateSpec> {
        HibernateDisableW::new(self, 30)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    #[must_use]
    pub fn hibernate(&mut self) -> HibernateW<PwrHibernateSpec> {
        HibernateW::new(self, 31)
    }
}
#[doc = "HIBERNATE Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_hibernate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_hibernate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrHibernateSpec;
impl crate::RegisterSpec for PwrHibernateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_hibernate::R`](R) reader structure"]
impl crate::Readable for PwrHibernateSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_hibernate::W`](W) writer structure"]
impl crate::Writable for PwrHibernateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_HIBERNATE to value 0"]
impl crate::Resettable for PwrHibernateSpec {
    const RESET_VALUE: u32 = 0;
}
