#[doc = "Register `TICKS` reader"]
pub type R = crate::R<TicksSpec>;
#[doc = "Field `CNT128HZ` reader - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
pub type Cnt128hzR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - 128Hz counter (msb=2Hz) When SECONDS is written this field will be reset."]
    #[inline(always)]
    pub fn cnt128hz(&self) -> Cnt128hzR {
        Cnt128hzR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "128Hz tick counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ticks::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TicksSpec;
impl crate::RegisterSpec for TicksSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ticks::R`](R) reader structure"]
impl crate::Readable for TicksSpec {}
#[doc = "`reset()` method sets TICKS to value 0"]
impl crate::Resettable for TicksSpec {
    const RESET_VALUE: u32 = 0;
}
