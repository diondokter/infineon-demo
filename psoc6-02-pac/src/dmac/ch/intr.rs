#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `COMPLETION` reader - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type CompletionR = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
pub type CompletionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load from the source."]
pub type SrcBusErrorR = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load from the source."]
pub type SrcBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_BUS_ERROR` reader - Activated (set to '1') on a bus error for a store to the destination."]
pub type DstBusErrorR = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - Activated (set to '1') on a bus error for a store to the destination."]
pub type DstBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MISAL` reader - Activated (set to '1') on a misalignment of the source address."]
pub type SrcMisalR = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - Activated (set to '1') on a misalignment of the source address."]
pub type SrcMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_MISAL` reader - Activated (set to '1') on a misalignment of the destination address."]
pub type DstMisalR = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - Activated (set to '1') on a misalignment of the destination address."]
pub type DstMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_PTR_NULL` reader - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
pub type CurrPtrNullR = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
pub type CurrPtrNullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
pub type ActiveChDisabledR = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
pub type ActiveChDisabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DescrBusErrorR = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - Activated (set to '1') on a bus error for a load of the descriptor."]
pub type DescrBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    pub fn completion(&self) -> CompletionR {
        CompletionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SrcBusErrorR {
        SrcBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DstBusErrorR {
        DstBusErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    pub fn src_misal(&self) -> SrcMisalR {
        SrcMisalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DstMisalR {
        DstMisalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CurrPtrNullR {
        CurrPtrNullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ActiveChDisabledR {
        ActiveChDisabledR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DescrBusErrorR {
        DescrBusErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated (set to '1') on completion of data transfer(s) as specified by the descriptor's CH_DESCR_CTL.INTR_TYPE."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> CompletionW<IntrSpec> {
        CompletionW::new(self, 0)
    }
    #[doc = "Bit 1 - Activated (set to '1') on a bus error for a load from the source."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SrcBusErrorW<IntrSpec> {
        SrcBusErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Activated (set to '1') on a bus error for a store to the destination."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DstBusErrorW<IntrSpec> {
        DstBusErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Activated (set to '1') on a misalignment of the source address."]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SrcMisalW<IntrSpec> {
        SrcMisalW::new(self, 3)
    }
    #[doc = "Bit 4 - Activated (set to '1') on a misalignment of the destination address."]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DstMisalW<IntrSpec> {
        DstMisalW::new(self, 4)
    }
    #[doc = "Bit 5 - Activated (set to '1') when the channel is enabled (CH_CTL.ENABLED is '1') and CH_CURR_PTR is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CurrPtrNullW<IntrSpec> {
        CurrPtrNullW::new(self, 5)
    }
    #[doc = "Bit 6 - Activated (set to '1') if the channel is disabled by SW (accidentally/incorrectly) when the data transfer engine is busy."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ActiveChDisabledW<IntrSpec> {
        ActiveChDisabledW::new(self, 6)
    }
    #[doc = "Bit 7 - Activated (set to '1') on a bus error for a load of the descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DescrBusErrorW<IntrSpec> {
        DescrBusErrorW::new(self, 7)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
