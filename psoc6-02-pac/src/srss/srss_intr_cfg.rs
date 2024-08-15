#[doc = "Register `SRSS_INTR_CFG` reader"]
pub type R = crate::R<SrssIntrCfgSpec>;
#[doc = "Register `SRSS_INTR_CFG` writer"]
pub type W = crate::W<SrssIntrCfgSpec>;
#[doc = "Sets which edge(s) will trigger an IRQ for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hvlvd1EdgeSel {
    #[doc = "0: Disabled"]
    Disable = 0,
    #[doc = "1: Rising edge"]
    Rising = 1,
    #[doc = "2: Falling edge"]
    Falling = 2,
    #[doc = "3: Both rising and falling edges"]
    Both = 3,
}
impl From<Hvlvd1EdgeSel> for u8 {
    #[inline(always)]
    fn from(variant: Hvlvd1EdgeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hvlvd1EdgeSel {
    type Ux = u8;
}
impl crate::IsEnum for Hvlvd1EdgeSel {}
#[doc = "Field `HVLVD1_EDGE_SEL` reader - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type Hvlvd1EdgeSelR = crate::FieldReader<Hvlvd1EdgeSel>;
impl Hvlvd1EdgeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hvlvd1EdgeSel {
        match self.bits {
            0 => Hvlvd1EdgeSel::Disable,
            1 => Hvlvd1EdgeSel::Rising,
            2 => Hvlvd1EdgeSel::Falling,
            3 => Hvlvd1EdgeSel::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hvlvd1EdgeSel::Disable
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Hvlvd1EdgeSel::Rising
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Hvlvd1EdgeSel::Falling
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Hvlvd1EdgeSel::Both
    }
}
#[doc = "Field `HVLVD1_EDGE_SEL` writer - Sets which edge(s) will trigger an IRQ for HVLVD1"]
pub type Hvlvd1EdgeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hvlvd1EdgeSel, crate::Safe>;
impl<'a, REG> Hvlvd1EdgeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1EdgeSel::Disable)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1EdgeSel::Rising)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1EdgeSel::Falling)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1EdgeSel::Both)
    }
}
impl R {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_edge_sel(&self) -> Hvlvd1EdgeSelR {
        Hvlvd1EdgeSelR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets which edge(s) will trigger an IRQ for HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_edge_sel(&mut self) -> Hvlvd1EdgeSelW<SrssIntrCfgSpec> {
        Hvlvd1EdgeSelW::new(self, 0)
    }
}
#[doc = "SRSS Interrupt Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srss_intr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssIntrCfgSpec;
impl crate::RegisterSpec for SrssIntrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_cfg::R`](R) reader structure"]
impl crate::Readable for SrssIntrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`srss_intr_cfg::W`](W) writer structure"]
impl crate::Writable for SrssIntrCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRSS_INTR_CFG to value 0"]
impl crate::Resettable for SrssIntrCfgSpec {
    const RESET_VALUE: u32 = 0;
}
