#[doc = "Register `CMD_START` reader"]
pub type R = crate::R<CmdStartSpec>;
#[doc = "Register `CMD_START` writer"]
pub type W = crate::W<CmdStartSpec>;
#[doc = "Field `COUNTER_START` reader - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterStartR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_START` writer - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterStartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_start(&self) -> CounterStartR {
        CounterStartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW start trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    #[must_use]
    pub fn counter_start(&mut self) -> CounterStartW<CmdStartSpec> {
        CounterStartW::new(self, 0)
    }
}
#[doc = "TCPWM start command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdStartSpec;
impl crate::RegisterSpec for CmdStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_start::R`](R) reader structure"]
impl crate::Readable for CmdStartSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_start::W`](W) writer structure"]
impl crate::Writable for CmdStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_START to value 0"]
impl crate::Resettable for CmdStartSpec {
    const RESET_VALUE: u32 = 0;
}
