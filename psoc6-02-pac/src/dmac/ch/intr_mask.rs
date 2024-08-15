#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `COMPLETION` reader - Mask for INTR.COMPLETION interrupt."]
pub type CompletionR = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Mask for INTR.COMPLETION interrupt."]
pub type CompletionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_BUS_ERROR` reader - Mask for INTR.SRC_BUS_ERROR interrupt."]
pub type SrcBusErrorR = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - Mask for INTR.SRC_BUS_ERROR interrupt."]
pub type SrcBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_BUS_ERROR` reader - Mask for INTR.DST_BUS_ERROR interrupt."]
pub type DstBusErrorR = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - Mask for INTR.DST_BUS_ERROR interrupt."]
pub type DstBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MISAL` reader - Mask for INTR.SRC_MISAL interrupt."]
pub type SrcMisalR = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - Mask for INTR.SRC_MISAL interrupt."]
pub type SrcMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_MISAL` reader - Mask for INTR.DST_MISAL interrupt."]
pub type DstMisalR = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - Mask for INTR.DST_MISAL interrupt."]
pub type DstMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_PTR_NULL` reader - Mask for INTR.CURR_PTR_NULL interrupt."]
pub type CurrPtrNullR = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - Mask for INTR.CURR_PTR_NULL interrupt."]
pub type CurrPtrNullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
pub type ActiveChDisabledR = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
pub type ActiveChDisabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Mask for INTR.DESCR_BUS_ERROR interrupt."]
pub type DescrBusErrorR = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - Mask for INTR.DESCR_BUS_ERROR interrupt."]
pub type DescrBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask for INTR.COMPLETION interrupt."]
    #[inline(always)]
    pub fn completion(&self) -> CompletionR {
        CompletionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask for INTR.SRC_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SrcBusErrorR {
        SrcBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for INTR.DST_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DstBusErrorR {
        DstBusErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for INTR.SRC_MISAL interrupt."]
    #[inline(always)]
    pub fn src_misal(&self) -> SrcMisalR {
        SrcMisalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask for INTR.DST_MISAL interrupt."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DstMisalR {
        DstMisalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask for INTR.CURR_PTR_NULL interrupt."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CurrPtrNullR {
        CurrPtrNullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ActiveChDisabledR {
        ActiveChDisabledR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask for INTR.DESCR_BUS_ERROR interrupt."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DescrBusErrorR {
        DescrBusErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for INTR.COMPLETION interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> CompletionW<IntrMaskSpec> {
        CompletionW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask for INTR.SRC_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SrcBusErrorW<IntrMaskSpec> {
        SrcBusErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask for INTR.DST_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DstBusErrorW<IntrMaskSpec> {
        DstBusErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask for INTR.SRC_MISAL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SrcMisalW<IntrMaskSpec> {
        SrcMisalW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask for INTR.DST_MISAL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DstMisalW<IntrMaskSpec> {
        DstMisalW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask for INTR.CURR_PTR_NULL interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CurrPtrNullW<IntrMaskSpec> {
        CurrPtrNullW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask for INTR.ACTIVE_CH_DISABLED interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ActiveChDisabledW<IntrMaskSpec> {
        ActiveChDisabledW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask for INTR.DESCR_BUS_ERROR interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DescrBusErrorW<IntrMaskSpec> {
        DescrBusErrorW::new(self, 7)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskSpec;
impl crate::RegisterSpec for IntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_mask::R`](R) reader structure"]
impl crate::Readable for IntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_mask::W`](W) writer structure"]
impl crate::Writable for IntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_MASK to value 0"]
impl crate::Resettable for IntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
