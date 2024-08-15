#[doc = "Register `VREF_TRIM1` reader"]
pub type R = crate::R<VrefTrim1Spec>;
#[doc = "Register `VREF_TRIM1` writer"]
pub type W = crate::W<VrefTrim1Spec>;
#[doc = "Field `VREF_TEMPCO_TRIM` reader - N/A"]
pub type VrefTempcoTrimR = crate::FieldReader;
#[doc = "Field `VREF_TEMPCO_TRIM` writer - N/A"]
pub type VrefTempcoTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_tempco_trim(&self) -> VrefTempcoTrimR {
        VrefTempcoTrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_tempco_trim(&mut self) -> VrefTempcoTrimW<VrefTrim1Spec> {
        VrefTempcoTrimW::new(self, 0)
    }
}
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefTrim1Spec;
impl crate::RegisterSpec for VrefTrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim1::R`](R) reader structure"]
impl crate::Readable for VrefTrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_trim1::W`](W) writer structure"]
impl crate::Writable for VrefTrim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF_TRIM1 to value 0"]
impl crate::Resettable for VrefTrim1Spec {
    const RESET_VALUE: u32 = 0;
}
