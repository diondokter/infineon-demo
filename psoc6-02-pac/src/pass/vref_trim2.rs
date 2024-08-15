#[doc = "Register `VREF_TRIM2` reader"]
pub type R = crate::R<VrefTrim2Spec>;
#[doc = "Register `VREF_TRIM2` writer"]
pub type W = crate::W<VrefTrim2Spec>;
#[doc = "Field `VREF_CURV_TRIM` reader - N/A"]
pub type VrefCurvTrimR = crate::FieldReader;
#[doc = "Field `VREF_CURV_TRIM` writer - N/A"]
pub type VrefCurvTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_curv_trim(&self) -> VrefCurvTrimR {
        VrefCurvTrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_curv_trim(&mut self) -> VrefCurvTrimW<VrefTrim2Spec> {
        VrefCurvTrimW::new(self, 0)
    }
}
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefTrim2Spec;
impl crate::RegisterSpec for VrefTrim2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim2::R`](R) reader structure"]
impl crate::Readable for VrefTrim2Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_trim2::W`](W) writer structure"]
impl crate::Writable for VrefTrim2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF_TRIM2 to value 0"]
impl crate::Resettable for VrefTrim2Spec {
    const RESET_VALUE: u32 = 0;
}
