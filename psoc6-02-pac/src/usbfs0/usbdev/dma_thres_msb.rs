#[doc = "Register `DMA_THRES_MSB` reader"]
pub type R = crate::R<DmaThresMsbSpec>;
#[doc = "Register `DMA_THRES_MSB` writer"]
pub type W = crate::W<DmaThresMsbSpec>;
#[doc = "Field `DMA_THS_MSB` reader - DMA Threshold count"]
pub type DmaThsMsbR = crate::BitReader;
#[doc = "Field `DMA_THS_MSB` writer - DMA Threshold count"]
pub type DmaThsMsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths_msb(&self) -> DmaThsMsbR {
        DmaThsMsbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Threshold count"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ths_msb(&mut self) -> DmaThsMsbW<DmaThresMsbSpec> {
        DmaThsMsbW::new(self, 0)
    }
}
#[doc = "DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaThresMsbSpec;
impl crate::RegisterSpec for DmaThresMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_thres_msb::R`](R) reader structure"]
impl crate::Readable for DmaThresMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_thres_msb::W`](W) writer structure"]
impl crate::Writable for DmaThresMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_THRES_MSB to value 0"]
impl crate::Resettable for DmaThresMsbSpec {
    const RESET_VALUE: u32 = 0;
}
