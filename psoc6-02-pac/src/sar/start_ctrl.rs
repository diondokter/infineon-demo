#[doc = "Register `START_CTRL` reader"]
pub type R = crate::R<StartCtrlSpec>;
#[doc = "Register `START_CTRL` writer"]
pub type W = crate::W<StartCtrlSpec>;
#[doc = "Field `FW_TRIGGER` reader - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub type FwTriggerR = crate::BitReader;
#[doc = "Field `FW_TRIGGER` writer - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub type FwTriggerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&self) -> FwTriggerR {
        FwTriggerR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn fw_trigger(&mut self) -> FwTriggerW<StartCtrlSpec> {
        FwTriggerW::new(self, 0)
    }
}
#[doc = "Start control register (firmware trigger).\n\nYou can [`read`](crate::Reg::read) this register and get [`start_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartCtrlSpec;
impl crate::RegisterSpec for StartCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start_ctrl::R`](R) reader structure"]
impl crate::Readable for StartCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`start_ctrl::W`](W) writer structure"]
impl crate::Writable for StartCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START_CTRL to value 0"]
impl crate::Resettable for StartCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
