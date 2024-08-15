#[doc = "Register `SAMPLE_TIME01` reader"]
pub type R = crate::R<SampleTime01Spec>;
#[doc = "Register `SAMPLE_TIME01` writer"]
pub type W = crate::W<SampleTime01Spec>;
#[doc = "Field `SAMPLE_TIME0` reader - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SampleTime0R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME0` writer - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
pub type SampleTime0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SAMPLE_TIME1` reader - Sample time1"]
pub type SampleTime1R = crate::FieldReader<u16>;
#[doc = "Field `SAMPLE_TIME1` writer - Sample time1"]
pub type SampleTime1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    pub fn sample_time0(&self) -> SampleTime0R {
        SampleTime0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    pub fn sample_time1(&self) -> SampleTime1R {
        SampleTime1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sample time0 (aperture) in ADC clock cycles. Note that actual sample time is one clock less than specified here. The minimum sample time is 167ns, which is 3.0 cycles (4 in this field) with an 18MHz clock. Minimum legal value in this register is 2."]
    #[inline(always)]
    #[must_use]
    pub fn sample_time0(&mut self) -> SampleTime0W<SampleTime01Spec> {
        SampleTime0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Sample time1"]
    #[inline(always)]
    #[must_use]
    pub fn sample_time1(&mut self) -> SampleTime1W<SampleTime01Spec> {
        SampleTime1W::new(self, 16)
    }
}
#[doc = "Sample time specification ST0 and ST1\n\nYou can [`read`](crate::Reg::read) this register and get [`sample_time01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sample_time01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampleTime01Spec;
impl crate::RegisterSpec for SampleTime01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sample_time01::R`](R) reader structure"]
impl crate::Readable for SampleTime01Spec {}
#[doc = "`write(|w| ..)` method takes [`sample_time01::W`](W) writer structure"]
impl crate::Writable for SampleTime01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAMPLE_TIME01 to value 0x0003_0003"]
impl crate::Resettable for SampleTime01Spec {
    const RESET_VALUE: u32 = 0x0003_0003;
}
