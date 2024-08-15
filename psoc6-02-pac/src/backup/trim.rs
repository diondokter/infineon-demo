#[doc = "Register `TRIM` reader"]
pub type R = crate::R<TrimSpec>;
#[doc = "Register `TRIM` writer"]
pub type W = crate::W<TrimSpec>;
#[doc = "Field `TRIM` reader - WCO trim"]
pub type TrimR = crate::FieldReader;
#[doc = "Field `TRIM` writer - WCO trim"]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WCO trim"]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<TrimSpec> {
        TrimW::new(self, 0)
    }
}
#[doc = "Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimSpec;
impl crate::RegisterSpec for TrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim::R`](R) reader structure"]
impl crate::Readable for TrimSpec {}
#[doc = "`write(|w| ..)` method takes [`trim::W`](W) writer structure"]
impl crate::Writable for TrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM to value 0"]
impl crate::Resettable for TrimSpec {
    const RESET_VALUE: u32 = 0;
}
