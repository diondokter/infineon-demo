#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `SAMPLE` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SampleR = crate::BitReader;
#[doc = "Field `SAMPLE` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INIT` reader - Mask bit for corresponding bit in interrupt request register."]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Mask bit for corresponding bit in interrupt request register."]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_RES` reader - Mask bit for corresponding bit in interrupt request register."]
pub type AdcResR = crate::BitReader;
#[doc = "Field `ADC_RES` writer - Mask bit for corresponding bit in interrupt request register."]
pub type AdcResW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn sample(&self) -> SampleR {
        SampleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn adc_res(&self) -> AdcResR {
        AdcResR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn sample(&mut self) -> SampleW<IntrMaskSpec> {
        SampleW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<IntrMaskSpec> {
        InitW::new(self, 2)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn adc_res(&mut self) -> AdcResW<IntrMaskSpec> {
        AdcResW::new(self, 8)
    }
}
#[doc = "CSD Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
