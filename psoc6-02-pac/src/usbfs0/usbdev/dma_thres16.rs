#[doc = "Register `DMA_THRES16` reader"]
pub type R = crate::R<DmaThres16Spec>;
#[doc = "Register `DMA_THRES16` writer"]
pub type W = crate::W<DmaThres16Spec>;
#[doc = "Field `DMA_THS16` reader - DMA Threshold count"]
pub type DmaThs16R = crate::FieldReader<u16>;
#[doc = "Field `DMA_THS16` writer - DMA Threshold count"]
pub type DmaThs16W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    pub fn dma_ths16(&self) -> DmaThs16R {
        DmaThs16R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - DMA Threshold count"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ths16(&mut self) -> DmaThs16W<DmaThres16Spec> {
        DmaThs16W::new(self, 0)
    }
}
#[doc = "DMA Burst / Threshold Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_thres16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_thres16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaThres16Spec;
impl crate::RegisterSpec for DmaThres16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_thres16::R`](R) reader structure"]
impl crate::Readable for DmaThres16Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_thres16::W`](W) writer structure"]
impl crate::Writable for DmaThres16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_THRES16 to value 0"]
impl crate::Resettable for DmaThres16Spec {
    const RESET_VALUE: u32 = 0;
}
