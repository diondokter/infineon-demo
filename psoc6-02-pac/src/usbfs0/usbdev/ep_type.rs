#[doc = "Register `EP_TYPE` reader"]
pub type R = crate::R<EpTypeSpec>;
#[doc = "Register `EP_TYPE` writer"]
pub type W = crate::W<EpTypeSpec>;
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep1Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep1Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep1Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP1_TYP` reader - Endpoint Type Indication."]
pub type Ep1TypR = crate::BitReader<Ep1Typ>;
impl Ep1TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep1Typ {
        match self.bits {
            false => Ep1Typ::EpIn,
            true => Ep1Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep1Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep1Typ::EpOut
    }
}
#[doc = "Field `EP1_TYP` writer - Endpoint Type Indication."]
pub type Ep1TypW<'a, REG> = crate::BitWriter<'a, REG, Ep1Typ>;
impl<'a, REG> Ep1TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep1Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep2Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep2Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep2Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP2_TYP` reader - Endpoint Type Indication."]
pub type Ep2TypR = crate::BitReader<Ep2Typ>;
impl Ep2TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep2Typ {
        match self.bits {
            false => Ep2Typ::EpIn,
            true => Ep2Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep2Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep2Typ::EpOut
    }
}
#[doc = "Field `EP2_TYP` writer - Endpoint Type Indication."]
pub type Ep2TypW<'a, REG> = crate::BitWriter<'a, REG, Ep2Typ>;
impl<'a, REG> Ep2TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep2Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep3Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep3Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep3Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP3_TYP` reader - Endpoint Type Indication."]
pub type Ep3TypR = crate::BitReader<Ep3Typ>;
impl Ep3TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep3Typ {
        match self.bits {
            false => Ep3Typ::EpIn,
            true => Ep3Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep3Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep3Typ::EpOut
    }
}
#[doc = "Field `EP3_TYP` writer - Endpoint Type Indication."]
pub type Ep3TypW<'a, REG> = crate::BitWriter<'a, REG, Ep3Typ>;
impl<'a, REG> Ep3TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep3Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep4Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep4Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep4Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP4_TYP` reader - Endpoint Type Indication."]
pub type Ep4TypR = crate::BitReader<Ep4Typ>;
impl Ep4TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep4Typ {
        match self.bits {
            false => Ep4Typ::EpIn,
            true => Ep4Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep4Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep4Typ::EpOut
    }
}
#[doc = "Field `EP4_TYP` writer - Endpoint Type Indication."]
pub type Ep4TypW<'a, REG> = crate::BitWriter<'a, REG, Ep4Typ>;
impl<'a, REG> Ep4TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep4Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep5Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep5Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep5Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP5_TYP` reader - Endpoint Type Indication."]
pub type Ep5TypR = crate::BitReader<Ep5Typ>;
impl Ep5TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep5Typ {
        match self.bits {
            false => Ep5Typ::EpIn,
            true => Ep5Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep5Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep5Typ::EpOut
    }
}
#[doc = "Field `EP5_TYP` writer - Endpoint Type Indication."]
pub type Ep5TypW<'a, REG> = crate::BitWriter<'a, REG, Ep5Typ>;
impl<'a, REG> Ep5TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep5Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep6Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep6Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep6Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP6_TYP` reader - Endpoint Type Indication."]
pub type Ep6TypR = crate::BitReader<Ep6Typ>;
impl Ep6TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep6Typ {
        match self.bits {
            false => Ep6Typ::EpIn,
            true => Ep6Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep6Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep6Typ::EpOut
    }
}
#[doc = "Field `EP6_TYP` writer - Endpoint Type Indication."]
pub type Ep6TypW<'a, REG> = crate::BitWriter<'a, REG, Ep6Typ>;
impl<'a, REG> Ep6TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep6Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep6Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep7Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep7Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep7Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP7_TYP` reader - Endpoint Type Indication."]
pub type Ep7TypR = crate::BitReader<Ep7Typ>;
impl Ep7TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep7Typ {
        match self.bits {
            false => Ep7Typ::EpIn,
            true => Ep7Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep7Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep7Typ::EpOut
    }
}
#[doc = "Field `EP7_TYP` writer - Endpoint Type Indication."]
pub type Ep7TypW<'a, REG> = crate::BitWriter<'a, REG, Ep7Typ>;
impl<'a, REG> Ep7TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep7Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep7Typ::EpOut)
    }
}
#[doc = "Endpoint Type Indication.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ep8Typ {
    #[doc = "0: IN outpoint"]
    EpIn = 0,
    #[doc = "1: OUT outpoint"]
    EpOut = 1,
}
impl From<Ep8Typ> for bool {
    #[inline(always)]
    fn from(variant: Ep8Typ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EP8_TYP` reader - Endpoint Type Indication."]
pub type Ep8TypR = crate::BitReader<Ep8Typ>;
impl Ep8TypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ep8Typ {
        match self.bits {
            false => Ep8Typ::EpIn,
            true => Ep8Typ::EpOut,
        }
    }
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn is_ep_in(&self) -> bool {
        *self == Ep8Typ::EpIn
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn is_ep_out(&self) -> bool {
        *self == Ep8Typ::EpOut
    }
}
#[doc = "Field `EP8_TYP` writer - Endpoint Type Indication."]
pub type Ep8TypW<'a, REG> = crate::BitWriter<'a, REG, Ep8Typ>;
impl<'a, REG> Ep8TypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IN outpoint"]
    #[inline(always)]
    pub fn ep_in(self) -> &'a mut crate::W<REG> {
        self.variant(Ep8Typ::EpIn)
    }
    #[doc = "OUT outpoint"]
    #[inline(always)]
    pub fn ep_out(self) -> &'a mut crate::W<REG> {
        self.variant(Ep8Typ::EpOut)
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep1_typ(&self) -> Ep1TypR {
        Ep1TypR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep2_typ(&self) -> Ep2TypR {
        Ep2TypR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep3_typ(&self) -> Ep3TypR {
        Ep3TypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep4_typ(&self) -> Ep4TypR {
        Ep4TypR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep5_typ(&self) -> Ep5TypR {
        Ep5TypR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep6_typ(&self) -> Ep6TypR {
        Ep6TypR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep7_typ(&self) -> Ep7TypR {
        Ep7TypR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    pub fn ep8_typ(&self) -> Ep8TypR {
        Ep8TypR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep1_typ(&mut self) -> Ep1TypW<EpTypeSpec> {
        Ep1TypW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep2_typ(&mut self) -> Ep2TypW<EpTypeSpec> {
        Ep2TypW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep3_typ(&mut self) -> Ep3TypW<EpTypeSpec> {
        Ep3TypW::new(self, 2)
    }
    #[doc = "Bit 3 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep4_typ(&mut self) -> Ep4TypW<EpTypeSpec> {
        Ep4TypW::new(self, 3)
    }
    #[doc = "Bit 4 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep5_typ(&mut self) -> Ep5TypW<EpTypeSpec> {
        Ep5TypW::new(self, 4)
    }
    #[doc = "Bit 5 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep6_typ(&mut self) -> Ep6TypW<EpTypeSpec> {
        Ep6TypW::new(self, 5)
    }
    #[doc = "Bit 6 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep7_typ(&mut self) -> Ep7TypW<EpTypeSpec> {
        Ep7TypW::new(self, 6)
    }
    #[doc = "Bit 7 - Endpoint Type Indication."]
    #[inline(always)]
    #[must_use]
    pub fn ep8_typ(&mut self) -> Ep8TypW<EpTypeSpec> {
        Ep8TypW::new(self, 7)
    }
}
#[doc = "Endpoint Type (IN/OUT) Indication *1\n\nYou can [`read`](crate::Reg::read) this register and get [`ep_type::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep_type::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpTypeSpec;
impl crate::RegisterSpec for EpTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_type::R`](R) reader structure"]
impl crate::Readable for EpTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_type::W`](W) writer structure"]
impl crate::Writable for EpTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP_TYPE to value 0"]
impl crate::Resettable for EpTypeSpec {
    const RESET_VALUE: u32 = 0;
}
