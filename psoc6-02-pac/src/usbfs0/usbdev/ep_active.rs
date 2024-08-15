#[doc = "Register `EP_ACTIVE` reader"]
pub type R = crate::R<EpActiveSpec>;
#[doc = "Register `EP_ACTIVE` writer"]
pub type W = crate::W<EpActiveSpec>;
#[doc = "Field `EP1_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep1ActR = crate::BitReader;
#[doc = "Field `EP1_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep1ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep2ActR = crate::BitReader;
#[doc = "Field `EP2_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep2ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep3ActR = crate::BitReader;
#[doc = "Field `EP3_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep3ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep4ActR = crate::BitReader;
#[doc = "Field `EP4_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep4ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep5ActR = crate::BitReader;
#[doc = "Field `EP5_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep5ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep6ActR = crate::BitReader;
#[doc = "Field `EP6_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep6ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep7ActR = crate::BitReader;
#[doc = "Field `EP7_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep7ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP8_ACT` reader - Indicates that Endpoint is currently active."]
pub type Ep8ActR = crate::BitReader;
#[doc = "Field `EP8_ACT` writer - Indicates that Endpoint is currently active."]
pub type Ep8ActW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&self) -> Ep1ActR {
        Ep1ActR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&self) -> Ep2ActR {
        Ep2ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&self) -> Ep3ActR {
        Ep3ActR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&self) -> Ep4ActR {
        Ep4ActR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&self) -> Ep5ActR {
        Ep5ActR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&self) -> Ep6ActR {
        Ep6ActR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&self) -> Ep7ActR {
        Ep7ActR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&self) -> Ep8ActR {
        Ep8ActR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_act(&mut self) -> Ep1ActW<EpActiveSpec> {
        Ep1ActW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_act(&mut self) -> Ep2ActW<EpActiveSpec> {
        Ep2ActW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep3_act(&mut self) -> Ep3ActW<EpActiveSpec> {
        Ep3ActW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep4_act(&mut self) -> Ep4ActW<EpActiveSpec> {
        Ep4ActW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep5_act(&mut self) -> Ep5ActW<EpActiveSpec> {
        Ep5ActW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep6_act(&mut self) -> Ep6ActW<EpActiveSpec> {
        Ep6ActW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep7_act(&mut self) -> Ep7ActW<EpActiveSpec> {
        Ep7ActW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    #[must_use]
    pub fn ep8_act(&mut self) -> Ep8ActW<EpActiveSpec> {
        Ep8ActW::new(self, 7)
    }
}
#[doc = "Endpoint Active Indication Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_active::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_active::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpActiveSpec;
impl crate::RegisterSpec for EpActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_active::R`](R) reader structure"]
impl crate::Readable for EpActiveSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_active::W`](W) writer structure"]
impl crate::Writable for EpActiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP_ACTIVE to value 0"]
impl crate::Resettable for EpActiveSpec {
    const RESET_VALUE: u32 = 0;
}
