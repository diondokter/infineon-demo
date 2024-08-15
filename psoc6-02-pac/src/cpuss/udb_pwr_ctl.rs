#[doc = "Register `UDB_PWR_CTL` reader"]
pub type R = crate::R<UdbPwrCtlSpec>;
#[doc = "Register `UDB_PWR_CTL` writer"]
pub type W = crate::W<UdbPwrCtlSpec>;
#[doc = "Set Power mode for UDBs\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: See CM4_PWR_CTL"]
    Off = 0,
    #[doc = "1: See CM4_PWR_CTL"]
    Reset = 1,
    #[doc = "2: See CM4_PWR_CTL"]
    Retained = 2,
    #[doc = "3: See CM4_PWR_CTL"]
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
#[doc = "Field `PWR_MODE` reader - Set Power mode for UDBs"]
pub type PwrModeR = crate::FieldReader<PwrMode>;
impl PwrModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrMode {
        match self.bits {
            0 => PwrMode::Off,
            1 => PwrMode::Reset,
            2 => PwrMode::Retained,
            3 => PwrMode::Enabled,
            _ => unreachable!(),
        }
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PwrMode::Reset
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PwrMode::Retained
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PwrMode::Enabled
    }
}
#[doc = "Field `PWR_MODE` writer - Set Power mode for UDBs"]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode, crate::Safe>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Reset)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Retained)
    }
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Enabled)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Set Power mode for UDBs"]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VectkeystatR {
        VectkeystatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set Power mode for UDBs"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<UdbPwrCtlSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "UDB power control\n\nYou can [`read`](crate::Reg::read) this register and get [`udb_pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udb_pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdbPwrCtlSpec;
impl crate::RegisterSpec for UdbPwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udb_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for UdbPwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`udb_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for UdbPwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDB_PWR_CTL to value 0xfa05_0001"]
impl crate::Resettable for UdbPwrCtlSpec {
    const RESET_VALUE: u32 = 0xfa05_0001;
}
