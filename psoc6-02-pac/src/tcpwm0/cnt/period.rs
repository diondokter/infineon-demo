#[doc = "Register `PERIOD` reader"]
pub type R = crate::R<PeriodSpec>;
#[doc = "Register `PERIOD` writer"]
pub type W = crate::W<PeriodSpec>;
#[doc = "Field `PERIOD` reader - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PeriodR = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Period value: upper value of the counter. When the counter should count for n cycles, this field should be set to n-1."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<PeriodSpec> {
        PeriodW::new(self, 0)
    }
}
#[doc = "Counter period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodSpec;
impl crate::RegisterSpec for PeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PeriodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIOD to value 0xffff_ffff"]
impl crate::Resettable for PeriodSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
