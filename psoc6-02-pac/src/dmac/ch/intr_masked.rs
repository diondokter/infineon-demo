#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `COMPLETION` reader - Logical and of corresponding INTR.COMPLETION and INTR_MASK.COMPLETION fields."]
pub type CompletionR = crate::BitReader;
#[doc = "Field `SRC_BUS_ERROR` reader - Logical and of corresponding INTR.SRC_BUS_ERROR and INTR_MASK.SRC_BUS_ERROR fields."]
pub type SrcBusErrorR = crate::BitReader;
#[doc = "Field `DST_BUS_ERROR` reader - Logical and of corresponding INTR.DST_BUS_ERROR and INTR_MASK.DST_BUS_ERROR fields."]
pub type DstBusErrorR = crate::BitReader;
#[doc = "Field `SRC_MISAL` reader - Logical and of corresponding INTR.SRC_MISAL and INTR_MASK.SRC_MISAL fields."]
pub type SrcMisalR = crate::BitReader;
#[doc = "Field `DST_MISAL` reader - Logical and of corresponding INTR.DST_MISAL and INTR_MASK.DST_MISAL fields."]
pub type DstMisalR = crate::BitReader;
#[doc = "Field `CURR_PTR_NULL` reader - Logical and of corresponding INTR.CURR_PTR_NULL and INTR_MASK.CURR_PTR_NULL fields."]
pub type CurrPtrNullR = crate::BitReader;
#[doc = "Field `ACTIVE_CH_DISABLED` reader - Logical and of corresponding INTR.ACTIVE_CH_DISABLED and INTR_MASK.ACTIVE_CH_DISABLED fields."]
pub type ActiveChDisabledR = crate::BitReader;
#[doc = "Field `DESCR_BUS_ERROR` reader - Logical and of corresponding INTR.DESCR_BUS_ERROR and INTR_MASK.DESCR_BUS_ERROR fields."]
pub type DescrBusErrorR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding INTR.COMPLETION and INTR_MASK.COMPLETION fields."]
    #[inline(always)]
    pub fn completion(&self) -> CompletionR {
        CompletionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding INTR.SRC_BUS_ERROR and INTR_MASK.SRC_BUS_ERROR fields."]
    #[inline(always)]
    pub fn src_bus_error(&self) -> SrcBusErrorR {
        SrcBusErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding INTR.DST_BUS_ERROR and INTR_MASK.DST_BUS_ERROR fields."]
    #[inline(always)]
    pub fn dst_bus_error(&self) -> DstBusErrorR {
        DstBusErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding INTR.SRC_MISAL and INTR_MASK.SRC_MISAL fields."]
    #[inline(always)]
    pub fn src_misal(&self) -> SrcMisalR {
        SrcMisalR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding INTR.DST_MISAL and INTR_MASK.DST_MISAL fields."]
    #[inline(always)]
    pub fn dst_misal(&self) -> DstMisalR {
        DstMisalR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding INTR.CURR_PTR_NULL and INTR_MASK.CURR_PTR_NULL fields."]
    #[inline(always)]
    pub fn curr_ptr_null(&self) -> CurrPtrNullR {
        CurrPtrNullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding INTR.ACTIVE_CH_DISABLED and INTR_MASK.ACTIVE_CH_DISABLED fields."]
    #[inline(always)]
    pub fn active_ch_disabled(&self) -> ActiveChDisabledR {
        ActiveChDisabledR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding INTR.DESCR_BUS_ERROR and INTR_MASK.DESCR_BUS_ERROR fields."]
    #[inline(always)]
    pub fn descr_bus_error(&self) -> DescrBusErrorR {
        DescrBusErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMaskedSpec;
impl crate::RegisterSpec for IntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_masked::R`](R) reader structure"]
impl crate::Readable for IntrMaskedSpec {}
#[doc = "`reset()` method sets INTR_MASKED to value 0"]
impl crate::Resettable for IntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
