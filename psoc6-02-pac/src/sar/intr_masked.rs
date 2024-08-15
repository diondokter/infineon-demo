#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `EOS_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type EosMaskedR = crate::BitReader;
#[doc = "Field `OVERFLOW_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type OverflowMaskedR = crate::BitReader;
#[doc = "Field `FW_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type FwCollisionMaskedR = crate::BitReader;
#[doc = "Field `DSI_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type DsiCollisionMaskedR = crate::BitReader;
#[doc = "Field `INJ_EOC_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type InjEocMaskedR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type InjSaturateMaskedR = crate::BitReader;
#[doc = "Field `INJ_RANGE_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type InjRangeMaskedR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type InjCollisionMaskedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn eos_masked(&self) -> EosMaskedR {
        EosMaskedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn overflow_masked(&self) -> OverflowMaskedR {
        OverflowMaskedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn fw_collision_masked(&self) -> FwCollisionMaskedR {
        FwCollisionMaskedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn dsi_collision_masked(&self) -> DsiCollisionMaskedR {
        DsiCollisionMaskedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_eoc_masked(&self) -> InjEocMaskedR {
        InjEocMaskedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_saturate_masked(&self) -> InjSaturateMaskedR {
        InjSaturateMaskedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_range_masked(&self) -> InjRangeMaskedR {
        InjRangeMaskedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn inj_collision_masked(&self) -> InjCollisionMaskedR {
        InjCollisionMaskedR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt masked request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
