#[doc = "Register `INTR_HOST_EP_SET` reader"]
pub type R = crate::R<IntrHostEpSetSpec>;
#[doc = "Register `INTR_HOST_EP_SET` writer"]
pub type W = crate::W<IntrHostEpSetSpec>;
#[doc = "Field `EP1DRQS` reader - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type Ep1drqsR = crate::BitReader;
#[doc = "Field `EP1DRQS` writer - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
pub type Ep1drqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1SPKS` reader - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type Ep1spksR = crate::BitReader;
#[doc = "Field `EP1SPKS` writer - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
pub type Ep1spksW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2DRQS` reader - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type Ep2drqsR = crate::BitReader;
#[doc = "Field `EP2DRQS` writer - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
pub type Ep2drqsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2SPKS` reader - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type Ep2spksR = crate::BitReader;
#[doc = "Field `EP2SPKS` writer - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
pub type Ep2spksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep1drqs(&self) -> Ep1drqsR {
        Ep1drqsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep1spks(&self) -> Ep1spksR {
        Ep1spksR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    pub fn ep2drqs(&self) -> Ep2drqsR {
        Ep2drqsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    pub fn ep2spks(&self) -> Ep2spksR {
        Ep2spksR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit sets EP1DRQ bit. If this bit is written to '1', EP1DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1DRQ can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep1drqs(&mut self) -> Ep1drqsW<IntrHostEpSetSpec> {
        Ep1drqsW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets EP1SPK bit. If this bit is written to '1', EP1SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 1 Control Register (HOST_EP1_CTL) is '1', EP1SPK can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep1spks(&mut self) -> Ep1spksW<IntrHostEpSetSpec> {
        Ep1spksW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets EP2DRQ bit. If this bit is written to '1', EP2DRQ is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2DRQ can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep2drqs(&mut self) -> Ep2drqsW<IntrHostEpSetSpec> {
        Ep2drqsW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets EP2SPK bit. If this bit is written to '1', EP2SPK is set to '1'. However, if this bit is written with '0', its value is ignored. Note: If BFINI bit of the Host Endpoint 2 Control Register (HOST_EP2_CTL) is '1', EP2SPK can't be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ep2spks(&mut self) -> Ep2spksW<IntrHostEpSetSpec> {
        Ep2spksW::new(self, 5)
    }
}
#[doc = "Interrupt USB Host Endpoint Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_host_ep_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_host_ep_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrHostEpSetSpec;
impl crate::RegisterSpec for IntrHostEpSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_host_ep_set::R`](R) reader structure"]
impl crate::Readable for IntrHostEpSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_host_ep_set::W`](W) writer structure"]
impl crate::Writable for IntrHostEpSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_HOST_EP_SET to value 0"]
impl crate::Resettable for IntrHostEpSetSpec {
    const RESET_VALUE: u32 = 0;
}
