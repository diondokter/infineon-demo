#[doc = "Register `SATURATE_INTR_MASK` reader"]
pub type R = crate::R<SaturateIntrMaskSpec>;
#[doc = "Register `SATURATE_INTR_MASK` writer"]
pub type W = crate::W<SaturateIntrMaskSpec>;
#[doc = "Field `SATURATE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SaturateMaskR = crate::FieldReader<u16>;
#[doc = "Field `SATURATE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SaturateMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_mask(&self) -> SaturateMaskR {
        SaturateMaskR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn saturate_mask(&mut self) -> SaturateMaskW<SaturateIntrMaskSpec> {
        SaturateMaskW::new(self, 0)
    }
}
#[doc = "Saturate interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaturateIntrMaskSpec;
impl crate::RegisterSpec for SaturateIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr_mask::R`](R) reader structure"]
impl crate::Readable for SaturateIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`saturate_intr_mask::W`](W) writer structure"]
impl crate::Writable for SaturateIntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATURATE_INTR_MASK to value 0"]
impl crate::Resettable for SaturateIntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
