#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `SAMPLE` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SampleR = crate::BitReader;
#[doc = "Field `SAMPLE` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RES` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type AdcResR = crate::BitReader;
#[doc = "Field `ADC_RES` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type AdcResW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn adc_res(&self) -> AdcResR {
        AdcResR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SampleW<IntrSetSpec> {
        SampleW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<IntrSetSpec> {
        InitW::new(self, 2)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn adc_res(&mut self) -> AdcResW<IntrSetSpec> {
        AdcResW::new(self, 8)
    }
}
#[doc = "CSD Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
