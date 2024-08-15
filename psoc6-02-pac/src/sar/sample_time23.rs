#[doc = "Register `SAMPLE_TIME23` reader"]
pub type R = crate::R<SampleTime23Spec>;
#[doc = "Register `SAMPLE_TIME23` writer"]
pub type W = crate::W<SampleTime23Spec>;
#[doc = "Field `SAMPLE_TIME2` reader - Sample time2"]
pub type SampleTime2R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME2` writer - Sample time2"]
pub type SampleTime2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SAMPLE_TIME3` reader - Sample time3"]
pub type SampleTime3R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME3` writer - Sample time3"]
pub type SampleTime3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    pub fn sample_time2(&self) -> SampleTime2R {
        SampleTime2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    pub fn sample_time3(&self) -> SampleTime3R {
        SampleTime3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time2"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time2(&mut self) -> SampleTime2W<SampleTime23Spec> {
        SampleTime2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sample time3"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time3(&mut self) -> SampleTime3W<SampleTime23Spec> {
        SampleTime3W::new(self, 16)
    }
}
#[doc = "Sample time specification ST2 and ST3\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_time23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_time23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleTime23Spec;
impl crate::RegisterSpec for SampleTime23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_time23::R`](R) reader structure"]
impl crate::Readable for SampleTime23Spec {}
#[doc = "`write(|w| ..)` method takes [`sample_time23::W`](W) writer structure"]
impl crate::Writable for SampleTime23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLE_TIME23 to value 0x0003_0003"]
impl crate::Resettable for SampleTime23Spec {
    const RESET_VALUE: u32 = 0x0003_0003;
}
