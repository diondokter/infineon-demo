#[doc = "Register `ALM1_DATE` reader"]
pub type R = crate::R<Alm1DateSpec>;
#[doc = "Register `ALM1_DATE` writer"]
pub type W = crate::W<Alm1DateSpec>;
#[doc = "Field `ALM_DATE` reader - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type AlmDateR = crate::FieldReader;
#[doc = "Field `ALM_DATE` writer - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
pub type AlmDateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALM_DATE_EN` reader - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type AlmDateEnR = crate::BitReader;
#[doc = "Field `ALM_DATE_EN` writer - Alarm Day of the Month enable: 0=ignore, 1=match"]
pub type AlmDateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_MON` reader - Alarm Month in BCD, 1-12"]
pub type AlmMonR = crate::FieldReader;
#[doc = "Field `ALM_MON` writer - Alarm Month in BCD, 1-12"]
pub type AlmMonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ALM_MON_EN` reader - Alarm Month enable: 0=ignore, 1=match"]
pub type AlmMonEnR = crate::BitReader;
#[doc = "Field `ALM_MON_EN` writer - Alarm Month enable: 0=ignore, 1=match"]
pub type AlmMonEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_EN` reader - Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type AlmEnR = crate::BitReader;
#[doc = "Field `ALM_EN` writer - Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
pub type AlmEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    pub fn alm_date(&self) -> AlmDateR {
        AlmDateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_date_en(&self) -> AlmDateEnR {
        AlmDateEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    pub fn alm_mon(&self) -> AlmMonR {
        AlmMonR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_mon_en(&self) -> AlmMonEnR {
        AlmMonEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    pub fn alm_en(&self) -> AlmEnR {
        AlmEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Alarm Day of the Month in BCD, 1-31 Leap Year corrected"]
    #[inline(always)]
    #[must_use]
    pub fn alm_date(&mut self) -> AlmDateW<Alm1DateSpec> {
        AlmDateW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm Day of the Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_date_en(&mut self) -> AlmDateEnW<Alm1DateSpec> {
        AlmDateEnW::new(self, 7)
    }
    #[doc = "Bits 8:12 - Alarm Month in BCD, 1-12"]
    #[inline(always)]
    #[must_use]
    pub fn alm_mon(&mut self) -> AlmMonW<Alm1DateSpec> {
        AlmMonW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm Month enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_mon_en(&mut self) -> AlmMonEnW<Alm1DateSpec> {
        AlmMonEnW::new(self, 15)
    }
    #[doc = "Bit 31 - Master enable for alarm 1. 0: Alarm 1 is disabled. Fields for date and time are ignored. 1: Alarm 1 is enabled. Alarm triggers whenever the new date and time matches all the enabled date and time fields, which can happen more than once depending on configuration. If none of the date and time fields are enabled, then this alarm triggers once every second."]
    #[inline(always)]
    #[must_use]
    pub fn alm_en(&mut self) -> AlmEnW<Alm1DateSpec> {
        AlmEnW::new(self, 31)
    }
}
#[doc = "Alarm 1 Day of Month, Month\n\nYou can [`read`](crate::Reg::read) this register and get [`alm1_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm1_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alm1DateSpec;
impl crate::RegisterSpec for Alm1DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alm1_date::R`](R) reader structure"]
impl crate::Readable for Alm1DateSpec {}
#[doc = "`write(|w| ..)` method takes [`alm1_date::W`](W) writer structure"]
impl crate::Writable for Alm1DateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALM1_DATE to value 0x0101"]
impl crate::Resettable for Alm1DateSpec {
    const RESET_VALUE: u32 = 0x0101;
}
