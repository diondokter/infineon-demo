#[doc = "Register `INTR_CAUSE` reader"]
pub type R = crate::R<IntrCauseSpec>;
#[doc = "Field `COUNTER_INT` reader - Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
pub type CounterIntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Counters interrupt signal active. If the counter is disabled through CTRL.COUNTER_ENABLED, the associated interrupt field is immediately set to '0'."]
    #[inline(always)]
    pub fn counter_int(&self) -> CounterIntR {
        CounterIntR::new(self.bits)
    }
}
#[doc = "TCPWM Counter interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCauseSpec;
impl crate::RegisterSpec for IntrCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause::R`](R) reader structure"]
impl crate::Readable for IntrCauseSpec {}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for IntrCauseSpec {
    const RESET_VALUE: u32 = 0;
}
