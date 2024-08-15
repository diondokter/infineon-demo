#[doc = "Register `INTR_LVL_SEL` reader"]
pub type R = crate::R<IntrLvlSelSpec>;
#[doc = "Register `INTR_LVL_SEL` writer"]
pub type W = crate::W<IntrLvlSelSpec>;
#[doc = "USB SOF Interrupt level select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SofLvlSel {
    #[doc = "0: High priority interrupt"]
    Hi = 0,
    #[doc = "1: Medium priority interrupt"]
    Med = 1,
    #[doc = "2: Low priority interrupt"]
    Lo = 2,
    #[doc = "3: illegal"]
    Rsvd = 3,
}
impl From<SofLvlSel> for u8 {
    #[inline(always)]
    fn from(variant: SofLvlSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SofLvlSel {
    type Ux = u8;
}
impl crate::IsEnum for SofLvlSel {}
#[doc = "Field `SOF_LVL_SEL` reader - USB SOF Interrupt level select"]
pub type SofLvlSelR = crate::FieldReader<SofLvlSel>;
impl SofLvlSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SofLvlSel {
        match self.bits {
            0 => SofLvlSel::Hi,
            1 => SofLvlSel::Med,
            2 => SofLvlSel::Lo,
            3 => SofLvlSel::Rsvd,
            _ => unreachable!(),
        }
    }
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == SofLvlSel::Hi
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == SofLvlSel::Med
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn is_lo(&self) -> bool {
        *self == SofLvlSel::Lo
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == SofLvlSel::Rsvd
    }
}
#[doc = "Field `SOF_LVL_SEL` writer - USB SOF Interrupt level select"]
pub type SofLvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, SofLvlSel, crate::Safe>;
impl<'a, REG> SofLvlSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High priority interrupt"]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(SofLvlSel::Hi)
    }
    #[doc = "Medium priority interrupt"]
    #[inline(always)]
    pub fn med(self) -> &'a mut crate::W<REG> {
        self.variant(SofLvlSel::Med)
    }
    #[doc = "Low priority interrupt"]
    #[inline(always)]
    pub fn lo(self) -> &'a mut crate::W<REG> {
        self.variant(SofLvlSel::Lo)
    }
    #[doc = "illegal"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(SofLvlSel::Rsvd)
    }
}
#[doc = "Field `BUS_RESET_LVL_SEL` reader - BUS RESET Interrupt level select"]
pub type BusResetLvlSelR = crate::FieldReader;
#[doc = "Field `BUS_RESET_LVL_SEL` writer - BUS RESET Interrupt level select"]
pub type BusResetLvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP0_LVL_SEL` reader - EP0 Interrupt level select"]
pub type Ep0LvlSelR = crate::FieldReader;
#[doc = "Field `EP0_LVL_SEL` writer - EP0 Interrupt level select"]
pub type Ep0LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPM_LVL_SEL` reader - LPM Interrupt level select"]
pub type LpmLvlSelR = crate::FieldReader;
#[doc = "Field `LPM_LVL_SEL` writer - LPM Interrupt level select"]
pub type LpmLvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESUME_LVL_SEL` reader - Resume Interrupt level select"]
pub type ResumeLvlSelR = crate::FieldReader;
#[doc = "Field `RESUME_LVL_SEL` writer - Resume Interrupt level select"]
pub type ResumeLvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARB_EP_LVL_SEL` reader - Arbiter Endpoint Interrupt level select"]
pub type ArbEpLvlSelR = crate::FieldReader;
#[doc = "Field `ARB_EP_LVL_SEL` writer - Arbiter Endpoint Interrupt level select"]
pub type ArbEpLvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP1_LVL_SEL` reader - EP1 Interrupt level select"]
pub type Ep1LvlSelR = crate::FieldReader;
#[doc = "Field `EP1_LVL_SEL` writer - EP1 Interrupt level select"]
pub type Ep1LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP2_LVL_SEL` reader - EP2 Interrupt level select"]
pub type Ep2LvlSelR = crate::FieldReader;
#[doc = "Field `EP2_LVL_SEL` writer - EP2 Interrupt level select"]
pub type Ep2LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP3_LVL_SEL` reader - EP3 Interrupt level select"]
pub type Ep3LvlSelR = crate::FieldReader;
#[doc = "Field `EP3_LVL_SEL` writer - EP3 Interrupt level select"]
pub type Ep3LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP4_LVL_SEL` reader - EP4 Interrupt level select"]
pub type Ep4LvlSelR = crate::FieldReader;
#[doc = "Field `EP4_LVL_SEL` writer - EP4 Interrupt level select"]
pub type Ep4LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP5_LVL_SEL` reader - EP5 Interrupt level select"]
pub type Ep5LvlSelR = crate::FieldReader;
#[doc = "Field `EP5_LVL_SEL` writer - EP5 Interrupt level select"]
pub type Ep5LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP6_LVL_SEL` reader - EP6 Interrupt level select"]
pub type Ep6LvlSelR = crate::FieldReader;
#[doc = "Field `EP6_LVL_SEL` writer - EP6 Interrupt level select"]
pub type Ep6LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP7_LVL_SEL` reader - EP7 Interrupt level select"]
pub type Ep7LvlSelR = crate::FieldReader;
#[doc = "Field `EP7_LVL_SEL` writer - EP7 Interrupt level select"]
pub type Ep7LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP8_LVL_SEL` reader - EP8 Interrupt level select"]
pub type Ep8LvlSelR = crate::FieldReader;
#[doc = "Field `EP8_LVL_SEL` writer - EP8 Interrupt level select"]
pub type Ep8LvlSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    pub fn sof_lvl_sel(&self) -> SofLvlSelR {
        SofLvlSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    pub fn bus_reset_lvl_sel(&self) -> BusResetLvlSelR {
        BusResetLvlSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    pub fn ep0_lvl_sel(&self) -> Ep0LvlSelR {
        Ep0LvlSelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    pub fn lpm_lvl_sel(&self) -> LpmLvlSelR {
        LpmLvlSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    pub fn resume_lvl_sel(&self) -> ResumeLvlSelR {
        ResumeLvlSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    pub fn arb_ep_lvl_sel(&self) -> ArbEpLvlSelR {
        ArbEpLvlSelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    pub fn ep1_lvl_sel(&self) -> Ep1LvlSelR {
        Ep1LvlSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    pub fn ep2_lvl_sel(&self) -> Ep2LvlSelR {
        Ep2LvlSelR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    pub fn ep3_lvl_sel(&self) -> Ep3LvlSelR {
        Ep3LvlSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    pub fn ep4_lvl_sel(&self) -> Ep4LvlSelR {
        Ep4LvlSelR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    pub fn ep5_lvl_sel(&self) -> Ep5LvlSelR {
        Ep5LvlSelR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    pub fn ep6_lvl_sel(&self) -> Ep6LvlSelR {
        Ep6LvlSelR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    pub fn ep7_lvl_sel(&self) -> Ep7LvlSelR {
        Ep7LvlSelR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    pub fn ep8_lvl_sel(&self) -> Ep8LvlSelR {
        Ep8LvlSelR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB SOF Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn sof_lvl_sel(&mut self) -> SofLvlSelW<IntrLvlSelSpec> {
        SofLvlSelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - BUS RESET Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn bus_reset_lvl_sel(&mut self) -> BusResetLvlSelW<IntrLvlSelSpec> {
        BusResetLvlSelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - EP0 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep0_lvl_sel(&mut self) -> Ep0LvlSelW<IntrLvlSelSpec> {
        Ep0LvlSelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - LPM Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_lvl_sel(&mut self) -> LpmLvlSelW<IntrLvlSelSpec> {
        LpmLvlSelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Resume Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn resume_lvl_sel(&mut self) -> ResumeLvlSelW<IntrLvlSelSpec> {
        ResumeLvlSelW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Arbiter Endpoint Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn arb_ep_lvl_sel(&mut self) -> ArbEpLvlSelW<IntrLvlSelSpec> {
        ArbEpLvlSelW::new(self, 14)
    }
    #[doc = "Bits 16:17 - EP1 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep1_lvl_sel(&mut self) -> Ep1LvlSelW<IntrLvlSelSpec> {
        Ep1LvlSelW::new(self, 16)
    }
    #[doc = "Bits 18:19 - EP2 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep2_lvl_sel(&mut self) -> Ep2LvlSelW<IntrLvlSelSpec> {
        Ep2LvlSelW::new(self, 18)
    }
    #[doc = "Bits 20:21 - EP3 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep3_lvl_sel(&mut self) -> Ep3LvlSelW<IntrLvlSelSpec> {
        Ep3LvlSelW::new(self, 20)
    }
    #[doc = "Bits 22:23 - EP4 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep4_lvl_sel(&mut self) -> Ep4LvlSelW<IntrLvlSelSpec> {
        Ep4LvlSelW::new(self, 22)
    }
    #[doc = "Bits 24:25 - EP5 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep5_lvl_sel(&mut self) -> Ep5LvlSelW<IntrLvlSelSpec> {
        Ep5LvlSelW::new(self, 24)
    }
    #[doc = "Bits 26:27 - EP6 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep6_lvl_sel(&mut self) -> Ep6LvlSelW<IntrLvlSelSpec> {
        Ep6LvlSelW::new(self, 26)
    }
    #[doc = "Bits 28:29 - EP7 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep7_lvl_sel(&mut self) -> Ep7LvlSelW<IntrLvlSelSpec> {
        Ep7LvlSelW::new(self, 28)
    }
    #[doc = "Bits 30:31 - EP8 Interrupt level select"]
    #[inline(always)]
    #[must_use]
    pub fn ep8_lvl_sel(&mut self) -> Ep8LvlSelW<IntrLvlSelSpec> {
        Ep8LvlSelW::new(self, 30)
    }
}
#[doc = "Select interrupt level for each interrupt source\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_lvl_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_lvl_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrLvlSelSpec;
impl crate::RegisterSpec for IntrLvlSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_lvl_sel::R`](R) reader structure"]
impl crate::Readable for IntrLvlSelSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_lvl_sel::W`](W) writer structure"]
impl crate::Writable for IntrLvlSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_LVL_SEL to value 0"]
impl crate::Resettable for IntrLvlSelSpec {
    const RESET_VALUE: u32 = 0;
}
