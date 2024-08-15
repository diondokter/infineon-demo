#[doc = "Register `RANGE_THRES` reader"]
pub type R = crate::R<RangeThresSpec>;
#[doc = "Register `RANGE_THRES` writer"]
pub type W = crate::W<RangeThresSpec>;
#[doc = "Field `RANGE_LOW` reader - Low threshold for range detect."]
pub type RangeLowR = crate::FieldReader<u16>;
#[doc = "Field `RANGE_LOW` writer - Low threshold for range detect."]
pub type RangeLowW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RANGE_HIGH` reader - High threshold for range detect."]
pub type RangeHighR = crate::FieldReader<u16>;
#[doc = "Field `RANGE_HIGH` writer - High threshold for range detect."]
pub type RangeHighW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    pub fn range_low(&self) -> RangeLowR {
        RangeLowR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    pub fn range_high(&self) -> RangeHighR {
        RangeHighR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low threshold for range detect."]
    #[inline(always)]
    #[must_use]
    pub fn range_low(&mut self) -> RangeLowW<RangeThresSpec> {
        RangeLowW::new(self, 0)
    }
    #[doc = "Bits 16:31 - High threshold for range detect."]
    #[inline(always)]
    #[must_use]
    pub fn range_high(&mut self) -> RangeHighW<RangeThresSpec> {
        RangeHighW::new(self, 16)
    }
}
#[doc = "Global range detect threshold register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeThresSpec;
impl crate::RegisterSpec for RangeThresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_thres::R`](R) reader structure"]
impl crate::Readable for RangeThresSpec {}
#[doc = "`write(|w| ..)` method takes [`range_thres::W`](W) writer structure"]
impl crate::Writable for RangeThresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANGE_THRES to value 0"]
impl crate::Resettable for RangeThresSpec {
    const RESET_VALUE: u32 = 0;
}
