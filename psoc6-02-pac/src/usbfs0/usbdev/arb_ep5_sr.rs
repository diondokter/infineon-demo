#[doc = "Register `ARB_EP5_SR` reader"]
pub type R = crate::R<ArbEp5SrSpec>;
#[doc = "Register `ARB_EP5_SR` writer"]
pub type W = crate::W<ArbEp5SrSpec>;
#[doc = "Field `IN_BUF_FULL` reader - IN Endpoint Local Buffer Full Interrupt"]
pub type InBufFullR = crate::BitReader;
#[doc = "Field `IN_BUF_FULL` writer - IN Endpoint Local Buffer Full Interrupt"]
pub type InBufFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_GNT` reader - Endpoint DMA Grant Interrupt"]
pub type DmaGntR = crate::BitReader;
#[doc = "Field `DMA_GNT` writer - Endpoint DMA Grant Interrupt"]
pub type DmaGntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVER` reader - Endpoint Buffer Overflow Interrupt"]
pub type BufOverR = crate::BitReader;
#[doc = "Field `BUF_OVER` writer - Endpoint Buffer Overflow Interrupt"]
pub type BufOverW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDER` reader - Endpoint Buffer Underflow Interrupt"]
pub type BufUnderR = crate::BitReader;
#[doc = "Field `BUF_UNDER` writer - Endpoint Buffer Underflow Interrupt"]
pub type BufUnderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TERMIN` reader - Endpoint DMA Terminated Interrupt"]
pub type DmaTerminR = crate::BitReader;
#[doc = "Field `DMA_TERMIN` writer - Endpoint DMA Terminated Interrupt"]
pub type DmaTerminW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    pub fn in_buf_full(&self) -> InBufFullR {
        InBufFullR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    pub fn dma_gnt(&self) -> DmaGntR {
        DmaGntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    pub fn buf_over(&self) -> BufOverR {
        BufOverR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    pub fn buf_under(&self) -> BufUnderR {
        BufUnderR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    pub fn dma_termin(&self) -> DmaTerminR {
        DmaTerminR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn in_buf_full(&mut self) -> InBufFullW<ArbEp5SrSpec> {
        InBufFullW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_gnt(&mut self) -> DmaGntW<ArbEp5SrSpec> {
        DmaGntW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf_over(&mut self) -> BufOverW<ArbEp5SrSpec> {
        BufOverW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn buf_under(&mut self) -> BufUnderW<ArbEp5SrSpec> {
        BufUnderW::new(self, 3)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_termin(&mut self) -> DmaTerminW<ArbEp5SrSpec> {
        DmaTerminW::new(self, 5)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep5_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep5_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbEp5SrSpec;
impl crate::RegisterSpec for ArbEp5SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep5_sr::R`](R) reader structure"]
impl crate::Readable for ArbEp5SrSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_ep5_sr::W`](W) writer structure"]
impl crate::Writable for ArbEp5SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_EP5_SR to value 0"]
impl crate::Resettable for ArbEp5SrSpec {
    const RESET_VALUE: u32 = 0;
}
