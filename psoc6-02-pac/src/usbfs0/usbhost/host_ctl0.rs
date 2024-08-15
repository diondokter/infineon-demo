#[doc = "Register `HOST_CTL0` reader"]
pub type R = crate::R<HostCtl0Spec>;
#[doc = "Register `HOST_CTL0` writer"]
pub type W = crate::W<HostCtl0Spec>;
#[doc = "Field `HOST` reader - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HostR = crate::BitReader;
#[doc = "Field `HOST` writer - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
pub type HostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    pub fn host(&self) -> HostR {
        HostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit selects an operating mode of this IP. '0' : USB Device '1' : USB Host Notes: - The mode of operation mode does not transition immediately after setting this bit. Read this bit to confirm that the operation mode has changed. - This bit is reset to '0' if the ENABLE bit in this register changes from '1' to '0'. - Before changing from the USB Host to the USB Device, check that the following conditions are satisfied and also set the RST bit of the Host Control 1 Register (HOST_CTL1). to '1'. * The SOFBUSY bit of the Host Status Register (HOST_STATUS) is set to '0'. * The TKNEN bits of the Host Token Endpoint Register (HOST_TOKEN) is set to '000'. * The SUSP bit of the Host Status Register (HOST_STATUS) is set to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HostW<HostCtl0Spec> {
        HostW::new(self, 0)
    }
    #[doc = "Bit 31 - This bit enables the operation of this IP. '0' : Disable USB Host '1' : Enable USB Host Note: - This bit doesn't affect the USB Device."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<HostCtl0Spec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Host Control 0 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtl0Spec;
impl crate::RegisterSpec for HostCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctl0::R`](R) reader structure"]
impl crate::Readable for HostCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`host_ctl0::W`](W) writer structure"]
impl crate::Writable for HostCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CTL0 to value 0"]
impl crate::Resettable for HostCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
