#[doc = "Register `AP_CTL` reader"]
pub type R = crate::R<ApCtlSpec>;
#[doc = "Register `AP_CTL` writer"]
pub type W = crate::W<ApCtlSpec>;
#[doc = "Field `CM0_ENABLE` reader - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type Cm0EnableR = crate::BitReader;
#[doc = "Field `CM0_ENABLE` writer - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
pub type Cm0EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM4_ENABLE` reader - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type Cm4EnableR = crate::BitReader;
#[doc = "Field `CM4_ENABLE` writer - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
pub type Cm4EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_ENABLE` reader - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SysEnableR = crate::BitReader;
#[doc = "Field `SYS_ENABLE` writer - Enables the system AP interface: '0': Disabled. '1': Enabled."]
pub type SysEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM0_DISABLE` reader - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type Cm0DisableR = crate::BitReader;
#[doc = "Field `CM0_DISABLE` writer - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
pub type Cm0DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM4_DISABLE` reader - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type Cm4DisableR = crate::BitReader;
#[doc = "Field `CM4_DISABLE` writer - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
pub type Cm4DisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_DISABLE` reader - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SysDisableR = crate::BitReader;
#[doc = "Field `SYS_DISABLE` writer - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
pub type SysDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm0_enable(&self) -> Cm0EnableR {
        Cm0EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn cm4_enable(&self) -> Cm4EnableR {
        Cm4EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn sys_enable(&self) -> SysEnableR {
        SysEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm0_disable(&self) -> Cm0DisableR {
        Cm0DisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    pub fn cm4_disable(&self) -> Cm4DisableR {
        Cm4DisableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    pub fn sys_disable(&self) -> SysDisableR {
        SysDisableR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the CM0 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cm0_enable(&mut self) -> Cm0EnableW<ApCtlSpec> {
        Cm0EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the CM4 AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cm4_enable(&mut self) -> Cm4EnableW<ApCtlSpec> {
        Cm4EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the system AP interface: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sys_enable(&mut self) -> SysEnableW<ApCtlSpec> {
        SysEnableW::new(self, 2)
    }
    #[doc = "Bit 16 - Disables the CM0 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM0_DISABLE is '0' and CM0_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cm0_disable(&mut self) -> Cm0DisableW<ApCtlSpec> {
        Cm0DisableW::new(self, 16)
    }
    #[doc = "Bit 17 - Disables the CM4 AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when CM4_DISABLE is '0' and CM4_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cm4_disable(&mut self) -> Cm4DisableW<ApCtlSpec> {
        Cm4DisableW::new(self, 17)
    }
    #[doc = "Bit 18 - Disables the system AP interface: '0': Enabled. '1': Disabled. Typically, this field is set by the Cypress boot code with information from eFUSE. The access port is only enabled when SYS_DISABLE is '0' and SYS_ENABLE is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn sys_disable(&mut self) -> SysDisableW<ApCtlSpec> {
        SysDisableW::new(self, 18)
    }
}
#[doc = "Access port control\n\nYou can [`read`](crate::Reg::read) this register and get [`ap_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApCtlSpec;
impl crate::RegisterSpec for ApCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ap_ctl::R`](R) reader structure"]
impl crate::Readable for ApCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ap_ctl::W`](W) writer structure"]
impl crate::Writable for ApCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AP_CTL to value 0"]
impl crate::Resettable for ApCtlSpec {
    const RESET_VALUE: u32 = 0;
}
