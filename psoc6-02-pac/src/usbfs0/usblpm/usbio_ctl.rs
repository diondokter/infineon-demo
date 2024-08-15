#[doc = "Register `USBIO_CTL` reader"]
pub type R = crate::R<UsbioCtlSpec>;
#[doc = "Register `USBIO_CTL` writer"]
pub type W = crate::W<UsbioCtlSpec>;
#[doc = "The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmP {
    #[doc = "0: Mode 0: Output buffer off (high Z). Input buffer off."]
    Off = 0,
    #[doc = "1: Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    Input = 1,
}
impl From<DmP> for u8 {
    #[inline(always)]
    fn from(variant: DmP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmP {
    type Ux = u8;
}
impl crate::IsEnum for DmP {}
#[doc = "Field `DM_P` reader - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DmPR = crate::FieldReader<DmP>;
impl DmPR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmP> {
        match self.bits {
            0 => Some(DmP::Off),
            1 => Some(DmP::Input),
            _ => None,
        }
    }
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DmP::Off
    }
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DmP::Input
    }
}
#[doc = "Field `DM_P` writer - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
pub type DmPW<'a, REG> = crate::FieldWriter<'a, REG, 3, DmP>;
impl<'a, REG> DmPW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0: Output buffer off (high Z). Input buffer off."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DmP::Off)
    }
    #[doc = "Mode 1: Output buffer off (high Z). Input buffer on. Other values, not supported."]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DmP::Input)
    }
}
#[doc = "Field `DM_M` reader - The GPIO Drive Mode for DM IO pad."]
pub type DmMR = crate::FieldReader;
#[doc = "Field `DM_M` writer - The GPIO Drive Mode for DM IO pad."]
pub type DmMW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    pub fn dm_p(&self) -> DmPR {
        DmPR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    pub fn dm_m(&self) -> DmMR {
        DmMR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - The GPIO Drive Mode for DP IO pad. This field only applies if USBIO_CR1.IOMODE =1. Data comes from the corresponding GPIO.DR register."]
    #[inline(always)]
    #[must_use]
    pub fn dm_p(&mut self) -> DmPW<UsbioCtlSpec> {
        DmPW::new(self, 0)
    }
    #[doc = "Bits 3:5 - The GPIO Drive Mode for DM IO pad."]
    #[inline(always)]
    #[must_use]
    pub fn dm_m(&mut self) -> DmMW<UsbioCtlSpec> {
        DmMW::new(self, 3)
    }
}
#[doc = "USB IO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbioCtlSpec;
impl crate::RegisterSpec for UsbioCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_ctl::R`](R) reader structure"]
impl crate::Readable for UsbioCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbio_ctl::W`](W) writer structure"]
impl crate::Writable for UsbioCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBIO_CTL to value 0"]
impl crate::Resettable for UsbioCtlSpec {
    const RESET_VALUE: u32 = 0;
}
