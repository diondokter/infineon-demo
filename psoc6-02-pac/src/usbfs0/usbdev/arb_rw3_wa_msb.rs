#[doc = "Register `ARB_RW3_WA_MSB` reader"]
pub type R = crate::R<ArbRw3WaMsbSpec>;
#[doc = "Register `ARB_RW3_WA_MSB` writer"]
pub type W = crate::W<ArbRw3WaMsbSpec>;
#[doc = "Field `WA_MSB` reader - Write Address for EP"]
pub type WaMsbR = crate::BitReader;
#[doc = "Field `WA_MSB` writer - Write Address for EP"]
pub type WaMsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    pub fn wa_msb(&self) -> WaMsbR {
        WaMsbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn wa_msb(&mut self) -> WaMsbW<ArbRw3WaMsbSpec> {
        WaMsbW::new(self, 0)
    }
}
#[doc = "Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw3_wa_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw3_wa_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw3WaMsbSpec;
impl crate::RegisterSpec for ArbRw3WaMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw3_wa_msb::R`](R) reader structure"]
impl crate::Readable for ArbRw3WaMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw3_wa_msb::W`](W) writer structure"]
impl crate::Writable for ArbRw3WaMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW3_WA_MSB to value 0"]
impl crate::Resettable for ArbRw3WaMsbSpec {
    const RESET_VALUE: u32 = 0;
}
