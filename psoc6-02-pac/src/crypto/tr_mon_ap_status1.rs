#[doc = "Register `TR_MON_AP_STATUS1` reader"]
pub type R = crate::R<TrMonApStatus1Spec>;
#[doc = "Field `OCC_COUNT` reader - Number of occurrences of the current active bit counter: '0': 0 occurrences ... '65535': 65535 occurrences"]
pub type OccCountR = crate::FieldReader<u16>;
#[doc = "Field `WINDOW_INDEX` reader - Counter to keep track of the current index in the window (counts from '0' to TR_MON_AP_CTL.WINDOW_SIZE to '0')."]
pub type WindowIndexR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of occurrences of the current active bit counter: '0': 0 occurrences ... '65535': 65535 occurrences"]
    #[inline(always)]
    pub fn occ_count(&self) -> OccCountR {
        OccCountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Counter to keep track of the current index in the window (counts from '0' to TR_MON_AP_CTL.WINDOW_SIZE to '0')."]
    #[inline(always)]
    pub fn window_index(&self) -> WindowIndexR {
        WindowIndexR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "True random monitor AP status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonApStatus1Spec;
impl crate::RegisterSpec for TrMonApStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_ap_status1::R`](R) reader structure"]
impl crate::Readable for TrMonApStatus1Spec {}
#[doc = "`reset()` method sets TR_MON_AP_STATUS1 to value 0"]
impl crate::Resettable for TrMonApStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
