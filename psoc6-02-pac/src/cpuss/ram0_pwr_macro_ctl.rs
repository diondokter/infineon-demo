#[doc = "Register `RAM0_PWR_MACRO_CTL[%s]` reader"]
pub type R = crate::R<Ram0PwrMacroCtlSpec>;
#[doc = "Register `RAM0_PWR_MACRO_CTL[%s]` writer"]
pub type W = crate::W<Ram0PwrMacroCtlSpec>;
#[doc = "SRAM Power mode.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    Off = 0,
    #[doc = "1: undefined"]
    Rsvd = 1,
    #[doc = "2: Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    Retained = 2,
    #[doc = "3: Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
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
#[doc = "Field `PWR_MODE` reader - SRAM Power mode."]
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
    #[doc = "Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PwrMode::Rsvd
    }
    #[doc = "Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PwrMode::Retained
    }
    #[doc = "Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PwrMode::Enabled
    }
}
#[doc = "Field `PWR_MODE` writer - SRAM Power mode."]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode, crate::Safe>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Turn OFF the SRAM. This will trun OFF both array and periphery power of the SRAM and SRAM memory contents are lost."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Rsvd)
    }
    #[doc = "Keep SRAM in Retained mode. This will turn OFF the SRAM periphery power, but array power is ON to retain memory contents. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Retained)
    }
    #[doc = "Enable SRAM for regular operation. The SRAM contents will be retained in DeepSleep system power mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Enabled)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - SRAM Power mode."]
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
    #[doc = "Bits 0:1 - SRAM Power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<Ram0PwrMacroCtlSpec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "RAM 0 power control\n\nYou can [`read`](crate::Reg::read) this register and get [`ram0_pwr_macro_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram0_pwr_macro_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram0PwrMacroCtlSpec;
impl crate::RegisterSpec for Ram0PwrMacroCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram0_pwr_macro_ctl::R`](R) reader structure"]
impl crate::Readable for Ram0PwrMacroCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ram0_pwr_macro_ctl::W`](W) writer structure"]
impl crate::Writable for Ram0PwrMacroCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAM0_PWR_MACRO_CTL[%s]
to value 0xfa05_0003"]
impl crate::Resettable for Ram0PwrMacroCtlSpec {
    const RESET_VALUE: u32 = 0xfa05_0003;
}
