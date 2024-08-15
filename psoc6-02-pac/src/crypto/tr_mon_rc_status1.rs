#[doc = "Register `TR_MON_RC_STATUS1` reader"]
pub type R = crate::R<TrMonRcStatus1Spec>;
#[doc = "Field `REP_COUNT` reader - Number of repetitions of the current active bit counter: '0': 0 repetitions. ... '255': 255 repetitions."]
pub type RepCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of repetitions of the current active bit counter: '0': 0 repetitions. ... '255': 255 repetitions."]
    #[inline(always)]
    pub fn rep_count(&self) -> RepCountR {
        RepCountR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "True random monitor RC status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonRcStatus1Spec;
impl crate::RegisterSpec for TrMonRcStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_rc_status1::R`](R) reader structure"]
impl crate::Readable for TrMonRcStatus1Spec {}
#[doc = "`reset()` method sets TR_MON_RC_STATUS1 to value 0"]
impl crate::Resettable for TrMonRcStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
