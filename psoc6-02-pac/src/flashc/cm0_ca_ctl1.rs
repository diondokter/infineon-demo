#[doc = "Register `CM0_CA_CTL1` reader"]
pub type R = crate::R<Cm0CaCtl1Spec>;
#[doc = "Register `CM0_CA_CTL1` writer"]
pub type W = crate::W<Cm0CaCtl1Spec>;
#[doc = "Specifies power mode for CM0 cache. The following sequnece should be followed for turning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrMode {
    #[doc = "0: Power OFF the CM0 cache SRAM."]
    Off = 0,
    #[doc = "1: Undefined"]
    Rsvd = 1,
    #[doc = "2: Put CM0 cache SRAM in retained mode."]
    Retained = 2,
    #[doc = "3: Enable/Turn ON the CM0 cache SRAM."]
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
#[doc = "Field `PWR_MODE` reader - Specifies power mode for CM0 cache. The following sequnece should be followed for turning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
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
    #[doc = "Power OFF the CM0 cache SRAM."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PwrMode::Off
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == PwrMode::Rsvd
    }
    #[doc = "Put CM0 cache SRAM in retained mode."]
    #[inline(always)]
    pub fn is_retained(&self) -> bool {
        *self == PwrMode::Retained
    }
    #[doc = "Enable/Turn ON the CM0 cache SRAM."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PwrMode::Enabled
    }
}
#[doc = "Field `PWR_MODE` writer - Specifies power mode for CM0 cache. The following sequnece should be followed for turning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
pub type PwrModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwrMode, crate::Safe>;
impl<'a, REG> PwrModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power OFF the CM0 cache SRAM."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Off)
    }
    #[doc = "Undefined"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Rsvd)
    }
    #[doc = "Put CM0 cache SRAM in retained mode."]
    #[inline(always)]
    pub fn retained(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Retained)
    }
    #[doc = "Enable/Turn ON the CM0 cache SRAM."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PwrMode::Enabled)
    }
}
#[doc = "Field `VECTKEYSTAT` reader - Register key (to prevent accidental writes). - Should be written with a 0x05fa key value for the write to take effect. - Always reads as 0xfa05. Note: Although the SW attribute for this field says ''R', SW need to write the key 0x05fa in this field for this register write to happen. This is a built in protection provided to prevent accidental writes from SW."]
pub type VectkeystatR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Specifies power mode for CM0 cache. The following sequnece should be followed for turning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
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
    #[doc = "Bits 0:1 - Specifies power mode for CM0 cache. The following sequnece should be followed for turning OFF/ON the cache SRAM. Turn OFF sequence: a) Write CM0_CA_CTL0 to disable cache. b) Write CM0_CA_CTL1 to turn OFF cache SRAM. Turn ON sequence: a) Write CM0_CA_CTL1 to turn ON cache SRAM. b) Delay to allow power up of cache SRAM. Delay should be at a minimum of CM0_CA_CTL2.PWRUP_DELAY CLK_SLOW clock cycles. c) Write CM0_CA_CTL0 to enable cache."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_mode(&mut self) -> PwrModeW<Cm0CaCtl1Spec> {
        PwrModeW::new(self, 0)
    }
}
#[doc = "CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CaCtl1Spec;
impl crate::RegisterSpec for Cm0CaCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_ctl1::R`](R) reader structure"]
impl crate::Readable for Cm0CaCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`cm0_ca_ctl1::W`](W) writer structure"]
impl crate::Writable for Cm0CaCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_CA_CTL1 to value 0xfa05_0003"]
impl crate::Resettable for Cm0CaCtl1Spec {
    const RESET_VALUE: u32 = 0xfa05_0003;
}
