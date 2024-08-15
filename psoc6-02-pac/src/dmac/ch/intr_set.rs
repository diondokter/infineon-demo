#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `COMPLETION` reader - Write this field with '1' to set INTR.COMPLETION field to '1' (a write of '0' has no effect)."]
pub type CompletionR = crate::BitReader;
#[doc = "Field `COMPLETION` writer - Write this field with '1' to set INTR.COMPLETION field to '1' (a write of '0' has no effect)."]
pub type CompletionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_BUS_ERROR` reader - Write this field with '1' to set INTR.SRC_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type SrcBusErrorR = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` writer - Write this field with '1' to set INTR.SRC_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type SrcBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_BUS_ERROR` reader - Write this field with '1' to set INTR.DST_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type DstBusErrorR = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` writer - Write this field with '1' to set INTR.DST_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type DstBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MISAL` reader - Write this field with '1' to set INTR.SRC_MISAL field to '1' (a write of '0' has no effect)."]
pub type SrcMisalR = crate::BitReader;
#[doc = "Field `SRC_MISAL` writer - Write this field with '1' to set INTR.SRC_MISAL field to '1' (a write of '0' has no effect)."]
pub type SrcMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DST_MISAL` reader - Write this field with '1' to set INTR.DST_MISAL field to '1' (a write of '0' has no effect)."]
pub type DstMisalR = crate::BitReader;
#[doc = "Field `DST_MISAL` writer - Write this field with '1' to set INTR.DST_MISAL field to '1' (a write of '0' has no effect)."]
pub type DstMisalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_PTR_NULL` reader - Write this field with '1' to set INTR.CURR_PTR_NULL field to '1' (a write of '0' has no effect)."]
pub type CurrPtrNullR = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` writer - Write this field with '1' to set INTR.CURR_PTR_NULL field to '1' (a write of '0' has no effect)."]
pub type CurrPtrNullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Write this field with '1' to set INTR.ACT_CH_DISABLED field to '1' (a write of '0' has no effect)."]
pub type ActiveChDisabledR = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` writer - Write this field with '1' to set INTR.ACT_CH_DISABLED field to '1' (a write of '0' has no effect)."]
pub type ActiveChDisabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESCR_BUS_ERROR` reader - Write this field with '1' to set INTR.DESCR_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type DescrBusErrorR = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` writer - Write this field with '1' to set INTR.DESCR_BUS_ERROR field to '1' (a write of '0' has no effect)."]
pub type DescrBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write this field with '1' to set INTR.COMPLETION field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn completion(&self) -> CompletionR {
        CompletionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write this field with '1' to set INTR.SRC_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SrcBusErrorR {
        SrcBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write this field with '1' to set INTR.DST_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DstBusErrorR {
        DstBusErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write this field with '1' to set INTR.SRC_MISAL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn src_misal(&self) -> SrcMisalR {
        SrcMisalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write this field with '1' to set INTR.DST_MISAL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DstMisalR {
        DstMisalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write this field with '1' to set INTR.CURR_PTR_NULL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CurrPtrNullR {
        CurrPtrNullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write this field with '1' to set INTR.ACT_CH_DISABLED field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ActiveChDisabledR {
        ActiveChDisabledR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write this field with '1' to set INTR.DESCR_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DescrBusErrorR {
        DescrBusErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write this field with '1' to set INTR.COMPLETION field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn completion(&mut self) -> CompletionW<IntrSetSpec> {
        CompletionW::new(self, 0)
    }
    #[doc = "Bit 1 - Write this field with '1' to set INTR.SRC_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn src_bus_error(&mut self) -> SrcBusErrorW<IntrSetSpec> {
        SrcBusErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - Write this field with '1' to set INTR.DST_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn dst_bus_error(&mut self) -> DstBusErrorW<IntrSetSpec> {
        DstBusErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Write this field with '1' to set INTR.SRC_MISAL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn src_misal(&mut self) -> SrcMisalW<IntrSetSpec> {
        SrcMisalW::new(self, 3)
    }
    #[doc = "Bit 4 - Write this field with '1' to set INTR.DST_MISAL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn dst_misal(&mut self) -> DstMisalW<IntrSetSpec> {
        DstMisalW::new(self, 4)
    }
    #[doc = "Bit 5 - Write this field with '1' to set INTR.CURR_PTR_NULL field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn curr_ptr_null(&mut self) -> CurrPtrNullW<IntrSetSpec> {
        CurrPtrNullW::new(self, 5)
    }
    #[doc = "Bit 6 - Write this field with '1' to set INTR.ACT_CH_DISABLED field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn active_ch_disabled(&mut self) -> ActiveChDisabledW<IntrSetSpec> {
        ActiveChDisabledW::new(self, 6)
    }
    #[doc = "Bit 7 - Write this field with '1' to set INTR.DESCR_BUS_ERROR field to '1' (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn descr_bus_error(&mut self) -> DescrBusErrorW<IntrSetSpec> {
        DescrBusErrorW::new(self, 7)
    }
}
#[doc = "Interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
