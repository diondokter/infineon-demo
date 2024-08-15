#[doc = "Register `ARB_EP3_INT_EN` reader"]
pub type R = crate::R<ArbEp3IntEnSpec>;
#[doc = "Register `ARB_EP3_INT_EN` writer"]
pub type W = crate::W<ArbEp3IntEnSpec>;
#[doc = "Field `IN_BUF_FULL_EN` reader - IN Endpoint Local Buffer Full Enable"]
pub type InBufFullEnR = crate::BitReader;
#[doc = "Field `IN_BUF_FULL_EN` writer - IN Endpoint Local Buffer Full Enable"]
pub type InBufFullEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_GNT_EN` reader - Endpoint DMA Grant Enable"]
pub type DmaGntEnR = crate::BitReader;
#[doc = "Field `DMA_GNT_EN` writer - Endpoint DMA Grant Enable"]
pub type DmaGntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVER_EN` reader - Endpoint Buffer Overflow Enable"]
pub type BufOverEnR = crate::BitReader;
#[doc = "Field `BUF_OVER_EN` writer - Endpoint Buffer Overflow Enable"]
pub type BufOverEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDER_EN` reader - Endpoint Buffer Underflow Enable"]
pub type BufUnderEnR = crate::BitReader;
#[doc = "Field `BUF_UNDER_EN` writer - Endpoint Buffer Underflow Enable"]
pub type BufUnderEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INT_EN` reader - Endpoint Error in Transaction Interrupt Enable"]
pub type ErrIntEnR = crate::BitReader;
#[doc = "Field `ERR_INT_EN` writer - Endpoint Error in Transaction Interrupt Enable"]
pub type ErrIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_TERMIN_EN` reader - Endpoint DMA Terminated Enable"]
pub type DmaTerminEnR = crate::BitReader;
#[doc = "Field `DMA_TERMIN_EN` writer - Endpoint DMA Terminated Enable"]
pub type DmaTerminEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    pub fn in_buf_full_en(&self) -> InBufFullEnR {
        InBufFullEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    pub fn dma_gnt_en(&self) -> DmaGntEnR {
        DmaGntEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    pub fn buf_over_en(&self) -> BufOverEnR {
        BufOverEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    pub fn buf_under_en(&self) -> BufUnderEnR {
        BufUnderEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn err_int_en(&self) -> ErrIntEnR {
        ErrIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    pub fn dma_termin_en(&self) -> DmaTerminEnR {
        DmaTerminEnR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IN Endpoint Local Buffer Full Enable"]
    #[inline(always)]
    #[must_use]
    pub fn in_buf_full_en(&mut self) -> InBufFullEnW<ArbEp3IntEnSpec> {
        InBufFullEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint DMA Grant Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_gnt_en(&mut self) -> DmaGntEnW<ArbEp3IntEnSpec> {
        DmaGntEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Buffer Overflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buf_over_en(&mut self) -> BufOverEnW<ArbEp3IntEnSpec> {
        BufOverEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Buffer Underflow Enable"]
    #[inline(always)]
    #[must_use]
    pub fn buf_under_en(&mut self) -> BufUnderEnW<ArbEp3IntEnSpec> {
        BufUnderEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Error in Transaction Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_int_en(&mut self) -> ErrIntEnW<ArbEp3IntEnSpec> {
        ErrIntEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint DMA Terminated Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma_termin_en(&mut self) -> DmaTerminEnW<ArbEp3IntEnSpec> {
        DmaTerminEnW::new(self, 5)
    }
}
#[doc = "Endpoint Interrupt Enable Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep3_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep3_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbEp3IntEnSpec;
impl crate::RegisterSpec for ArbEp3IntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep3_int_en::R`](R) reader structure"]
impl crate::Readable for ArbEp3IntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_ep3_int_en::W`](W) writer structure"]
impl crate::Writable for ArbEp3IntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_EP3_INT_EN to value 0"]
impl crate::Resettable for ArbEp3IntEnSpec {
    const RESET_VALUE: u32 = 0;
}
