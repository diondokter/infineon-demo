#[doc = "Register `CM4_PWR_DELAY_CTL` reader"]
pub type R = crate::R<Cm4PwrDelayCtlSpec>;
#[doc = "Register `CM4_PWR_DELAY_CTL` writer"]
pub type W = crate::W<Cm4PwrDelayCtlSpec>;
#[doc = "Field `UP` reader - Number clock cycles delay needed after power domain power up"]
pub type UpR = crate::FieldReader<u16>;
#[doc = "Field `UP` writer - Number clock cycles delay needed after power domain power up"]
pub type UpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number clock cycles delay needed after power domain power up"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UpW<Cm4PwrDelayCtlSpec> {
        UpW::new(self, 0)
    }
}
#[doc = "CM4 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_pwr_delay_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_pwr_delay_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4PwrDelayCtlSpec;
impl crate::RegisterSpec for Cm4PwrDelayCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_pwr_delay_ctl::R`](R) reader structure"]
impl crate::Readable for Cm4PwrDelayCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_pwr_delay_ctl::W`](W) writer structure"]
impl crate::Writable for Cm4PwrDelayCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_PWR_DELAY_CTL to value 0x012c"]
impl crate::Resettable for Cm4PwrDelayCtlSpec {
    const RESET_VALUE: u32 = 0x012c;
}
