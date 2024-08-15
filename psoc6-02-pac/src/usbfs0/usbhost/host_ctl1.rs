#[doc = "Register `HOST_CTL1` reader"]
pub type R = crate::R<HostCtl1Spec>;
#[doc = "Register `HOST_CTL1` writer"]
pub type W = crate::W<HostCtl1Spec>;
#[doc = "Field `CLKSEL` reader - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type ClkselR = crate::BitReader;
#[doc = "Field `CLKSEL` writer - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USTP` reader - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type UstpR = crate::BitReader;
#[doc = "Field `USTP` writer - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
pub type UstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    pub fn ustp(&self) -> UstpR {
        UstpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects the operating clock of USB Host. '0' : Low-speed clock '1' : Full-speed clock Notes: - This bit is set to it's default vaulue '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - This bit must always be set to '1' in the USB Device mode."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> ClkselW<HostCtl1Spec> {
        ClkselW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit stops the clock for the USB Host operating unit. When this bit is '1', power consumption can be reduced by configuring this bit. '0' : Normal operating mode. '1' : Stops the clock for the USB Host operating unit. Notes: - If this bit is set to '1', the function of USB Host can't be used because internal clock is stopped. - This bit is initialized if ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ustp(&mut self) -> UstpW<HostCtl1Spec> {
        UstpW::new(self, 1)
    }
    #[doc = "Bit 7 - This bit resets the USB Host. '0' : Normal operating mode. '1' : USB Host is reset. Notes: - This bit is to it's default value '1' if the ENABLE bit of the Host Control 0 Register (HOST_CTL0) changes from '1' to '0'. - If this bit is set to '1', both the BFINI bits of the Host Endpoint 1 Control Register (HOST_EP1_CTL) and Host Endpoint 2 Control Register (HOST_EP2_CTL) are set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<HostCtl1Spec> {
        RstW::new(self, 7)
    }
}
#[doc = "Host Control 1 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtl1Spec;
impl crate::RegisterSpec for HostCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctl1::R`](R) reader structure"]
impl crate::Readable for HostCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`host_ctl1::W`](W) writer structure"]
impl crate::Writable for HostCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CTL1 to value 0x83"]
impl crate::Resettable for HostCtl1Spec {
    const RESET_VALUE: u32 = 0x83;
}
