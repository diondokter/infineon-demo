#[doc = "Register `INTR_HOST_EP_CAUSE_LO` reader"]
pub type R = crate::R<IntrHostEpCauseLoSpec>;
#[doc = "Field `EP1DRQ_INT` reader - EP1DRQ interrupt"]
pub type Ep1drqIntR = crate::BitReader;
#[doc = "Field `EP1SPK_INT` reader - EP1SPK interrupt"]
pub type Ep1spkIntR = crate::BitReader;
#[doc = "Field `EP2DRQ_INT` reader - EP2DRQ interrupt"]
pub type Ep2drqIntR = crate::BitReader;
#[doc = "Field `EP2SPK_INT` reader - EP2SPK interrupt"]
pub type Ep2spkIntR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - EP1DRQ interrupt"]
    #[inline(always)]
    pub fn ep1drq_int(&self) -> Ep1drqIntR {
        Ep1drqIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EP1SPK interrupt"]
    #[inline(always)]
    pub fn ep1spk_int(&self) -> Ep1spkIntR {
        Ep1spkIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EP2DRQ interrupt"]
    #[inline(always)]
    pub fn ep2drq_int(&self) -> Ep2drqIntR {
        Ep2drqIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EP2SPK interrupt"]
    #[inline(always)]
    pub fn ep2spk_int(&self) -> Ep2spkIntR {
        Ep2spkIntR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Endpoint Cause Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_cause_lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrHostEpCauseLoSpec;
impl crate::RegisterSpec for IntrHostEpCauseLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_cause_lo::R`](R) reader structure"]
impl crate::Readable for IntrHostEpCauseLoSpec {}
#[doc = "`reset()` method sets INTR_HOST_EP_CAUSE_LO to value 0"]
impl crate::Resettable for IntrHostEpCauseLoSpec {
    const RESET_VALUE: u32 = 0;
}
