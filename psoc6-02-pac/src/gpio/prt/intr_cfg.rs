#[doc = "Register `INTR_CFG` reader"]
pub type R = crate::R<IntrCfgSpec>;
#[doc = "Register `INTR_CFG` writer"]
pub type W = crate::W<IntrCfgSpec>;
#[doc = "Sets which edge will trigger an IRQ for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Edge0Sel {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both rising and falling edges"]
    Both = 3,
}
impl From<Edge0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Edge0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Edge0Sel {
    type Ux = u8;
}
impl crate::IsEnum for Edge0Sel {}
#[doc = "Field `EDGE0_SEL` reader - Sets which edge will trigger an IRQ for IO pin 0"]
pub type Edge0SelR = crate::FieldReader<Edge0Sel>;
impl Edge0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge0Sel {
        match self.bits {
            0 => Edge0Sel::Disable,
            1 => Edge0Sel::Rising,
            2 => Edge0Sel::Falling,
            3 => Edge0Sel::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Edge0Sel::Disable
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edge0Sel::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Edge0Sel::Falling
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Edge0Sel::Both
    }
}
#[doc = "Field `EDGE0_SEL` writer - Sets which edge will trigger an IRQ for IO pin 0"]
pub type Edge0SelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Edge0Sel, crate::Safe>;
impl<'a, REG> Edge0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Edge0Sel::Disable)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edge0Sel::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Edge0Sel::Falling)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Edge0Sel::Both)
    }
}
#[doc = "Field `EDGE1_SEL` reader - Sets which edge will trigger an IRQ for IO pin 1"]
pub type Edge1SelR = crate::FieldReader;
#[doc = "Field `EDGE1_SEL` writer - Sets which edge will trigger an IRQ for IO pin 1"]
pub type Edge1SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE2_SEL` reader - Sets which edge will trigger an IRQ for IO pin 2"]
pub type Edge2SelR = crate::FieldReader;
#[doc = "Field `EDGE2_SEL` writer - Sets which edge will trigger an IRQ for IO pin 2"]
pub type Edge2SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE3_SEL` reader - Sets which edge will trigger an IRQ for IO pin 3"]
pub type Edge3SelR = crate::FieldReader;
#[doc = "Field `EDGE3_SEL` writer - Sets which edge will trigger an IRQ for IO pin 3"]
pub type Edge3SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE4_SEL` reader - Sets which edge will trigger an IRQ for IO pin 4"]
pub type Edge4SelR = crate::FieldReader;
#[doc = "Field `EDGE4_SEL` writer - Sets which edge will trigger an IRQ for IO pin 4"]
pub type Edge4SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE5_SEL` reader - Sets which edge will trigger an IRQ for IO pin 5"]
pub type Edge5SelR = crate::FieldReader;
#[doc = "Field `EDGE5_SEL` writer - Sets which edge will trigger an IRQ for IO pin 5"]
pub type Edge5SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE6_SEL` reader - Sets which edge will trigger an IRQ for IO pin 6"]
pub type Edge6SelR = crate::FieldReader;
#[doc = "Field `EDGE6_SEL` writer - Sets which edge will trigger an IRQ for IO pin 6"]
pub type Edge6SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EDGE7_SEL` reader - Sets which edge will trigger an IRQ for IO pin 7"]
pub type Edge7SelR = crate::FieldReader;
#[doc = "Field `EDGE7_SEL` writer - Sets which edge will trigger an IRQ for IO pin 7"]
pub type Edge7SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FltEdgeSel {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both rising and falling edges"]
    Both = 3,
}
impl From<FltEdgeSel> for u8 {
    #[inline(always)]
    fn from(variant: FltEdgeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FltEdgeSel {
    type Ux = u8;
}
impl crate::IsEnum for FltEdgeSel {}
#[doc = "Field `FLT_EDGE_SEL` reader - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeSelR = crate::FieldReader<FltEdgeSel>;
impl FltEdgeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FltEdgeSel {
        match self.bits {
            0 => FltEdgeSel::Disable,
            1 => FltEdgeSel::Rising,
            2 => FltEdgeSel::Falling,
            3 => FltEdgeSel::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FltEdgeSel::Disable
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == FltEdgeSel::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == FltEdgeSel::Falling
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == FltEdgeSel::Both
    }
}
#[doc = "Field `FLT_EDGE_SEL` writer - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
pub type FltEdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, FltEdgeSel, crate::Safe>;
impl<'a, REG> FltEdgeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FltEdgeSel::Disable)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(FltEdgeSel::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(FltEdgeSel::Falling)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(FltEdgeSel::Both)
    }
}
#[doc = "Field `FLT_SEL` reader - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FltSelR = crate::FieldReader;
#[doc = "Field `FLT_SEL` writer - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
pub type FltSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    pub fn edge0_sel(&self) -> Edge0SelR {
        Edge0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    pub fn edge1_sel(&self) -> Edge1SelR {
        Edge1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    pub fn edge2_sel(&self) -> Edge2SelR {
        Edge2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    pub fn edge3_sel(&self) -> Edge3SelR {
        Edge3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    pub fn edge4_sel(&self) -> Edge4SelR {
        Edge4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    pub fn edge5_sel(&self) -> Edge5SelR {
        Edge5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    pub fn edge6_sel(&self) -> Edge6SelR {
        Edge6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    pub fn edge7_sel(&self) -> Edge7SelR {
        Edge7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    pub fn flt_edge_sel(&self) -> FltEdgeSelR {
        FltEdgeSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    pub fn flt_sel(&self) -> FltSelR {
        FltSelR::new(((self.bits >> 18) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge will trigger an IRQ for IO pin 0"]
    #[inline(always)]
    #[must_use]
    pub fn edge0_sel(&mut self) -> Edge0SelW<IntrCfgSpec> {
        Edge0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Sets which edge will trigger an IRQ for IO pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn edge1_sel(&mut self) -> Edge1SelW<IntrCfgSpec> {
        Edge1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Sets which edge will trigger an IRQ for IO pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2_sel(&mut self) -> Edge2SelW<IntrCfgSpec> {
        Edge2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ for IO pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3_sel(&mut self) -> Edge3SelW<IntrCfgSpec> {
        Edge3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Sets which edge will trigger an IRQ for IO pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn edge4_sel(&mut self) -> Edge4SelW<IntrCfgSpec> {
        Edge4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Sets which edge will trigger an IRQ for IO pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn edge5_sel(&mut self) -> Edge5SelW<IntrCfgSpec> {
        Edge5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Sets which edge will trigger an IRQ for IO pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn edge6_sel(&mut self) -> Edge6SelW<IntrCfgSpec> {
        Edge6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Sets which edge will trigger an IRQ for IO pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn edge7_sel(&mut self) -> Edge7SelW<IntrCfgSpec> {
        Edge7SelW::new(self, 14)
    }
    #[doc = "Bits 16:17 - Sets which edge will trigger an IRQ for the glitch filtered pin (selected by INTR_CFG.FLT_SEL"]
    #[inline(always)]
    #[must_use]
    pub fn flt_edge_sel(&mut self) -> FltEdgeSelW<IntrCfgSpec> {
        FltEdgeSelW::new(self, 16)
    }
    #[doc = "Bits 18:20 - Selects which pin is routed through the 50ns glitch filter to provide a glitch-safe interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn flt_sel(&mut self) -> FltSelW<IntrCfgSpec> {
        FltSelW::new(self, 18)
    }
}
#[doc = "Port interrupt configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCfgSpec;
impl crate::RegisterSpec for IntrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cfg::R`](R) reader structure"]
impl crate::Readable for IntrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_cfg::W`](W) writer structure"]
impl crate::Writable for IntrCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_CFG to value 0"]
impl crate::Resettable for IntrCfgSpec {
    const RESET_VALUE: u32 = 0;
}
