#[doc = "Register `MONITOR_CTL_0` reader"]
pub type R = crate::R<MonitorCtl0Spec>;
#[doc = "Register `MONITOR_CTL_0` writer"]
pub type W = crate::W<MonitorCtl0Spec>;
#[doc = "Field `MONITOR_EN` reader - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MonitorEnR = crate::FieldReader<u32>;
#[doc = "Field `MONITOR_EN` writer - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
pub type MonitorEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn monitor_en(&self) -> MonitorEnR {
        MonitorEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - control for switch, which connects the power/ground supply to AMUXBUS_A/B respectively when switch is closed: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn monitor_en(&mut self) -> MonitorEnW<MonitorCtl0Spec> {
        MonitorEnW::new(self, 0)
    }
}
#[doc = "Power/Ground Monitor cell control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`monitor_ctl_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`monitor_ctl_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MonitorCtl0Spec;
impl crate::RegisterSpec for MonitorCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`monitor_ctl_0::R`](R) reader structure"]
impl crate::Readable for MonitorCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`monitor_ctl_0::W`](W) writer structure"]
impl crate::Writable for MonitorCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MONITOR_CTL_0 to value 0"]
impl crate::Resettable for MonitorCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
