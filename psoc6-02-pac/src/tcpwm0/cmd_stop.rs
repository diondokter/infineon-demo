#[doc = "Register `CMD_STOP` reader"]
pub type R = crate::R<CmdStopSpec>;
#[doc = "Register `CMD_STOP` writer"]
pub type W = crate::W<CmdStopSpec>;
#[doc = "Field `COUNTER_STOP` reader - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterStopR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_STOP` writer - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterStopW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_stop(&self) -> CounterStopR {
        CounterStopR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW stop trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    #[must_use]
    pub fn counter_stop(&mut self) -> CounterStopW<CmdStopSpec> {
        CounterStopW::new(self, 0)
    }
}
#[doc = "TCPWM stop command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdStopSpec;
impl crate::RegisterSpec for CmdStopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_stop::R`](R) reader structure"]
impl crate::Readable for CmdStopSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_stop::W`](W) writer structure"]
impl crate::Writable for CmdStopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_STOP to value 0"]
impl crate::Resettable for CmdStopSpec {
    const RESET_VALUE: u32 = 0;
}
