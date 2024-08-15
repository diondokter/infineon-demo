#[doc = "Register `RANGE_INTR` reader"]
pub type R = crate::R<RangeIntrSpec>;
#[doc = "Register `RANGE_INTR` writer"]
pub type W = crate::W<RangeIntrSpec>;
#[doc = "Field `RANGE_INTR` reader - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RangeIntrR = crate::FieldReader<u16>;
#[doc = "Field `RANGE_INTR` writer - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type RangeIntrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn range_intr(&self) -> RangeIntrR {
        RangeIntrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn range_intr(&mut self) -> RangeIntrW<RangeIntrSpec> {
        RangeIntrW::new(self, 0)
    }
}
#[doc = "Range detect interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeIntrSpec;
impl crate::RegisterSpec for RangeIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_intr::R`](R) reader structure"]
impl crate::Readable for RangeIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`range_intr::W`](W) writer structure"]
impl crate::Writable for RangeIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANGE_INTR to value 0"]
impl crate::Resettable for RangeIntrSpec {
    const RESET_VALUE: u32 = 0;
}
