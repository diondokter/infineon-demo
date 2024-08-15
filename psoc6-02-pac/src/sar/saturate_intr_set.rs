#[doc = "Register `SATURATE_INTR_SET` reader"]
pub type R = crate::R<SaturateIntrSetSpec>;
#[doc = "Register `SATURATE_INTR_SET` writer"]
pub type W = crate::W<SaturateIntrSetSpec>;
#[doc = "Field `SATURATE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SaturateSetR = crate::FieldReader<u16>;
#[doc = "Field `SATURATE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SaturateSetW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn saturate_set(&self) -> SaturateSetR {
        SaturateSetR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn saturate_set(&mut self) -> SaturateSetW<SaturateIntrSetSpec> {
        SaturateSetW::new(self, 0)
    }
}
#[doc = "Saturate interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaturateIntrSetSpec;
impl crate::RegisterSpec for SaturateIntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr_set::R`](R) reader structure"]
impl crate::Readable for SaturateIntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`saturate_intr_set::W`](W) writer structure"]
impl crate::Writable for SaturateIntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATURATE_INTR_SET to value 0"]
impl crate::Resettable for SaturateIntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
