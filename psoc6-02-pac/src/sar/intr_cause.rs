#[doc = "Register `INTR_CAUSE` reader"]
pub type R = crate::R<IntrCauseSpec>;
#[doc = "Field `EOS_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type EosMaskedMirR = crate::BitReader;
#[doc = "Field `OVERFLOW_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type OverflowMaskedMirR = crate::BitReader;
#[doc = "Field `FW_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type FwCollisionMaskedMirR = crate::BitReader;
#[doc = "Field `DSI_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type DsiCollisionMaskedMirR = crate::BitReader;
#[doc = "Field `INJ_EOC_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type InjEocMaskedMirR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type InjSaturateMaskedMirR = crate::BitReader;
#[doc = "Field `INJ_RANGE_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type InjRangeMaskedMirR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_MASKED_MIR` reader - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
pub type InjCollisionMaskedMirR = crate::BitReader;
#[doc = "Field `SATURATE_MASKED_RED` reader - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
pub type SaturateMaskedRedR = crate::BitReader;
#[doc = "Field `RANGE_MASKED_RED` reader - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
pub type RangeMaskedRedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn eos_masked_mir(&self) -> EosMaskedMirR {
        EosMaskedMirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn overflow_masked_mir(&self) -> OverflowMaskedMirR {
        OverflowMaskedMirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn fw_collision_masked_mir(&self) -> FwCollisionMaskedMirR {
        FwCollisionMaskedMirR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn dsi_collision_masked_mir(&self) -> DsiCollisionMaskedMirR {
        DsiCollisionMaskedMirR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_eoc_masked_mir(&self) -> InjEocMaskedMirR {
        InjEocMaskedMirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_saturate_masked_mir(&self) -> InjSaturateMaskedMirR {
        InjSaturateMaskedMirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_range_masked_mir(&self) -> InjRangeMaskedMirR {
        InjRangeMaskedMirR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mirror copy of corresponding bit in SAR_INTR_MASKED"]
    #[inline(always)]
    pub fn inj_collision_masked_mir(&self) -> InjCollisionMaskedMirR {
        InjCollisionMaskedMirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 30 - Reduction OR of all SAR_SATURATION_INTR_MASKED bits"]
    #[inline(always)]
    pub fn saturate_masked_red(&self) -> SaturateMaskedRedR {
        SaturateMaskedRedR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reduction OR of all SAR_RANGE_INTR_MASKED bits"]
    #[inline(always)]
    pub fn range_masked_red(&self) -> RangeMaskedRedR {
        RangeMaskedRedR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCauseSpec;
impl crate::RegisterSpec for IntrCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause::R`](R) reader structure"]
impl crate::Readable for IntrCauseSpec {}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for IntrCauseSpec {
    const RESET_VALUE: u32 = 0;
}
