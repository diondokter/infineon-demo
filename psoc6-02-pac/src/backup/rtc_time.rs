#[doc = "Register `RTC_TIME` reader"]
pub type R = crate::R<RtcTimeSpec>;
#[doc = "Register `RTC_TIME` writer"]
pub type W = crate::W<RtcTimeSpec>;
#[doc = "Field `RTC_SEC` reader - Calendar seconds in BCD, 0-59"]
pub type RtcSecR = crate::FieldReader;
#[doc = "Field `RTC_SEC` writer - Calendar seconds in BCD, 0-59"]
pub type RtcSecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RTC_MIN` reader - Calendar minutes in BCD, 0-59"]
pub type RtcMinR = crate::FieldReader;
#[doc = "Field `RTC_MIN` writer - Calendar minutes in BCD, 0-59"]
pub type RtcMinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RTC_HOUR` reader - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
pub type RtcHourR = crate::FieldReader;
#[doc = "Field `RTC_HOUR` writer - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
pub type RtcHourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CTRL_12HR` reader - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub type Ctrl12hrR = crate::BitReader;
#[doc = "Field `CTRL_12HR` writer - Select 12/24HR mode: 1=12HR, 0=24HR"]
pub type Ctrl12hrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_DAY` reader - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type RtcDayR = crate::FieldReader;
#[doc = "Field `RTC_DAY` writer - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type RtcDayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_sec(&self) -> RtcSecR {
        RtcSecR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn rtc_min(&self) -> RtcMinR {
        RtcMinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    pub fn rtc_hour(&self) -> RtcHourR {
        RtcHourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    pub fn ctrl_12hr(&self) -> Ctrl12hrR {
        Ctrl12hrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn rtc_day(&self) -> RtcDayR {
        RtcDayR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calendar seconds in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_sec(&mut self) -> RtcSecW<RtcTimeSpec> {
        RtcSecW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Calendar minutes in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_min(&mut self) -> RtcMinW<RtcTimeSpec> {
        RtcMinW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Calendar hours in BCD, value depending on 12/24HR mode 0=24HR: \\[21:16\\]=0-23 1=12HR: \\[21\\]:0=AM, 1=PM, \\[20:16\\]=1-12"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_hour(&mut self) -> RtcHourW<RtcTimeSpec> {
        RtcHourW::new(self, 16)
    }
    #[doc = "Bit 22 - Select 12/24HR mode: 1=12HR, 0=24HR"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_12hr(&mut self) -> Ctrl12hrW<RtcTimeSpec> {
        Ctrl12hrW::new(self, 22)
    }
    #[doc = "Bits 24:26 - Calendar Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_day(&mut self) -> RtcDayW<RtcTimeSpec> {
        RtcDayW::new(self, 24)
    }
}
#[doc = "Calendar Seconds, Minutes, Hours, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTimeSpec;
impl crate::RegisterSpec for RtcTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_time::R`](R) reader structure"]
impl crate::Readable for RtcTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_time::W`](W) writer structure"]
impl crate::Writable for RtcTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTC_TIME to value 0"]
impl crate::Resettable for RtcTimeSpec {
    const RESET_VALUE: u32 = 0;
}
