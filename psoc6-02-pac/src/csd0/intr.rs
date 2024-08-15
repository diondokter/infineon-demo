#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `SAMPLE` reader - A normal sample is complete"]
pub type SampleR = crate::BitReader;
#[doc = "Field `SAMPLE` writer - A normal sample is complete"]
pub type SampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RES` reader - ADC Result ready"]
pub type AdcResR = crate::BitReader;
#[doc = "Field `ADC_RES` writer - ADC Result ready"]
pub type AdcResW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - A normal sample is complete"]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Result ready"]
    #[inline(always)]
    pub fn adc_res(&self) -> AdcResR {
        AdcResR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - A normal sample is complete"]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SampleW<IntrSpec> {
        SampleW::new(self, 1)
    }
    #[doc = "Bit 2 - Coarse initialization complete or Sample initialization complete (the latter is typically ignored)"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<IntrSpec> {
        InitW::new(self, 2)
    }
    #[doc = "Bit 8 - ADC Result ready"]
    #[inline(always)]
    #[must_use]
    pub fn adc_res(&mut self) -> AdcResW<IntrSpec> {
        AdcResW::new(self, 8)
    }
}
#[doc = "CSD Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
