#[doc = "Register `INTR_MASK` reader"]
pub type R = crate::R<IntrMaskSpec>;
#[doc = "Register `INTR_MASK` writer"]
pub type W = crate::W<IntrMaskSpec>;
#[doc = "Field `EDGE0` reader - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
pub type Edge0R = crate::BitReader;
#[doc = "Field `EDGE0` writer - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
pub type Edge0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE1` reader - Masks edge interrupt on IO pin 1"]
pub type Edge1R = crate::BitReader;
#[doc = "Field `EDGE1` writer - Masks edge interrupt on IO pin 1"]
pub type Edge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE2` reader - Masks edge interrupt on IO pin 2"]
pub type Edge2R = crate::BitReader;
#[doc = "Field `EDGE2` writer - Masks edge interrupt on IO pin 2"]
pub type Edge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE3` reader - Masks edge interrupt on IO pin 3"]
pub type Edge3R = crate::BitReader;
#[doc = "Field `EDGE3` writer - Masks edge interrupt on IO pin 3"]
pub type Edge3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE4` reader - Masks edge interrupt on IO pin 4"]
pub type Edge4R = crate::BitReader;
#[doc = "Field `EDGE4` writer - Masks edge interrupt on IO pin 4"]
pub type Edge4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE5` reader - Masks edge interrupt on IO pin 5"]
pub type Edge5R = crate::BitReader;
#[doc = "Field `EDGE5` writer - Masks edge interrupt on IO pin 5"]
pub type Edge5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE6` reader - Masks edge interrupt on IO pin 6"]
pub type Edge6R = crate::BitReader;
#[doc = "Field `EDGE6` writer - Masks edge interrupt on IO pin 6"]
pub type Edge6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE7` reader - Masks edge interrupt on IO pin 7"]
pub type Edge7R = crate::BitReader;
#[doc = "Field `EDGE7` writer - Masks edge interrupt on IO pin 7"]
pub type Edge7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT_EDGE` reader - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeR = crate::BitReader;
#[doc = "Field `FLT_EDGE` writer - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    pub fn edge0(&self) -> Edge0R {
        Edge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> Edge1R {
        Edge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> Edge2R {
        Edge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> Edge3R {
        Edge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> Edge4R {
        Edge4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> Edge5R {
        Edge5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> Edge6R {
        Edge6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> Edge7R {
        Edge7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FltEdgeR {
        FltEdgeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masks edge interrupt on IO pin 0 '0': Pin interrupt forwarding disabled '1': Pin interrupt forwarding enabled"]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> Edge0W<IntrMaskSpec> {
        Edge0W::new(self, 0)
    }
    #[doc = "Bit 1 - Masks edge interrupt on IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1(&mut self) -> Edge1W<IntrMaskSpec> {
        Edge1W::new(self, 1)
    }
    #[doc = "Bit 2 - Masks edge interrupt on IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> Edge2W<IntrMaskSpec> {
        Edge2W::new(self, 2)
    }
    #[doc = "Bit 3 - Masks edge interrupt on IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3(&mut self) -> Edge3W<IntrMaskSpec> {
        Edge3W::new(self, 3)
    }
    #[doc = "Bit 4 - Masks edge interrupt on IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4(&mut self) -> Edge4W<IntrMaskSpec> {
        Edge4W::new(self, 4)
    }
    #[doc = "Bit 5 - Masks edge interrupt on IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn edge5(&mut self) -> Edge5W<IntrMaskSpec> {
        Edge5W::new(self, 5)
    }
    #[doc = "Bit 6 - Masks edge interrupt on IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6(&mut self) -> Edge6W<IntrMaskSpec> {
        Edge6W::new(self, 6)
    }
    #[doc = "Bit 7 - Masks edge interrupt on IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn edge7(&mut self) -> Edge7W<IntrMaskSpec> {
        Edge7W::new(self, 7)
    }
    #[doc = "Bit 8 - Masks edge interrupt on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn flt_edge(&mut self) -> FltEdgeW<IntrMaskSpec> {
        FltEdgeW::new(self, 8)
    }
}
#[doc = "Port interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
