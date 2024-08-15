#[doc = "Register `PWR_TRIM_WAKE_CTL` reader"]
pub type R = crate::R<PwrTrimWakeCtlSpec>;
#[doc = "Register `PWR_TRIM_WAKE_CTL` writer"]
pub type W = crate::W<PwrTrimWakeCtlSpec>;
#[doc = "Field `WAKE_DELAY` reader - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WakeDelayR = crate::FieldReader;
#[doc = "Field `WAKE_DELAY` writer - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
pub type WakeDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    pub fn wake_delay(&self) -> WakeDelayR {
        WakeDelayR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wakeup holdoff. Spec (fastest) wake time is achieved with a setting of 0. Additional delay can be added for debugging or workaround. The delay is counted by the IMO."]
    #[inline(always)]
    #[must_use]
    pub fn wake_delay(&mut self) -> WakeDelayW<PwrTrimWakeCtlSpec> {
        WakeDelayW::new(self, 0)
    }
}
#[doc = "Wakeup Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_wake_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_wake_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrTrimWakeCtlSpec;
impl crate::RegisterSpec for PwrTrimWakeCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_wake_ctl::R`](R) reader structure"]
impl crate::Readable for PwrTrimWakeCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_wake_ctl::W`](W) writer structure"]
impl crate::Writable for PwrTrimWakeCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_WAKE_CTL to value 0"]
impl crate::Resettable for PwrTrimWakeCtlSpec {
    const RESET_VALUE: u32 = 0;
}
