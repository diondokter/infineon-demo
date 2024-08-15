#[doc = "Register `ANA_TRIM0` reader"]
pub type R = crate::R<AnaTrim0Spec>;
#[doc = "Register `ANA_TRIM0` writer"]
pub type W = crate::W<AnaTrim0Spec>;
#[doc = "Field `CAP_TRIM` reader - Attenuation cap trimming"]
pub type CapTrimR = crate::FieldReader;
#[doc = "Field `CAP_TRIM` writer - Attenuation cap trimming"]
pub type CapTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIMUNIT` reader - Attenuation cap trimming"]
pub type TrimunitR = crate::BitReader;
#[doc = "Field `TRIMUNIT` writer - Attenuation cap trimming"]
pub type TrimunitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn cap_trim(&self) -> CapTrimR {
        CapTrimR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    pub fn trimunit(&self) -> TrimunitR {
        TrimunitR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Attenuation cap trimming"]
    #[inline(always)]
    #[must_use]
    pub fn cap_trim(&mut self) -> CapTrimW<AnaTrim0Spec> {
        CapTrimW::new(self, 0)
    }
    #[doc = "Bit 5 - Attenuation cap trimming"]
    #[inline(always)]
    #[must_use]
    pub fn trimunit(&mut self) -> TrimunitW<AnaTrim0Spec> {
        TrimunitW::new(self, 5)
    }
}
#[doc = "Analog trim register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_trim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_trim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaTrim0Spec;
impl crate::RegisterSpec for AnaTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_trim0::R`](R) reader structure"]
impl crate::Readable for AnaTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_trim0::W`](W) writer structure"]
impl crate::Writable for AnaTrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_TRIM0 to value 0"]
impl crate::Resettable for AnaTrim0Spec {
    const RESET_VALUE: u32 = 0;
}
