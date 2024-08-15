#[doc = "Register `ALM1_TIME` reader"]
pub type R = crate::R<Alm1TimeSpec>;
#[doc = "Register `ALM1_TIME` writer"]
pub type W = crate::W<Alm1TimeSpec>;
#[doc = "Field `ALM_SEC` reader - Alarm seconds in BCD, 0-59"]
pub type AlmSecR = crate::FieldReader;
#[doc = "Field `ALM_SEC` writer - Alarm seconds in BCD, 0-59"]
pub type AlmSecW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALM_SEC_EN` reader - Alarm second enable: 0=ignore, 1=match"]
pub type AlmSecEnR = crate::BitReader;
#[doc = "Field `ALM_SEC_EN` writer - Alarm second enable: 0=ignore, 1=match"]
pub type AlmSecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_MIN` reader - Alarm minutes in BCD, 0-59"]
pub type AlmMinR = crate::FieldReader;
#[doc = "Field `ALM_MIN` writer - Alarm minutes in BCD, 0-59"]
pub type AlmMinW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ALM_MIN_EN` reader - Alarm minutes enable: 0=ignore, 1=match"]
pub type AlmMinEnR = crate::BitReader;
#[doc = "Field `ALM_MIN_EN` writer - Alarm minutes enable: 0=ignore, 1=match"]
pub type AlmMinEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_HOUR` reader - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type AlmHourR = crate::FieldReader;
#[doc = "Field `ALM_HOUR` writer - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
pub type AlmHourW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ALM_HOUR_EN` reader - Alarm hour enable: 0=ignore, 1=match"]
pub type AlmHourEnR = crate::BitReader;
#[doc = "Field `ALM_HOUR_EN` writer - Alarm hour enable: 0=ignore, 1=match"]
pub type AlmHourEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALM_DAY` reader - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type AlmDayR = crate::FieldReader;
#[doc = "Field `ALM_DAY` writer - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
pub type AlmDayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALM_DAY_EN` reader - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type AlmDayEnR = crate::BitReader;
#[doc = "Field `ALM_DAY_EN` writer - Alarm Day of the Week enable: 0=ignore, 1=match"]
pub type AlmDayEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_sec(&self) -> AlmSecR {
        AlmSecR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_sec_en(&self) -> AlmSecEnR {
        AlmSecEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    pub fn alm_min(&self) -> AlmMinR {
        AlmMinR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_min_en(&self) -> AlmMinEnR {
        AlmMinEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    pub fn alm_hour(&self) -> AlmHourR {
        AlmHourR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_hour_en(&self) -> AlmHourEnR {
        AlmHourEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    pub fn alm_day(&self) -> AlmDayR {
        AlmDayR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    pub fn alm_day_en(&self) -> AlmDayEnR {
        AlmDayEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Alarm seconds in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn alm_sec(&mut self) -> AlmSecW<Alm1TimeSpec> {
        AlmSecW::new(self, 0)
    }
    #[doc = "Bit 7 - Alarm second enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_sec_en(&mut self) -> AlmSecEnW<Alm1TimeSpec> {
        AlmSecEnW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Alarm minutes in BCD, 0-59"]
    #[inline(always)]
    #[must_use]
    pub fn alm_min(&mut self) -> AlmMinW<Alm1TimeSpec> {
        AlmMinW::new(self, 8)
    }
    #[doc = "Bit 15 - Alarm minutes enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_min_en(&mut self) -> AlmMinEnW<Alm1TimeSpec> {
        AlmMinEnW::new(self, 15)
    }
    #[doc = "Bits 16:21 - Alarm hours in BCD, value depending on 12/24HR mode 12HR: \\[5\\]:0=AM, 1=PM, \\[4:0\\]=1-12 24HR: \\[5:0\\]=0-23"]
    #[inline(always)]
    #[must_use]
    pub fn alm_hour(&mut self) -> AlmHourW<Alm1TimeSpec> {
        AlmHourW::new(self, 16)
    }
    #[doc = "Bit 23 - Alarm hour enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_hour_en(&mut self) -> AlmHourEnW<Alm1TimeSpec> {
        AlmHourEnW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Alarm Day of the week in BCD, 1-7 It is up to the user to define the meaning of the values, but 1=Monday is recommended"]
    #[inline(always)]
    #[must_use]
    pub fn alm_day(&mut self) -> AlmDayW<Alm1TimeSpec> {
        AlmDayW::new(self, 24)
    }
    #[doc = "Bit 31 - Alarm Day of the Week enable: 0=ignore, 1=match"]
    #[inline(always)]
    #[must_use]
    pub fn alm_day_en(&mut self) -> AlmDayEnW<Alm1TimeSpec> {
        AlmDayEnW::new(self, 31)
    }
}
#[doc = "Alarm 1 Seconds, Minute, Hours, Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`alm1_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alm1_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Alm1TimeSpec;
impl crate::RegisterSpec for Alm1TimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alm1_time::R`](R) reader structure"]
impl crate::Readable for Alm1TimeSpec {}
#[doc = "`write(|w| ..)` method takes [`alm1_time::W`](W) writer structure"]
impl crate::Writable for Alm1TimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALM1_TIME to value 0x0100_0000"]
impl crate::Resettable for Alm1TimeSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
