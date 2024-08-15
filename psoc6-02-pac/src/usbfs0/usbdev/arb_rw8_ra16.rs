#[doc = "Register `ARB_RW8_RA16` reader"]
pub type R = crate::R<ArbRw8Ra16Spec>;
#[doc = "Register `ARB_RW8_RA16` writer"]
pub type W = crate::W<ArbRw8Ra16Spec>;
#[doc = "Field `RA16` reader - Read Address for EP"]
pub type Ra16R = crate::FieldReader<u16>;
#[doc = "Field `RA16` writer - Read Address for EP"]
pub type Ra16W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    pub fn ra16(&self) -> Ra16R {
        Ra16R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra16(&mut self) -> Ra16W<ArbRw8Ra16Spec> {
        Ra16W::new(self, 0)
    }
}
#[doc = "Endpoint Read Address value *3\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_ra16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_ra16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw8Ra16Spec;
impl crate::RegisterSpec for ArbRw8Ra16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw8_ra16::R`](R) reader structure"]
impl crate::Readable for ArbRw8Ra16Spec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw8_ra16::W`](W) writer structure"]
impl crate::Writable for ArbRw8Ra16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW8_RA16 to value 0"]
impl crate::Resettable for ArbRw8Ra16Spec {
    const RESET_VALUE: u32 = 0;
}
