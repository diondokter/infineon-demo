#[doc = "Register `INTR_HOST_EP_MASKED` reader"]
pub type R = crate::R<IntrHostEpMaskedSpec>;
#[doc = "Field `EP1DRQED` reader - This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
pub type Ep1drqedR = crate::BitReader;
#[doc = "Field `EP1SPKED` reader - This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
pub type Ep1spkedR = crate::BitReader;
#[doc = "Field `EP2DRQED` reader - This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
pub type Ep2drqedR = crate::BitReader;
#[doc = "Field `EP2SPKED` reader - This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
pub type Ep2spkedR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - This bit indicates the interrupt by EP1DRQ flag. '0' : Doesn't request the interrupt by EP1DRQ '1' : Request the interrupt by EP1DRQ"]
    #[inline(always)]
    pub fn ep1drqed(&self) -> Ep1drqedR {
        Ep1drqedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit indicates the interrupt by EP1SPK flag. '0' : Doesn't request the interrupt by EP1SPK '1' : Request the interrupt by EP1SPK"]
    #[inline(always)]
    pub fn ep1spked(&self) -> Ep1spkedR {
        Ep1spkedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit indicates the interrupt by EP2DRQ flag. '0' : Doesn't request the interrupt by EP2DRQ '1' : Request the interrupt by EP2DRQ"]
    #[inline(always)]
    pub fn ep2drqed(&self) -> Ep2drqedR {
        Ep2drqedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit indicates the interrupt by EP2SPK flag. '0' : Doesn't request the interrupt by EP2SPK '1' : Request the interrupt by EP2SPK"]
    #[inline(always)]
    pub fn ep2spked(&self) -> Ep2spkedR {
        Ep2spkedR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Endpoint Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrHostEpMaskedSpec;
impl crate::RegisterSpec for IntrHostEpMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_masked::R`](R) reader structure"]
impl crate::Readable for IntrHostEpMaskedSpec {}
#[doc = "`reset()` method sets INTR_HOST_EP_MASKED to value 0"]
impl crate::Resettable for IntrHostEpMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
