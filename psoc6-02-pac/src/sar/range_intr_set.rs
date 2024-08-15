#[doc = "Register `RANGE_INTR_SET` reader"]
pub type R = crate::R<RangeIntrSetSpec>;
#[doc = "Register `RANGE_INTR_SET` writer"]
pub type W = crate::W<RangeIntrSetSpec>;
#[doc = "Field `RANGE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type RangeSetR = crate::FieldReader<u16>;
#[doc = "Field `RANGE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type RangeSetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn range_set(&self) -> RangeSetR {
        RangeSetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn range_set(&mut self) -> RangeSetW<RangeIntrSetSpec> {
        RangeSetW::new(self, 0)
    }
}
#[doc = "Range detect interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeIntrSetSpec;
impl crate::RegisterSpec for RangeIntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr_set::R`](R) reader structure"]
impl crate::Readable for RangeIntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`range_intr_set::W`](W) writer structure"]
impl crate::Writable for RangeIntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANGE_INTR_SET to value 0"]
impl crate::Resettable for RangeIntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
