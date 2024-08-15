#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `EC_BUSY` reader - Indicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ mode). This bit can be used by SW to determine whether it is safe to issue a SW access to the EZ memory (without bus wait states (a blocked SW access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether a SW access was actually blocked by externally clocked logic."]
pub type EcBusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates whether the externally clocked logic is potentially accessing the EZ memory (this is only possible in EZ mode). This bit can be used by SW to determine whether it is safe to issue a SW access to the EZ memory (without bus wait states (a blocked SW access) or bus errors being generated). Note that the INTR_TX.BLOCKED and INTR_RX.BLOCKED interrupt causes are used to indicate whether a SW access was actually blocked by externally clocked logic."]
    #[inline(always)]
    pub fn ec_busy(&self) -> EcBusyR {
        EcBusyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Generic status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
