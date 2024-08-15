#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `WIN_ACTIVE` reader - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
pub type WinActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates if the profiling time window is active. '0': Not active. '1': Active."]
    #[inline(always)]
    pub fn win_active(&self) -> WinActiveR {
        WinActiveR::new((self.bits & 1) != 0)
    }
}
#[doc = "Profile status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
