#[doc = "Register `ARB_RW8_DR16` reader"]
pub type R = crate::R<ArbRw8Dr16Spec>;
#[doc = "Register `ARB_RW8_DR16` writer"]
pub type W = crate::W<ArbRw8Dr16Spec>;
#[doc = "Field `DR16` reader - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type Dr16R = crate::FieldReader<u16>;
#[doc = "Field `DR16` writer - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
pub type Dr16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr16(&self) -> Dr16R {
        Dr16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    #[must_use]
    pub fn dr16(&mut self) -> Dr16W<ArbRw8Dr16Spec> {
        Dr16W::new(self, 0)
    }
}
#[doc = "Endpoint Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw8_dr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw8_dr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw8Dr16Spec;
impl crate::RegisterSpec for ArbRw8Dr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw8_dr16::R`](R) reader structure"]
impl crate::Readable for ArbRw8Dr16Spec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw8_dr16::W`](W) writer structure"]
impl crate::Writable for ArbRw8Dr16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW8_DR16 to value 0"]
impl crate::Resettable for ArbRw8Dr16Spec {
    const RESET_VALUE: u32 = 0;
}
