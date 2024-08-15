#[doc = "Register `TR_MON_AP_STATUS0` reader"]
pub type R = crate::R<TrMonApStatus0Spec>;
#[doc = "Field `BIT` reader - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_AP_STATUS1.OCC_COUNT is NOT equal to '0'."]
pub type BitR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current active bit value: '0': '0'. '1': '1'. This field is only valid when TR_MON_AP_STATUS1.OCC_COUNT is NOT equal to '0'."]
    #[inline(always)]
    pub fn bit(&self) -> BitR {
        BitR::new((self.bits & 1) != 0)
    }
}
#[doc = "True random monitor AP status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonApStatus0Spec;
impl crate::RegisterSpec for TrMonApStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_ap_status0::R`](R) reader structure"]
impl crate::Readable for TrMonApStatus0Spec {}
#[doc = "`reset()` method sets TR_MON_AP_STATUS0 to value 0"]
impl crate::Resettable for TrMonApStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
