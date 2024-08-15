#[doc = "Register `RTC_DATE` reader"]
pub type R = crate::R<RtcDateSpec>;
#[doc = "Register `RTC_DATE` writer"]
pub type W = crate::W<RtcDateSpec>;
#[doc = "Field `RTC_DATE` reader - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RtcDateR = crate::FieldReader;
#[doc = "Field `RTC_DATE` writer - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
pub type RtcDateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RTC_MON` reader - Calendar Month in BCD, 1-12"]
pub type RtcMonR = crate::FieldReader;
#[doc = "Field `RTC_MON` writer - Calendar Month in BCD, 1-12"]
pub type RtcMonW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTC_YEAR` reader - Calendar year in BCD, 0-99"]
pub type RtcYearR = crate::FieldReader;
#[doc = "Field `RTC_YEAR` writer - Calendar year in BCD, 0-99"]
pub type RtcYearW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    pub fn rtc_date(&self) -> RtcDateR {
        RtcDateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    pub fn rtc_mon(&self) -> RtcMonR {
        RtcMonR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    pub fn rtc_year(&self) -> RtcYearR {
        RtcYearR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calendar Day of the Month in BCD, 1-31 Automatic Leap Year Correction"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_date(&mut self) -> RtcDateW<RtcDateSpec> {
        RtcDateW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Calendar Month in BCD, 1-12"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_mon(&mut self) -> RtcMonW<RtcDateSpec> {
        RtcMonW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Calendar year in BCD, 0-99"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_year(&mut self) -> RtcYearW<RtcDateSpec> {
        RtcYearW::new(self, 16)
    }
}
#[doc = "Calendar Day of Month, Month, Year\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcDateSpec;
impl crate::RegisterSpec for RtcDateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_date::R`](R) reader structure"]
impl crate::Readable for RtcDateSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_date::W`](W) writer structure"]
impl crate::Writable for RtcDateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_DATE to value 0"]
impl crate::Resettable for RtcDateSpec {
    const RESET_VALUE: u32 = 0;
}
