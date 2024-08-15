#[doc = "Register `CTRL_CLR` reader"]
pub type R = crate::R<CtrlClrSpec>;
#[doc = "Register `CTRL_CLR` writer"]
pub type W = crate::W<CtrlClrSpec>;
#[doc = "Field `COUNTER_ENABLED` reader - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type CounterEnabledR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_ENABLED` writer - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
pub type CounterEnabledW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> CounterEnabledR {
        CounterEnabledR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Alias of CTRL that only allows disabling of counters. A write access: '0': Does nothing. '1': Clears respective COUNTER_ENABLED field. A read access returns CTRL.COUNTER_ENABLED."]
    #[inline(always)]
    #[must_use]
    pub fn counter_enabled(&mut self) -> CounterEnabledW<CtrlClrSpec> {
        CounterEnabledW::new(self, 0)
    }
}
#[doc = "TCPWM control clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlClrSpec;
impl crate::RegisterSpec for CtrlClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_clr::R`](R) reader structure"]
impl crate::Readable for CtrlClrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl_clr::W`](W) writer structure"]
impl crate::Writable for CtrlClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_CLR to value 0"]
impl crate::Resettable for CtrlClrSpec {
    const RESET_VALUE: u32 = 0;
}
