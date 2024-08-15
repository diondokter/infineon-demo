#[doc = "Register `ARB_RW4_RA` reader"]
pub type R = crate::R<ArbRw4RaSpec>;
#[doc = "Register `ARB_RW4_RA` writer"]
pub type W = crate::W<ArbRw4RaSpec>;
#[doc = "Field `RA` reader - Read Address for EP"]
pub type RaR = crate::FieldReader;
#[doc = "Field `RA` writer - Read Address for EP"]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<ArbRw4RaSpec> {
        RaW::new(self, 0)
    }
}
#[doc = "Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw4_ra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw4_ra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw4RaSpec;
impl crate::RegisterSpec for ArbRw4RaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw4_ra::R`](R) reader structure"]
impl crate::Readable for ArbRw4RaSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw4_ra::W`](W) writer structure"]
impl crate::Writable for ArbRw4RaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW4_RA to value 0"]
impl crate::Resettable for ArbRw4RaSpec {
    const RESET_VALUE: u32 = 0;
}
