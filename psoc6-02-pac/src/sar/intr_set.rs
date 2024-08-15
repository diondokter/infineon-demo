#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `EOS_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type EosSetR = crate::BitReader;
#[doc = "Field `EOS_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type EosSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type OverflowSetR = crate::BitReader;
#[doc = "Field `OVERFLOW_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type OverflowSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type FwCollisionSetR = crate::BitReader;
#[doc = "Field `FW_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type FwCollisionSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type DsiCollisionSetR = crate::BitReader;
#[doc = "Field `DSI_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type DsiCollisionSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjEocSetR = crate::BitReader;
#[doc = "Field `INJ_EOC_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjEocSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjSaturateSetR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjSaturateSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjRangeSetR = crate::BitReader;
#[doc = "Field `INJ_RANGE_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjRangeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_SET` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjCollisionSetR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_SET` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type InjCollisionSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn eos_set(&self) -> EosSetR {
        EosSetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn overflow_set(&self) -> OverflowSetR {
        OverflowSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn fw_collision_set(&self) -> FwCollisionSetR {
        FwCollisionSetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn dsi_collision_set(&self) -> DsiCollisionSetR {
        DsiCollisionSetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_eoc_set(&self) -> InjEocSetR {
        InjEocSetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_saturate_set(&self) -> InjSaturateSetR {
        InjSaturateSetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_range_set(&self) -> InjRangeSetR {
        InjRangeSetR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn inj_collision_set(&self) -> InjCollisionSetR {
        InjCollisionSetR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn eos_set(&mut self) -> EosSetW<IntrSetSpec> {
        EosSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_set(&mut self) -> OverflowSetW<IntrSetSpec> {
        OverflowSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_set(&mut self) -> FwCollisionSetW<IntrSetSpec> {
        FwCollisionSetW::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_set(&mut self) -> DsiCollisionSetW<IntrSetSpec> {
        DsiCollisionSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_set(&mut self) -> InjEocSetW<IntrSetSpec> {
        InjEocSetW::new(self, 4)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_set(&mut self) -> InjSaturateSetW<IntrSetSpec> {
        InjSaturateSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_set(&mut self) -> InjRangeSetW<IntrSetSpec> {
        InjRangeSetW::new(self, 6)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_set(&mut self) -> InjCollisionSetW<IntrSetSpec> {
        InjCollisionSetW::new(self, 7)
    }
}
#[doc = "Interrupt set request register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
