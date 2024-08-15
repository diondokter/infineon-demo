#[doc = "Register `HOST_LVL2_SEL` reader"]
pub type R = crate::R<HostLvl2SelSpec>;
#[doc = "Register `HOST_LVL2_SEL` writer"]
pub type W = crate::W<HostLvl2SelSpec>;
#[doc = "These bits assign EP1_DRQ interrupt flag to selected interrupt signals.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ep1DrqSel {
    #[doc = "0: High priority interrupt"]
    Hi = 0,
    #[doc = "1: Medium priority interrupt"]
    Med = 1,
    #[doc = "2: Low priority interrupt"]
    Lo = 2,
    #[doc = "3: N/A"]
    Rsvd = 3,
}
impl From<Ep1DrqSel> for u8 {
    #[inline(always)]
    fn from(variant: Ep1DrqSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ep1DrqSel {
    type Ux = u8;
}
impl crate::IsEnum for Ep1DrqSel {}
#[doc = "Field `EP1_DRQ_SEL` reader - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
pub type Ep1DrqSelR = crate::FieldReader<Ep1DrqSel>;
impl Ep1DrqSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1DrqSel {
        match self.bits {
            0 => Ep1DrqSel::Hi,
            1 => Ep1DrqSel::Med,
            2 => Ep1DrqSel::Lo,
            3 => Ep1DrqSel::Rsvd,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == Ep1DrqSel::Hi
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == Ep1DrqSel::Med
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == Ep1DrqSel::Lo
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == Ep1DrqSel::Rsvd
    }
}
#[doc = "Field `EP1_DRQ_SEL` writer - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
pub type Ep1DrqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ep1DrqSel, crate::Safe>;
impl<'a, REG> Ep1DrqSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1DrqSel::Hi)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1DrqSel::Med)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1DrqSel::Lo)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1DrqSel::Rsvd)
    }
}
#[doc = "Field `EP1_SPK_SEL` reader - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
pub type Ep1SpkSelR = crate::FieldReader;
#[doc = "Field `EP1_SPK_SEL` writer - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
pub type Ep1SpkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_DRQ_SEL` reader - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
pub type Ep2DrqSelR = crate::FieldReader;
#[doc = "Field `EP2_DRQ_SEL` writer - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
pub type Ep2DrqSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_SPK_SEL` reader - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
pub type Ep2SpkSelR = crate::FieldReader;
#[doc = "Field `EP2_SPK_SEL` writer - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
pub type Ep2SpkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_drq_sel(&self) -> Ep1DrqSelR {
        Ep1DrqSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep1_spk_sel(&self) -> Ep1SpkSelR {
        Ep1SpkSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_drq_sel(&self) -> Ep2DrqSelR {
        Ep2DrqSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    pub fn ep2_spk_sel(&self) -> Ep2SpkSelR {
        Ep2SpkSelR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - These bits assign EP1_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_drq_sel(&mut self) -> Ep1DrqSelW<HostLvl2SelSpec> {
        Ep1DrqSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - These bits assign EP1_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_spk_sel(&mut self) -> Ep1SpkSelW<HostLvl2SelSpec> {
        Ep1SpkSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - These bits assign EP2_DRQ interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_drq_sel(&mut self) -> Ep2DrqSelW<HostLvl2SelSpec> {
        Ep2DrqSelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - These bits assign EP2_SPK interrupt flag to selected interrupt signals."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_spk_sel(&mut self) -> Ep2SpkSelW<HostLvl2SelSpec> {
        Ep2SpkSelW::new(self, 10)
    }
}
#[doc = "Host Interrupt Level 2 Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_lvl2_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_lvl2_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostLvl2SelSpec;
impl crate::RegisterSpec for HostLvl2SelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_lvl2_sel::R`](R) reader structure"]
impl crate::Readable for HostLvl2SelSpec {}
#[doc = "`write(|w| ..)` method takes [`host_lvl2_sel::W`](W) writer structure"]
impl crate::Writable for HostLvl2SelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_LVL2_SEL to value 0"]
impl crate::Resettable for HostLvl2SelSpec {
    const RESET_VALUE: u32 = 0;
}
