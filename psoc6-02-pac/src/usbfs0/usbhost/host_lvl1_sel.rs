#[doc = "Register `HOST_LVL1_SEL` reader"]
pub type R = crate::R<HostLvl1SelSpec>;
#[doc = "Register `HOST_LVL1_SEL` writer"]
pub type W = crate::W<HostLvl1SelSpec>;
#[doc = "These bits assign SOFIRQ interrupt flag to selected interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SofirqSel {
    #[doc = "0: High priority interrupt"]
    Hi = 0,
    #[doc = "1: Medium priority interrupt"]
    Med = 1,
    #[doc = "2: Low priority interrupt"]
    Lo = 2,
    #[doc = "3: N/A"]
    Rsvd = 3,
}
impl From<SofirqSel> for u8 {
    #[inline(always)]
    fn from(variant: SofirqSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SofirqSel {
    type Ux = u8;
}
impl crate::IsEnum for SofirqSel {}
#[doc = "Field `SOFIRQ_SEL` reader - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
pub type SofirqSelR = crate::FieldReader<SofirqSel>;
impl SofirqSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SofirqSel {
        match self.bits {
            0 => SofirqSel::Hi,
            1 => SofirqSel::Med,
            2 => SofirqSel::Lo,
            3 => SofirqSel::Rsvd,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SofirqSel::Hi
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SofirqSel::Med
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SofirqSel::Lo
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SofirqSel::Rsvd
    }
}
#[doc = "Field `SOFIRQ_SEL` writer - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
pub type SofirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, SofirqSel, crate::Safe>;
impl<'a, REG> SofirqSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(SofirqSel::Hi)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(SofirqSel::Med)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(SofirqSel::Lo)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(SofirqSel::Rsvd)
    }
}
#[doc = "Field `DIRQ_SEL` reader - These bits assign DIRQ interrupt flag to selected interrupt signals."]
pub type DirqSelR = crate::FieldReader;
#[doc = "Field `DIRQ_SEL` writer - These bits assign DIRQ interrupt flag to selected interrupt signals."]
pub type DirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNNIRQ_SEL` reader - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
pub type CnnirqSelR = crate::FieldReader;
#[doc = "Field `CNNIRQ_SEL` writer - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
pub type CnnirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMPIRQ_SEL` reader - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub type CmpirqSelR = crate::FieldReader;
#[doc = "Field `CMPIRQ_SEL` writer - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub type CmpirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `URIRQ_SEL` reader - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub type UrirqSelR = crate::FieldReader;
#[doc = "Field `URIRQ_SEL` writer - These bits assign URIRQ interrupt flag to selected interrupt signals."]
pub type UrirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RWKIRQ_SEL` reader - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
pub type RwkirqSelR = crate::FieldReader;
#[doc = "Field `RWKIRQ_SEL` writer - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
pub type RwkirqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD_13_12` reader - N/A"]
pub type Rsvd13_12R = crate::FieldReader;
#[doc = "Field `RSVD_13_12` writer - N/A"]
pub type Rsvd13_12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCAN_SEL` reader - These bits assign TCAN interrupt flag to selected interrupt signals."]
pub type TcanSelR = crate::FieldReader;
#[doc = "Field `TCAN_SEL` writer - These bits assign TCAN interrupt flag to selected interrupt signals."]
pub type TcanSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn sofirq_sel(&self) -> SofirqSelR {
        SofirqSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn dirq_sel(&self) -> DirqSelR {
        DirqSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cnnirq_sel(&self) -> CnnirqSelR {
        CnnirqSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn cmpirq_sel(&self) -> CmpirqSelR {
        CmpirqSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn urirq_sel(&self) -> UrirqSelR {
        UrirqSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn rwkirq_sel(&self) -> RwkirqSelR {
        RwkirqSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn rsvd_13_12(&self) -> Rsvd13_12R {
        Rsvd13_12R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn tcan_sel(&self) -> TcanSelR {
        TcanSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - These bits assign SOFIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn sofirq_sel(&mut self) -> SofirqSelW<HostLvl1SelSpec> {
        SofirqSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - These bits assign DIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn dirq_sel(&mut self) -> DirqSelW<HostLvl1SelSpec> {
        DirqSelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - These bits assign CNNIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn cnnirq_sel(&mut self) -> CnnirqSelW<HostLvl1SelSpec> {
        CnnirqSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn cmpirq_sel(&mut self) -> CmpirqSelW<HostLvl1SelSpec> {
        CmpirqSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - These bits assign URIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn urirq_sel(&mut self) -> UrirqSelW<HostLvl1SelSpec> {
        UrirqSelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - These bits assign RWKIRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn rwkirq_sel(&mut self) -> RwkirqSelW<HostLvl1SelSpec> {
        RwkirqSelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_13_12(&mut self) -> Rsvd13_12W<HostLvl1SelSpec> {
        Rsvd13_12W::new(self, 12)
    }
    #[doc = "Bits 14:15 - These bits assign TCAN interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn tcan_sel(&mut self) -> TcanSelW<HostLvl1SelSpec> {
        TcanSelW::new(self, 14)
    }
}
#[doc = "Host Interrupt Level 1 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_lvl1_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_lvl1_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostLvl1SelSpec;
impl crate::RegisterSpec for HostLvl1SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_lvl1_sel::R`](R) reader structure"]
impl crate::Readable for HostLvl1SelSpec {}
#[doc = "`write(|w| ..)` method takes [`host_lvl1_sel::W`](W) writer structure"]
impl crate::Writable for HostLvl1SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_LVL1_SEL to value 0"]
impl crate::Resettable for HostLvl1SelSpec {
    const RESET_VALUE: u32 = 0;
}
