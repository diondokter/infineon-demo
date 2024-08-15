#[doc = "Register `PWR_HIB_DATA[%s]` reader"]
pub type R = crate::R<PwrHibDataSpec>;
#[doc = "Register `PWR_HIB_DATA[%s]` writer"]
pub type W = crate::W<PwrHibDataSpec>;
#[doc = "Field `HIB_DATA` reader - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type HibDataR = crate::FieldReader<u32>;
#[doc = "Field `HIB_DATA` writer - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub type HibDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&self) -> HibDataR {
        HibDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    #[must_use]
    pub fn hib_data(&mut self) -> HibDataW<PwrHibDataSpec> {
        HibDataW::new(self, 0)
    }
}
#[doc = "HIBERNATE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_hib_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_hib_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrHibDataSpec;
impl crate::RegisterSpec for PwrHibDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_hib_data::R`](R) reader structure"]
impl crate::Readable for PwrHibDataSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_hib_data::W`](W) writer structure"]
impl crate::Writable for PwrHibDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_HIB_DATA[%s]
to value 0"]
impl crate::Resettable for PwrHibDataSpec {
    const RESET_VALUE: u32 = 0;
}
