#[doc = "Register `ARB_RW5_WA` reader"]
pub type R = crate::R<ArbRw5WaSpec>;
#[doc = "Register `ARB_RW5_WA` writer"]
pub type W = crate::W<ArbRw5WaSpec>;
#[doc = "Field `WA` reader - Write Address for EP"]
pub type WaR = crate::FieldReader;
#[doc = "Field `WA` writer - Write Address for EP"]
pub type WaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    pub fn wa(&self) -> WaR {
        WaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for EP"]
    #[inline(always)]
    #[must_use]
    pub fn wa(&mut self) -> WaW<ArbRw5WaSpec> {
        WaW::new(self, 0)
    }
}
#[doc = "Endpoint Write Address value *1, *2\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_rw5_wa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_rw5_wa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbRw5WaSpec;
impl crate::RegisterSpec for ArbRw5WaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_rw5_wa::R`](R) reader structure"]
impl crate::Readable for ArbRw5WaSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_rw5_wa::W`](W) writer structure"]
impl crate::Writable for ArbRw5WaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_RW5_WA to value 0"]
impl crate::Resettable for ArbRw5WaSpec {
    const RESET_VALUE: u32 = 0;
}
