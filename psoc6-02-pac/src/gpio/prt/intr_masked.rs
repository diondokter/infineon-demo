#[doc = "Register `INTR_MASKED` reader"]
pub type R = crate::R<IntrMaskedSpec>;
#[doc = "Field `EDGE0` reader - Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
pub type Edge0R = crate::BitReader;
#[doc = "Field `EDGE1` reader - Edge detected and masked on IO pin 1"]
pub type Edge1R = crate::BitReader;
#[doc = "Field `EDGE2` reader - Edge detected and masked on IO pin 2"]
pub type Edge2R = crate::BitReader;
#[doc = "Field `EDGE3` reader - Edge detected and masked on IO pin 3"]
pub type Edge3R = crate::BitReader;
#[doc = "Field `EDGE4` reader - Edge detected and masked on IO pin 4"]
pub type Edge4R = crate::BitReader;
#[doc = "Field `EDGE5` reader - Edge detected and masked on IO pin 5"]
pub type Edge5R = crate::BitReader;
#[doc = "Field `EDGE6` reader - Edge detected and masked on IO pin 6"]
pub type Edge6R = crate::BitReader;
#[doc = "Field `EDGE7` reader - Edge detected and masked on IO pin 7"]
pub type Edge7R = crate::BitReader;
#[doc = "Field `FLT_EDGE` reader - Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Edge detected AND masked on IO pin 0 '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
    #[inline(always)]
    pub fn edge0(&self) -> Edge0R {
        Edge0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge detected and masked on IO pin 1"]
    #[inline(always)]
    pub fn edge1(&self) -> Edge1R {
        Edge1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge detected and masked on IO pin 2"]
    #[inline(always)]
    pub fn edge2(&self) -> Edge2R {
        Edge2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge detected and masked on IO pin 3"]
    #[inline(always)]
    pub fn edge3(&self) -> Edge3R {
        Edge3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Edge detected and masked on IO pin 4"]
    #[inline(always)]
    pub fn edge4(&self) -> Edge4R {
        Edge4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Edge detected and masked on IO pin 5"]
    #[inline(always)]
    pub fn edge5(&self) -> Edge5R {
        Edge5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Edge detected and masked on IO pin 6"]
    #[inline(always)]
    pub fn edge6(&self) -> Edge6R {
        Edge6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Edge detected and masked on IO pin 7"]
    #[inline(always)]
    pub fn edge7(&self) -> Edge7R {
        Edge7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Edge detected and masked on filtered pin selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge(&self) -> FltEdgeR {
        FltEdgeR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Port interrupt masked status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
