#[doc = "Register `CMD_RELOAD` reader"]
pub type R = crate::R<CmdReloadSpec>;
#[doc = "Register `CMD_RELOAD` writer"]
pub type W = crate::W<CmdReloadSpec>;
#[doc = "Field `COUNTER_RELOAD` reader - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterReloadR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_RELOAD` writer - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
pub type CounterReloadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    pub fn counter_reload(&self) -> CounterReloadR {
        CounterReloadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counters SW reload trigger. For HW behavior, see COUNTER_CAPTURE field."]
    #[inline(always)]
    #[must_use]
    pub fn counter_reload(&mut self) -> CounterReloadW<CmdReloadSpec> {
        CounterReloadW::new(self, 0)
    }
}
#[doc = "TCPWM reload command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_reload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_reload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdReloadSpec;
impl crate::RegisterSpec for CmdReloadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_reload::R`](R) reader structure"]
impl crate::Readable for CmdReloadSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_reload::W`](W) writer structure"]
impl crate::Writable for CmdReloadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_RELOAD to value 0"]
impl crate::Resettable for CmdReloadSpec {
    const RESET_VALUE: u32 = 0;
}
