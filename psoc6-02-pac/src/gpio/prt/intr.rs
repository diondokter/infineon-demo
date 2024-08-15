#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `EDGE0` reader - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
pub type Edge0R = crate::BitReader;
#[doc = "Field `EDGE0` writer - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
pub type Edge0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE1` reader - Edge detect for IO pin 1"]
pub type Edge1R = crate::BitReader;
#[doc = "Field `EDGE1` writer - Edge detect for IO pin 1"]
pub type Edge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE2` reader - Edge detect for IO pin 2"]
pub type Edge2R = crate::BitReader;
#[doc = "Field `EDGE2` writer - Edge detect for IO pin 2"]
pub type Edge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE3` reader - Edge detect for IO pin 3"]
pub type Edge3R = crate::BitReader;
#[doc = "Field `EDGE3` writer - Edge detect for IO pin 3"]
pub type Edge3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE4` reader - Edge detect for IO pin 4"]
pub type Edge4R = crate::BitReader;
#[doc = "Field `EDGE4` writer - Edge detect for IO pin 4"]
pub type Edge4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE5` reader - Edge detect for IO pin 5"]
pub type Edge5R = crate::BitReader;
#[doc = "Field `EDGE5` writer - Edge detect for IO pin 5"]
pub type Edge5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE6` reader - Edge detect for IO pin 6"]
pub type Edge6R = crate::BitReader;
#[doc = "Field `EDGE6` writer - Edge detect for IO pin 6"]
pub type Edge6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE7` reader - Edge detect for IO pin 7"]
pub type Edge7R = crate::BitReader;
#[doc = "Field `EDGE7` writer - Edge detect for IO pin 7"]
pub type Edge7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT_EDGE` reader - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeR = crate::BitReader;
#[doc = "Field `FLT_EDGE` writer - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_IN0` reader - IO pin state for pin 0"]
pub type InIn0R = crate::BitReader;
#[doc = "Field `IN_IN1` reader - IO pin state for pin 1"]
pub type InIn1R = crate::BitReader;
#[doc = "Field `IN_IN2` reader - IO pin state for pin 2"]
pub type InIn2R = crate::BitReader;
#[doc = "Field `IN_IN3` reader - IO pin state for pin 3"]
pub type InIn3R = crate::BitReader;
#[doc = "Field `IN_IN4` reader - IO pin state for pin 4"]
pub type InIn4R = crate::BitReader;
#[doc = "Field `IN_IN5` reader - IO pin state for pin 5"]
pub type InIn5R = crate::BitReader;
#[doc = "Field `IN_IN6` reader - IO pin state for pin 6"]
pub type InIn6R = crate::BitReader;
#[doc = "Field `IN_IN7` reader - IO pin state for pin 7"]
pub type InIn7R = crate::BitReader;
#[doc = "Field `FLT_IN_IN` reader - Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
pub type FltInInR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    pub fn edge0(&self) -> Edge0R {
        Edge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge detect for IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> Edge1R {
        Edge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge detect for IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> Edge2R {
        Edge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge detect for IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> Edge3R {
        Edge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge detect for IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> Edge4R {
        Edge4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge detect for IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> Edge5R {
        Edge5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge detect for IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> Edge6R {
        Edge6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge detect for IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> Edge7R {
        Edge7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FltEdgeR {
        FltEdgeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - IO pin state for pin 0"]
    #[inline(always)]
    pub fn in_in0(&self) -> InIn0R {
        InIn0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - IO pin state for pin 1"]
    #[inline(always)]
    pub fn in_in1(&self) -> InIn1R {
        InIn1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - IO pin state for pin 2"]
    #[inline(always)]
    pub fn in_in2(&self) -> InIn2R {
        InIn2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IO pin state for pin 3"]
    #[inline(always)]
    pub fn in_in3(&self) -> InIn3R {
        InIn3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IO pin state for pin 4"]
    #[inline(always)]
    pub fn in_in4(&self) -> InIn4R {
        InIn4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IO pin state for pin 5"]
    #[inline(always)]
    pub fn in_in5(&self) -> InIn5R {
        InIn5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - IO pin state for pin 6"]
    #[inline(always)]
    pub fn in_in6(&self) -> InIn6R {
        InIn6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - IO pin state for pin 7"]
    #[inline(always)]
    pub fn in_in7(&self) -> InIn7R {
        InIn7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Filtered pin state for pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_in_in(&self) -> FltInInR {
        FltInInR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Edge detect for IO pin 0 '0': No edge was detected on pin. '1': An edge was detected on pin."]
    #[inline(always)]
    #[must_use]
    pub fn edge0(&mut self) -> Edge0W<IntrSpec> {
        Edge0W::new(self, 0)
    }
    #[doc = "Bit 1 - Edge detect for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1(&mut self) -> Edge1W<IntrSpec> {
        Edge1W::new(self, 1)
    }
    #[doc = "Bit 2 - Edge detect for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> Edge2W<IntrSpec> {
        Edge2W::new(self, 2)
    }
    #[doc = "Bit 3 - Edge detect for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3(&mut self) -> Edge3W<IntrSpec> {
        Edge3W::new(self, 3)
    }
    #[doc = "Bit 4 - Edge detect for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4(&mut self) -> Edge4W<IntrSpec> {
        Edge4W::new(self, 4)
    }
    #[doc = "Bit 5 - Edge detect for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn edge5(&mut self) -> Edge5W<IntrSpec> {
        Edge5W::new(self, 5)
    }
    #[doc = "Bit 6 - Edge detect for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6(&mut self) -> Edge6W<IntrSpec> {
        Edge6W::new(self, 6)
    }
    #[doc = "Bit 7 - Edge detect for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn edge7(&mut self) -> Edge7W<IntrSpec> {
        Edge7W::new(self, 7)
    }
    #[doc = "Bit 8 - Edge detected on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn flt_edge(&mut self) -> FltEdgeW<IntrSpec> {
        FltEdgeW::new(self, 8)
    }
}
#[doc = "Port interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
