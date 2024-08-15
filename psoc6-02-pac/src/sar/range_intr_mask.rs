#[doc = "Register `RANGE_INTR_MASK` reader"]
pub type R = crate::R<RangeIntrMaskSpec>;
#[doc = "Register `RANGE_INTR_MASK` writer"]
pub type W = crate::W<RangeIntrMaskSpec>;
#[doc = "Field `RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type RangeMaskR = crate::FieldReader<u16>;
#[doc = "Field `RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type RangeMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_mask(&self) -> RangeMaskR {
        RangeMaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn range_mask(&mut self) -> RangeMaskW<RangeIntrMaskSpec> {
        RangeMaskW::new(self, 0)
    }
}
#[doc = "Range detect interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeIntrMaskSpec;
impl crate::RegisterSpec for RangeIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr_mask::R`](R) reader structure"]
impl crate::Readable for RangeIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`range_intr_mask::W`](W) writer structure"]
impl crate::Writable for RangeIntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANGE_INTR_MASK to value 0"]
impl crate::Resettable for RangeIntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
