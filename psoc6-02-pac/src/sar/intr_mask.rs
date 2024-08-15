#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `EOS_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EosMaskR = crate::BitReader;
#[doc = "Field `EOS_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EosMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type OverflowMaskR = crate::BitReader;
#[doc = "Field `OVERFLOW_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type OverflowMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type FwCollisionMaskR = crate::BitReader;
#[doc = "Field `FW_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type FwCollisionMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type DsiCollisionMaskR = crate::BitReader;
#[doc = "Field `DSI_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type DsiCollisionMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type InjEocMaskR = crate::BitReader;
#[doc = "Field `INJ_EOC_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type InjEocMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type InjSaturateMaskR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type InjSaturateMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type InjRangeMaskR = crate::BitReader;
#[doc = "Field `INJ_RANGE_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type InjRangeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_MASK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type InjCollisionMaskR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_MASK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type InjCollisionMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_mask(&self) -> EosMaskR {
        EosMaskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_mask(&self) -> OverflowMaskR {
        OverflowMaskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_mask(&self) -> FwCollisionMaskR {
        FwCollisionMaskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_mask(&self) -> DsiCollisionMaskR {
        DsiCollisionMaskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_mask(&self) -> InjEocMaskR {
        InjEocMaskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_mask(&self) -> InjSaturateMaskR {
        InjSaturateMaskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_mask(&self) -> InjRangeMaskR {
        InjRangeMaskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_mask(&self) -> InjCollisionMaskR {
        InjCollisionMaskR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn eos_mask(&mut self) -> EosMaskW<IntrMaskSpec> {
        EosMaskW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mask(&mut self) -> OverflowMaskW<IntrMaskSpec> {
        OverflowMaskW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_mask(&mut self) -> FwCollisionMaskW<IntrMaskSpec> {
        FwCollisionMaskW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_mask(&mut self) -> DsiCollisionMaskW<IntrMaskSpec> {
        DsiCollisionMaskW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_mask(&mut self) -> InjEocMaskW<IntrMaskSpec> {
        InjEocMaskW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_mask(&mut self) -> InjSaturateMaskW<IntrMaskSpec> {
        InjSaturateMaskW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_mask(&mut self) -> InjRangeMaskW<IntrMaskSpec> {
        InjRangeMaskW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_mask(&mut self) -> InjCollisionMaskW<IntrMaskSpec> {
        InjCollisionMaskW::new(self, 7)
    }
}
#[doc = "Interrupt mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
