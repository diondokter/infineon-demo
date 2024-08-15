#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enabled {
    #[doc = "0: N/A"]
    Disabled = 0,
    #[doc = "1: N/A"]
    Enabled = 1,
}
impl From<Enabled> for bool {
    #[inline(always)]
    fn from(variant: Enabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
pub type EnabledR = crate::BitReader<Enabled>;
impl EnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enabled {
        match self.bits {
            false => Enabled::Disabled,
            true => Enabled::Enabled,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enabled::Disabled
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enabled::Enabled
    }
}
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG, Enabled>;
impl<'a, REG> EnabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Disabled)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Enabled)
    }
}
impl R {
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers (command and status registers) are reset to their default value when the IP is disabled. All retention registers retain their value when the IP is disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
