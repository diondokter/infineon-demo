#[doc = "Register `HOST_EP1_BLK` reader"]
pub type R = crate::R<HostEp1BlkSpec>;
#[doc = "Register `HOST_EP1_BLK` writer"]
pub type W = crate::W<HostEp1BlkSpec>;
#[doc = "Field `BLK_NUM` reader - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
pub type BlkNumR = crate::FieldReader<u16>;
#[doc = "Field `BLK_NUM` writer - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
pub type BlkNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&self) -> BlkNumR {
        BlkNumR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    #[must_use]
    pub fn blk_num(&mut self) -> BlkNumW<HostEp1BlkSpec> {
        BlkNumW::new(self, 16)
    }
}
#[doc = "Host Endpoint 1 Block Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ep1_blk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ep1_blk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEp1BlkSpec;
impl crate::RegisterSpec for HostEp1BlkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ep1_blk::R`](R) reader structure"]
impl crate::Readable for HostEp1BlkSpec {}
#[doc = "`write(|w| ..)` method takes [`host_ep1_blk::W`](W) writer structure"]
impl crate::Writable for HostEp1BlkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_EP1_BLK to value 0"]
impl crate::Resettable for HostEp1BlkSpec {
    const RESET_VALUE: u32 = 0;
}
