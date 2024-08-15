#[doc = "Register `DMA_THRES` reader"]
pub type R = crate::R<DmaThresSpec>;
#[doc = "Register `DMA_THRES` writer"]
pub type W = crate::W<DmaThresSpec>;
#[doc = "Field `DMA_THS` reader - DMA Threshold count"]
pub type DmaThsR = crate::FieldReader;
#[doc = "Field `DMA_THS` writer - DMA Threshold count"]
pub type DmaThsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths(&self) -> DmaThsR {
        DmaThsR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Threshold count"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ths(&mut self) -> DmaThsW<DmaThresSpec> {
        DmaThsW::new(self, 0)
    }
}
#[doc = "DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaThresSpec;
impl crate::RegisterSpec for DmaThresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_thres::R`](R) reader structure"]
impl crate::Readable for DmaThresSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_thres::W`](W) writer structure"]
impl crate::Writable for DmaThresSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_THRES to value 0"]
impl crate::Resettable for DmaThresSpec {
    const RESET_VALUE: u32 = 0;
}
