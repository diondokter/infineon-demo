#[doc = "Register `INTR_HOST_EP_MASK` reader"]
pub type R = crate::R<IntrHostEpMaskSpec>;
#[doc = "Register `INTR_HOST_EP_MASK` writer"]
pub type W = crate::W<IntrHostEpMaskSpec>;
#[doc = "Field `EP1DRQM` reader - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type Ep1drqmR = crate::BitReader;
#[doc = "Field `EP1DRQM` writer - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
pub type Ep1drqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPKM` reader - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type Ep1spkmR = crate::BitReader;
#[doc = "Field `EP1SPKM` writer - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
pub type Ep1spkmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQM` reader - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type Ep2drqmR = crate::BitReader;
#[doc = "Field `EP2DRQM` writer - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
pub type Ep2drqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPKM` reader - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type Ep2spkmR = crate::BitReader;
#[doc = "Field `EP2SPKM` writer - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
pub type Ep2spkmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1drqm(&self) -> Ep1drqmR {
        Ep1drqmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep1spkm(&self) -> Ep1spkmR {
        Ep1spkmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2drqm(&self) -> Ep2drqmR {
        Ep2drqmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    pub fn ep2spkm(&self) -> Ep2spkmR {
        Ep2spkmR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit masks the interrupt by EP1DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep1drqm(&mut self) -> Ep1drqmW<IntrHostEpMaskSpec> {
        Ep1drqmW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit masks the interrupt by EP1SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep1spkm(&mut self) -> Ep1spkmW<IntrHostEpMaskSpec> {
        Ep1spkmW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit masks the interrupt by EP2DRQ flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep2drqm(&mut self) -> Ep2drqmW<IntrHostEpMaskSpec> {
        Ep2drqmW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit masks the interrupt by EP2SPK flag. '0' : Disables '1' : Enables"]
    #[inline(always)]
    #[must_use]
    pub fn ep2spkm(&mut self) -> Ep2spkmW<IntrHostEpMaskSpec> {
        Ep2spkmW::new(self, 5)
    }
}
#[doc = "Interrupt USB Host Endpoint Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrHostEpMaskSpec;
impl crate::RegisterSpec for IntrHostEpMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_mask::R`](R) reader structure"]
impl crate::Readable for IntrHostEpMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep_mask::W`](W) writer structure"]
impl crate::Writable for IntrHostEpMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP_MASK to value 0"]
impl crate::Resettable for IntrHostEpMaskSpec {
    const RESET_VALUE: u32 = 0;
}
