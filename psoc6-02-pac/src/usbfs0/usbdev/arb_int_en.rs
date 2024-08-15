#[doc = "Register `ARB_INT_EN` reader"]
pub type R = crate::R<ArbIntEnSpec>;
#[doc = "Register `ARB_INT_EN` writer"]
pub type W = crate::W<ArbIntEnSpec>;
#[doc = "Field `EP1_INTR_EN` reader - Enables interrupt for EP1"]
pub type Ep1IntrEnR = crate::BitReader;
#[doc = "Field `EP1_INTR_EN` writer - Enables interrupt for EP1"]
pub type Ep1IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_INTR_EN` reader - Enables interrupt for EP2"]
pub type Ep2IntrEnR = crate::BitReader;
#[doc = "Field `EP2_INTR_EN` writer - Enables interrupt for EP2"]
pub type Ep2IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_INTR_EN` reader - Enables interrupt for EP3"]
pub type Ep3IntrEnR = crate::BitReader;
#[doc = "Field `EP3_INTR_EN` writer - Enables interrupt for EP3"]
pub type Ep3IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_INTR_EN` reader - Enables interrupt for EP4"]
pub type Ep4IntrEnR = crate::BitReader;
#[doc = "Field `EP4_INTR_EN` writer - Enables interrupt for EP4"]
pub type Ep4IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_INTR_EN` reader - Enables interrupt for EP5"]
pub type Ep5IntrEnR = crate::BitReader;
#[doc = "Field `EP5_INTR_EN` writer - Enables interrupt for EP5"]
pub type Ep5IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_INTR_EN` reader - Enables interrupt for EP6"]
pub type Ep6IntrEnR = crate::BitReader;
#[doc = "Field `EP6_INTR_EN` writer - Enables interrupt for EP6"]
pub type Ep6IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_INTR_EN` reader - Enables interrupt for EP7"]
pub type Ep7IntrEnR = crate::BitReader;
#[doc = "Field `EP7_INTR_EN` writer - Enables interrupt for EP7"]
pub type Ep7IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_INTR_EN` reader - Enables interrupt for EP8"]
pub type Ep8IntrEnR = crate::BitReader;
#[doc = "Field `EP8_INTR_EN` writer - Enables interrupt for EP8"]
pub type Ep8IntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    pub fn ep1_intr_en(&self) -> Ep1IntrEnR {
        Ep1IntrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    pub fn ep2_intr_en(&self) -> Ep2IntrEnR {
        Ep2IntrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    pub fn ep3_intr_en(&self) -> Ep3IntrEnR {
        Ep3IntrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    pub fn ep4_intr_en(&self) -> Ep4IntrEnR {
        Ep4IntrEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    pub fn ep5_intr_en(&self) -> Ep5IntrEnR {
        Ep5IntrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    pub fn ep6_intr_en(&self) -> Ep6IntrEnR {
        Ep6IntrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    pub fn ep7_intr_en(&self) -> Ep7IntrEnR {
        Ep7IntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    pub fn ep8_intr_en(&self) -> Ep8IntrEnR {
        Ep8IntrEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables interrupt for EP1"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_intr_en(&mut self) -> Ep1IntrEnW<ArbIntEnSpec> {
        Ep1IntrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables interrupt for EP2"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_intr_en(&mut self) -> Ep2IntrEnW<ArbIntEnSpec> {
        Ep2IntrEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables interrupt for EP3"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_intr_en(&mut self) -> Ep3IntrEnW<ArbIntEnSpec> {
        Ep3IntrEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables interrupt for EP4"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_intr_en(&mut self) -> Ep4IntrEnW<ArbIntEnSpec> {
        Ep4IntrEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables interrupt for EP5"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_intr_en(&mut self) -> Ep5IntrEnW<ArbIntEnSpec> {
        Ep5IntrEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables interrupt for EP6"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_intr_en(&mut self) -> Ep6IntrEnW<ArbIntEnSpec> {
        Ep6IntrEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Enables interrupt for EP7"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_intr_en(&mut self) -> Ep7IntrEnW<ArbIntEnSpec> {
        Ep7IntrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Enables interrupt for EP8"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_intr_en(&mut self) -> Ep8IntrEnW<ArbIntEnSpec> {
        Ep8IntrEnW::new(self, 7)
    }
}
#[doc = "Arbiter Interrupt Enable *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbIntEnSpec;
impl crate::RegisterSpec for ArbIntEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_int_en::R`](R) reader structure"]
impl crate::Readable for ArbIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_int_en::W`](W) writer structure"]
impl crate::Writable for ArbIntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_INT_EN to value 0"]
impl crate::Resettable for ArbIntEnSpec {
    const RESET_VALUE: u32 = 0;
}
