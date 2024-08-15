#[doc = "Register `CM4_PWR_CTL` reader"]
pub type R = crate::R<Cm4PwrCtlSpec>;
#[doc = "Register `CM4_PWR_CTL` writer"]
pub type W = crate::W<Cm4PwrCtlSpec>;
#[doc = "Power mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    Off = 0,
    #[doc = "1: Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    Reset = 1,
    #[doc = "2: Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    Retained = 2,
    #[doc = "3: Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
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
            1 => PwrMode::Reset,
            2 => PwrMode::Retained,
            3 => PwrMode::Enabled,
            _ => unreachable!(),
        }
    }
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == PwrMode::Reset
    }
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PwrMode::Retained
    }
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
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
    #[doc = "Switch CM4 off Power off, clock off, isolate, reset and no retain."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "Reset CM4 Clock off, no isolated, no retain and reset. Note: The CM4 CPU has a AIRCR.SYSRESETREQ register field that allows the CM4 to reset the complete device (RESET only resets the CM4), resulting in a warm boot."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Reset)
    }
    #[doc = "Put CM4 in Retained mode This can only become effective if CM4 is in SleepDeep mode. Check PWR_DONE flag to see if CM4 RETAINED state has been reached. Power off, clock off, isolate, no reset and retain."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Retained)
    }
    #[doc = "Switch CM4 on. Power on, clock on, no isolate, no reset and no retain."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Enabled)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    pub fn pwr_mode(&self) -> PwrModeR {
        PwrModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
    #[inline(always)]
    pub fn vectkeystat(&self) -> VectkeystatR {
        VectkeystatR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<Cm4PwrCtlSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "CM4 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4PwrCtlSpec;
impl crate::RegisterSpec for Cm4PwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for Cm4PwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for Cm4PwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_PWR_CTL to value 0xfa05_0001"]
impl crate::Resettable for Cm4PwrCtlSpec {
    const RESET_VALUE: u32 = 0xfa05_0001;
}
