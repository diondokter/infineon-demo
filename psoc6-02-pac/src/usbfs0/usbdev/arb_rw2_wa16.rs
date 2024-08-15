#[doc = "Register `ARB_RW2_WA16` reader"]
pub type R = crate::R<ArbRw2Wa16Spec>;
#[doc = "Register `ARB_RW2_WA16` writer"]
pub type W = crate::W<ArbRw2Wa16Spec>;
#[doc = "Field `WA16` reader - Write Address for EP"]
pub type Wa16R = crate::FieldReader<u16>;
#[doc = "Field `WA16` writer - Write Address for EP"]
pub type Wa16W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Write Address for EP"]
    #[inline(always)]
    pub fn wa16(&self) -> Wa16R {
        Wa16R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn wa16(&mut self) -> Wa16W<ArbRw2Wa16Spec> {
        Wa16W::new(self, 0)
    }
}
#[doc = "Endpoint Write Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw2_wa16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw2_wa16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw2Wa16Spec;
impl crate::RegisterSpec for ArbRw2Wa16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw2_wa16::R`](R) reader structure"]
impl crate::Readable for ArbRw2Wa16Spec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw2_wa16::W`](W) writer structure"]
impl crate::Writable for ArbRw2Wa16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW2_WA16 to value 0"]
impl crate::Resettable for ArbRw2Wa16Spec {
    const RESET_VALUE: u32 = 0;
}
