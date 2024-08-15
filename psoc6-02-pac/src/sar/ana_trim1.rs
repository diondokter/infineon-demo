#[doc = "Register `ANA_TRIM1` reader"]
pub type R = crate::R<AnaTrim1Spec>;
#[doc = "Register `ANA_TRIM1` writer"]
pub type W = crate::W<AnaTrim1Spec>;
#[doc = "Field `SAR_REF_BUF_TRIM` reader - SAR Reference buffer trim"]
pub type SarRefBufTrimR = crate::FieldReader;
#[doc = "Field `SAR_REF_BUF_TRIM` writer - SAR Reference buffer trim"]
pub type SarRefBufTrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    pub fn sar_ref_buf_trim(&self) -> SarRefBufTrimR {
        SarRefBufTrimR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - SAR Reference buffer trim"]
    #[inline(always)]
    #[must_use]
    pub fn sar_ref_buf_trim(&mut self) -> SarRefBufTrimW<AnaTrim1Spec> {
        SarRefBufTrimW::new(self, 0)
    }
}
#[doc = "Analog trim register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_trim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_trim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaTrim1Spec;
impl crate::RegisterSpec for AnaTrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_trim1::R`](R) reader structure"]
impl crate::Readable for AnaTrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_trim1::W`](W) writer structure"]
impl crate::Writable for AnaTrim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_TRIM1 to value 0"]
impl crate::Resettable for AnaTrim1Spec {
    const RESET_VALUE: u32 = 0;
}
