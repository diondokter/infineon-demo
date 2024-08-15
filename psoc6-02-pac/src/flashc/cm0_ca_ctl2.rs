#[doc = "Register `CM0_CA_CTL2` reader"]
pub type R = crate::R<Cm0CaCtl2Spec>;
#[doc = "Register `CM0_CA_CTL2` writer"]
pub type W = crate::W<Cm0CaCtl2Spec>;
#[doc = "Field `PWRUP_DELAY` reader - Number clock cycles delay needed after power domain power up"]
pub type PwrupDelayR = crate::FieldReader<u16>;
#[doc = "Field `PWRUP_DELAY` writer - Number clock cycles delay needed after power domain power up"]
pub type PwrupDelayW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn pwrup_delay(&self) -> PwrupDelayR {
        PwrupDelayR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_delay(&mut self) -> PwrupDelayW<Cm0CaCtl2Spec> {
        PwrupDelayW::new(self, 0)
    }
}
#[doc = "CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CaCtl2Spec;
impl crate::RegisterSpec for Cm0CaCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_ctl2::R`](R) reader structure"]
impl crate::Readable for Cm0CaCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`cm0_ca_ctl2::W`](W) writer structure"]
impl crate::Writable for Cm0CaCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_CA_CTL2 to value 0x012c"]
impl crate::Resettable for Cm0CaCtl2Spec {
    const RESET_VALUE: u32 = 0x012c;
}
