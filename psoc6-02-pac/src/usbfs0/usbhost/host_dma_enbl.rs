#[doc = "Register `HOST_DMA_ENBL` reader"]
pub type R = crate::R<HostDmaEnblSpec>;
#[doc = "Register `HOST_DMA_ENBL` writer"]
pub type W = crate::W<HostDmaEnblSpec>;
#[doc = "Field `DM_EP1DRQE` reader - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DmEp1drqeR = crate::BitReader;
#[doc = "Field `DM_EP1DRQE` writer - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
pub type DmEp1drqeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM_EP2DRQE` reader - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DmEp2drqeR = crate::BitReader;
#[doc = "Field `DM_EP2DRQE` writer - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
pub type DmEp2drqeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep1drqe(&self) -> DmEp1drqeR {
        DmEp1drqeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    pub fn dm_ep2drqe(&self) -> DmEp2drqeR {
        DmEp2drqeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - This bit enables DMA Request by EP1DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_ep1drqe(&mut self) -> DmEp1drqeW<HostDmaEnblSpec> {
        DmEp1drqeW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit enables DMA Request by EP2DRQ. '0' : Disable '1' : Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dm_ep2drqe(&mut self) -> DmEp2drqeW<HostDmaEnblSpec> {
        DmEp2drqeW::new(self, 3)
    }
}
#[doc = "Host DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_dma_enbl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_dma_enbl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostDmaEnblSpec;
impl crate::RegisterSpec for HostDmaEnblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_dma_enbl::R`](R) reader structure"]
impl crate::Readable for HostDmaEnblSpec {}
#[doc = "`write(|w| ..)` method takes [`host_dma_enbl::W`](W) writer structure"]
impl crate::Writable for HostDmaEnblSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_DMA_ENBL to value 0"]
impl crate::Resettable for HostDmaEnblSpec {
    const RESET_VALUE: u32 = 0;
}
