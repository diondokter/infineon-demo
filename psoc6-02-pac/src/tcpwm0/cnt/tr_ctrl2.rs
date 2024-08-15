#[doc = "Register `TR_CTRL2` reader"]
pub type R = crate::R<TrCtrl2Spec>;
#[doc = "Register `TR_CTRL2` writer"]
pub type W = crate::W<TrCtrl2Spec>;
#[doc = "Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CcMatchMode {
    #[doc = "0: Set to '1'"]
    Set = 0,
    #[doc = "1: Set to '0'"]
    Clear = 1,
    #[doc = "2: Invert"]
    Invert = 2,
    #[doc = "3: No Change"]
    NoChange = 3,
}
impl From<CcMatchMode> for u8 {
    #[inline(always)]
    fn from(variant: CcMatchMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CcMatchMode {
    type Ux = u8;
}
impl crate::IsEnum for CcMatchMode {}
#[doc = "Field `CC_MATCH_MODE` reader - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub type CcMatchModeR = crate::FieldReader<CcMatchMode>;
impl CcMatchModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcMatchMode {
        match self.bits {
            0 => CcMatchMode::Set,
            1 => CcMatchMode::Clear,
            2 => CcMatchMode::Invert,
            3 => CcMatchMode::NoChange,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == CcMatchMode::Set
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CcMatchMode::Clear
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == CcMatchMode::Invert
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == CcMatchMode::NoChange
    }
}
#[doc = "Field `CC_MATCH_MODE` writer - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
pub type CcMatchModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CcMatchMode, crate::Safe>;
impl<'a, REG> CcMatchModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(CcMatchMode::Set)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CcMatchMode::Clear)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(CcMatchMode::Invert)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(CcMatchMode::NoChange)
    }
}
#[doc = "Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OverflowMode {
    #[doc = "0: Set to '1'"]
    Set = 0,
    #[doc = "1: Set to '0'"]
    Clear = 1,
    #[doc = "2: Invert"]
    Invert = 2,
    #[doc = "3: No Change"]
    NoChange = 3,
}
impl From<OverflowMode> for u8 {
    #[inline(always)]
    fn from(variant: OverflowMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OverflowMode {
    type Ux = u8;
}
impl crate::IsEnum for OverflowMode {}
#[doc = "Field `OVERFLOW_MODE` reader - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OverflowModeR = crate::FieldReader<OverflowMode>;
impl OverflowModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OverflowMode {
        match self.bits {
            0 => OverflowMode::Set,
            1 => OverflowMode::Clear,
            2 => OverflowMode::Invert,
            3 => OverflowMode::NoChange,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == OverflowMode::Set
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == OverflowMode::Clear
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == OverflowMode::Invert
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == OverflowMode::NoChange
    }
}
#[doc = "Field `OVERFLOW_MODE` writer - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
pub type OverflowModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, OverflowMode, crate::Safe>;
impl<'a, REG> OverflowModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowMode::Set)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowMode::Clear)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowMode::Invert)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(OverflowMode::NoChange)
    }
}
#[doc = "Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UnderflowMode {
    #[doc = "0: Set to '1'"]
    Set = 0,
    #[doc = "1: Set to '0'"]
    Clear = 1,
    #[doc = "2: Invert"]
    Invert = 2,
    #[doc = "3: No Change"]
    NoChange = 3,
}
impl From<UnderflowMode> for u8 {
    #[inline(always)]
    fn from(variant: UnderflowMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UnderflowMode {
    type Ux = u8;
}
impl crate::IsEnum for UnderflowMode {}
#[doc = "Field `UNDERFLOW_MODE` reader - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UnderflowModeR = crate::FieldReader<UnderflowMode>;
impl UnderflowModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UnderflowMode {
        match self.bits {
            0 => UnderflowMode::Set,
            1 => UnderflowMode::Clear,
            2 => UnderflowMode::Invert,
            3 => UnderflowMode::NoChange,
            _ => unreachable!(),
        }
    }
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == UnderflowMode::Set
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == UnderflowMode::Clear
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == UnderflowMode::Invert
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == UnderflowMode::NoChange
    }
}
#[doc = "Field `UNDERFLOW_MODE` writer - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
pub type UnderflowModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, UnderflowMode, crate::Safe>;
impl<'a, REG> UnderflowModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set to '1'"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(UnderflowMode::Set)
    }
    #[doc = "Set to '0'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UnderflowMode::Clear)
    }
    #[doc = "Invert"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut crate::W<REG> {
        self.variant(UnderflowMode::Invert)
    }
    #[doc = "No Change"]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(UnderflowMode::NoChange)
    }
}
impl R {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    pub fn cc_match_mode(&self) -> CcMatchModeR {
        CcMatchModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    pub fn overflow_mode(&self) -> OverflowModeR {
        OverflowModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    pub fn underflow_mode(&self) -> UnderflowModeR {
        UnderflowModeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Determines the effect of a compare match event (COUNTER equals CC register) on the 'line_out' output signals. Note that INVERT is especially useful for center aligned pulse width modulation. To generate a duty cycle of 0 percent, the counter CC register should be set to '0'. For a 100 percent duty cycle, the counter CC register should be set to larger than the counter PERIOD register."]
    #[inline(always)]
    #[must_use]
    pub fn cc_match_mode(&mut self) -> CcMatchModeW<TrCtrl2Spec> {
        CcMatchModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Determines the effect of a counter overflow event (COUNTER reaches PERIOD) on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_mode(&mut self) -> OverflowModeW<TrCtrl2Spec> {
        OverflowModeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Determines the effect of a counter underflow event (COUNTER reaches '0') on the 'line_out' output signals."]
    #[inline(always)]
    #[must_use]
    pub fn underflow_mode(&mut self) -> UnderflowModeW<TrCtrl2Spec> {
        UnderflowModeW::new(self, 4)
    }
}
#[doc = "Counter trigger control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtrl2Spec;
impl crate::RegisterSpec for TrCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctrl2::R`](R) reader structure"]
impl crate::Readable for TrCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctrl2::W`](W) writer structure"]
impl crate::Writable for TrCtrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTRL2 to value 0x3f"]
impl crate::Resettable for TrCtrl2Spec {
    const RESET_VALUE: u32 = 0x3f;
}
