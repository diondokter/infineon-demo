#[doc = "Register `OSCCNT` reader"]
pub type R = crate::R<OsccntSpec>;
#[doc = "Field `CNT32KHZ` reader - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
pub type Cnt32khzR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 32kHz oscillator count (msb=128Hz), calibration can cause bit 6 to skip. Reset when RTC_TIME.RTC_SEC fields is written."]
    #[inline(always)]
    pub fn cnt32khz(&self) -> Cnt32khzR {
        Cnt32khzR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "32kHz oscillator counter\n\nYou can [`read`](crate::Reg::read) this register and get [`osccnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsccntSpec;
impl crate::RegisterSpec for OsccntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osccnt::R`](R) reader structure"]
impl crate::Readable for OsccntSpec {}
#[doc = "`reset()` method sets OSCCNT to value 0"]
impl crate::Resettable for OsccntSpec {
    const RESET_VALUE: u32 = 0;
}
