#[doc = "Register `ARB_RW1_RA_MSB` reader"]
pub type R = crate::R<ArbRw1RaMsbSpec>;
#[doc = "Register `ARB_RW1_RA_MSB` writer"]
pub type W = crate::W<ArbRw1RaMsbSpec>;
#[doc = "Field `RA_MSB` reader - Read Address for EP"]
pub type RaMsbR = crate::BitReader;
#[doc = "Field `RA_MSB` writer - Read Address for EP"]
pub type RaMsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    pub fn ra_msb(&self) -> RaMsbR {
        RaMsbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn ra_msb(&mut self) -> RaMsbW<ArbRw1RaMsbSpec> {
        RaMsbW::new(self, 0)
    }
}
#[doc = "Endpoint Read Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw1_ra_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw1_ra_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw1RaMsbSpec;
impl crate::RegisterSpec for ArbRw1RaMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw1_ra_msb::R`](R) reader structure"]
impl crate::Readable for ArbRw1RaMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw1_ra_msb::W`](W) writer structure"]
impl crate::Writable for ArbRw1RaMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW1_RA_MSB to value 0"]
impl crate::Resettable for ArbRw1RaMsbSpec {
    const RESET_VALUE: u32 = 0;
}
