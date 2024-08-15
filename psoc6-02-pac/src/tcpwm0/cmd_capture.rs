#[doc = "Register `CMD_CAPTURE` reader"]
pub type R = crate::R<CmdCaptureSpec>;
#[doc = "Register `CMD_CAPTURE` writer"]
pub type W = crate::W<CmdCaptureSpec>;
#[doc = "Field `COUNTER_CAPTURE` reader - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type CounterCaptureR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_CAPTURE` writer - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
pub type CounterCaptureW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_capture(&self) -> CounterCaptureR {
        CounterCaptureR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW capture trigger. When written with '1', a capture trigger is generated and the HW sets the field to '0' when the SW trigger has taken effect. It should be noted that the HW operates on the counter frequency. If the counter is disabled through CTRL.COUNTER_ENABLED, the field is immediately set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn counter_capture(&mut self) -> CounterCaptureW<CmdCaptureSpec> {
        CounterCaptureW::new(self, 0)
    }
}
#[doc = "TCPWM capture command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_capture::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_capture::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdCaptureSpec;
impl crate::RegisterSpec for CmdCaptureSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_capture::R`](R) reader structure"]
impl crate::Readable for CmdCaptureSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_capture::W`](W) writer structure"]
impl crate::Writable for CmdCaptureSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_CAPTURE to value 0"]
impl crate::Resettable for CmdCaptureSpec {
    const RESET_VALUE: u32 = 0;
}
