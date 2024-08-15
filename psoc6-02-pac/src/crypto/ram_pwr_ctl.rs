#[doc = "Register `RAM_PWR_CTL` reader"]
pub type R = crate::R<RamPwrCtlSpec>;
#[doc = "Register `RAM_PWR_CTL` writer"]
pub type W = crate::W<RamPwrCtlSpec>;
#[doc = "Set power mode for memory buffer SRAM.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: See CM4_PWR_CTL"]
    Off = 0,
    #[doc = "1: undefined"]
    Rsvd = 1,
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
#[doc = "Field `PWR_MODE` reader - Set power mode for memory buffer SRAM."]
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
    #[doc = "See CM4_PWR_CTL"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PwrMode::Rsvd
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
#[doc = "Field `PWR_MODE` writer - Set power mode for memory buffer SRAM."]
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
    #[doc = "undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Rsvd)
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
impl R {
    #[doc = "Bits 0:1 - Set power mode for memory buffer SRAM."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set power mode for memory buffer SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<RamPwrCtlSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "SRAM power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamPwrCtlSpec;
impl crate::RegisterSpec for RamPwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for RamPwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for RamPwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM_PWR_CTL to value 0x03"]
impl crate::Resettable for RamPwrCtlSpec {
    const RESET_VALUE: u32 = 0x03;
}
