#[doc = "Register `TR_MON_RC_STATUS0` reader"]
pub type R = crate::R<TrMonRcStatus0Spec>;
#[doc = "Field `BIT` reader - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_RC_STATUS1.REP_COUNT is NOT equal to '0'."]
pub type BitR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_RC_STATUS1.REP_COUNT is NOT equal to '0'."]
    #[inline(always)]
    pub fn bit(&self) -> BitR {
        BitR::new((self.bits & 1) != 0)
    }
}
#[doc = "True random monitor RC status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonRcStatus0Spec;
impl crate::RegisterSpec for TrMonRcStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_rc_status0::R`](R) reader structure"]
impl crate::Readable for TrMonRcStatus0Spec {}
#[doc = "`reset()` method sets TR_MON_RC_STATUS0 to value 0"]
impl crate::Resettable for TrMonRcStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
