#[doc = "Register `RAM1_PWR_CTL` reader"]
pub type R = crate::R<Ram1PwrCtlSpec>;
#[doc = "Register `RAM1_PWR_CTL` writer"]
pub type W = crate::W<Ram1PwrCtlSpec>;
#[doc = "Power mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: See RAM0_PWR_MACRO_CTL."]
    Off = 0,
    #[doc = "1: undefined"]
    Rsvd = 1,
    #[doc = "2: See RAM0_PWR_MACRO_CTL."]
    Retained = 2,
    #[doc = "3: See RAM0_PWR_MACRO_CTL."]
    Enabled = 3,
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
#[doc = "Field `PWR_MODE` reader - Power mode."]
pub type PwrModeR = crate::FieldReader<PwrMode>;
impl PwrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrMode {
        match self.bits {
            0 => PwrMode::Off,
            1 => PwrMode::Rsvd,
            2 => PwrMode::Retained,
            3 => PwrMode::Enabled,
            _ => unreachable!(),
        }
    }
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PwrMode::Rsvd
    }
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PwrMode::Retained
    }
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PwrMode::Enabled
    }
}
#[doc = "Field `PWR_MODE` writer - Power mode."]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode, crate::Safe>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Rsvd)
    }
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Retained)
    }
    #[doc = "See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Enabled)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - See RAM0_PWR_MACRO_CTL."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - See RAM0_PWR_MACRO_CTL."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VectkeystatR {
        VectkeystatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<Ram1PwrCtlSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "RAM 1 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram1_pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram1PwrCtlSpec;
impl crate::RegisterSpec for Ram1PwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for Ram1PwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram1_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for Ram1PwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM1_PWR_CTL to value 0xfa05_0003"]
impl crate::Resettable for Ram1PwrCtlSpec {
    const RESET_VALUE: u32 = 0xfa05_0003;
}
