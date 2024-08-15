#[doc = "Register `PERIOD_BUFF` reader"]
pub type R = crate::R<PeriodBuffSpec>;
#[doc = "Register `PERIOD_BUFF` writer"]
pub type W = crate::W<PeriodBuffSpec>;
#[doc = "Field `PERIOD` reader - Additional buffer for counter PERIOD register."]
pub type PeriodR = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - Additional buffer for counter PERIOD register."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter PERIOD register."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<PeriodBuffSpec> {
        PeriodW::new(self, 0)
    }
}
#[doc = "Counter buffered period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period_buff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period_buff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodBuffSpec;
impl crate::RegisterSpec for PeriodBuffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period_buff::R`](R) reader structure"]
impl crate::Readable for PeriodBuffSpec {}
#[doc = "`write(|w| ..)` method takes [`period_buff::W`](W) writer structure"]
impl crate::Writable for PeriodBuffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIOD_BUFF to value 0xffff_ffff"]
impl crate::Resettable for PeriodBuffSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
