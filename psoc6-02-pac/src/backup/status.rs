#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RTC_BUSY` reader - pending RTC write"]
pub type RtcBusyR = crate::BitReader;
#[doc = "Field `WCO_OK` reader - Indicates that output has transitioned."]
pub type WcoOkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - pending RTC write"]
    #[inline(always)]
    pub fn rtc_busy(&self) -> RtcBusyR {
        RtcBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that output has transitioned."]
    #[inline(always)]
    pub fn wco_ok(&self) -> WcoOkR {
        WcoOkR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
