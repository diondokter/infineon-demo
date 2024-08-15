#[doc = "Register `TR_STATUS` reader"]
pub type R = crate::R<TrStatusSpec>;
#[doc = "Field `INITIALIZED` reader - Reflects the state of the true random number generator: '0': Not initialized (TR_CTL0.INIT_DELAY has NOT passed). '1': Initialized (TR_CTL0.INIT_DELAY has passed)."]
pub type InitializedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reflects the state of the true random number generator: '0': Not initialized (TR_CTL0.INIT_DELAY has NOT passed). '1': Initialized (TR_CTL0.INIT_DELAY has passed)."]
    #[inline(always)]
    pub fn initialized(&self) -> InitializedR {
        InitializedR::new((self.bits & 1) != 0)
    }
}
#[doc = "True random status\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrStatusSpec;
impl crate::RegisterSpec for TrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_status::R`](R) reader structure"]
impl crate::Readable for TrStatusSpec {}
#[doc = "`reset()` method sets TR_STATUS to value 0"]
impl crate::Resettable for TrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
