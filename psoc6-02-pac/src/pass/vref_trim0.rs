#[doc = "Register `VREF_TRIM0` reader"]
pub type R = crate::R<VrefTrim0Spec>;
#[doc = "Register `VREF_TRIM0` writer"]
pub type W = crate::W<VrefTrim0Spec>;
#[doc = "Field `VREF_ABS_TRIM` reader - N/A"]
pub type VrefAbsTrimR = crate::FieldReader;
#[doc = "Field `VREF_ABS_TRIM` writer - N/A"]
pub type VrefAbsTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn vref_abs_trim(&self) -> VrefAbsTrimR {
        VrefAbsTrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vref_abs_trim(&mut self) -> VrefAbsTrimW<VrefTrim0Spec> {
        VrefAbsTrimW::new(self, 0)
    }
}
#[doc = "VREF Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`vref_trim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref_trim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefTrim0Spec;
impl crate::RegisterSpec for VrefTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref_trim0::R`](R) reader structure"]
impl crate::Readable for VrefTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`vref_trim0::W`](W) writer structure"]
impl crate::Writable for VrefTrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF_TRIM0 to value 0"]
impl crate::Resettable for VrefTrim0Spec {
    const RESET_VALUE: u32 = 0;
}
