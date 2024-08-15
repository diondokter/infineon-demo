#[doc = "Register `VREF_TRIM3` reader"]
pub type R = crate::R<VrefTrim3Spec>;
#[doc = "Register `VREF_TRIM3` writer"]
pub type W = crate::W<VrefTrim3Spec>;
#[doc = "Field `VREF_ATTEN_TRIM` reader - Obsolete"]
pub type VrefAttenTrimR = crate::FieldReader;
#[doc = "Field `VREF_ATTEN_TRIM` writer - Obsolete"]
pub type VrefAttenTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    pub fn vref_atten_trim(&self) -> VrefAttenTrimR {
        VrefAttenTrimR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Obsolete"]
    #[inline(always)]
    #[must_use]
    pub fn vref_atten_trim(&mut self) -> VrefAttenTrimW<VrefTrim3Spec> {
        VrefAttenTrimW::new(self, 0)
    }
}
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefTrim3Spec;
impl crate::RegisterSpec for VrefTrim3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim3::R`](R) reader structure"]
impl crate::Readable for VrefTrim3Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_trim3::W`](W) writer structure"]
impl crate::Writable for VrefTrim3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF_TRIM3 to value 0"]
impl crate::Resettable for VrefTrim3Spec {
    const RESET_VALUE: u32 = 0;
}
