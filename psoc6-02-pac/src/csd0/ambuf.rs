#[doc = "Register `AMBUF` reader"]
pub type R = crate::R<AmbufSpec>;
#[doc = "Register `AMBUF` writer"]
pub type W = crate::W<AmbufSpec>;
#[doc = "Amux buffer power level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: Disable buffer"]
    Off = 0,
    #[doc = "1: On, normal or low power level depending on CONFIG.LP_MODE."]
    Norm = 1,
    #[doc = "2: On, high or low power level depending on CONFIG.LP_MODE."]
    Hi = 2,
}
impl From<PwrMode> for u8 {
    #[inline(always)]
    fn from(variant: PwrMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrMode {
    type Ux = u8;
}
impl crate::IsEnum for PwrMode {}
#[doc = "Field `PWR_MODE` reader - Amux buffer power level"]
pub type PwrModeR = crate::FieldReader<PwrMode>;
impl PwrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PwrMode> {
        match self.bits {
            0 => Some(PwrMode::Off),
            1 => Some(PwrMode::Norm),
            2 => Some(PwrMode::Hi),
            _ => None,
        }
    }
    #[doc = "Disable buffer"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        *self == PwrMode::Norm
    }
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn is_hi(&self) -> bool {
        *self == PwrMode::Hi
    }
}
#[doc = "Field `PWR_MODE` writer - Amux buffer power level"]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable buffer"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "On, normal or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Norm)
    }
    #[doc = "On, high or low power level depending on CONFIG.LP_MODE."]
    #[inline(always)]
    pub fn hi(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Hi)
    }
}
impl R {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Amux buffer power level"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<AmbufSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "Reference Generator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ambuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ambuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmbufSpec;
impl crate::RegisterSpec for AmbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ambuf::R`](R) reader structure"]
impl crate::Readable for AmbufSpec {}
#[doc = "`write(|w| ..)` method takes [`ambuf::W`](W) writer structure"]
impl crate::Writable for AmbufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMBUF to value 0"]
impl crate::Resettable for AmbufSpec {
    const RESET_VALUE: u32 = 0;
}
