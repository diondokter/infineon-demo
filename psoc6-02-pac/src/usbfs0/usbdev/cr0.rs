#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `DEVICE_ADDRESS` reader - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
pub type DeviceAddressR = crate::FieldReader;
#[doc = "Field `DEVICE_ADDRESS` writer - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
pub type DeviceAddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `USB_ENABLE` reader - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
pub type UsbEnableR = crate::BitReader;
#[doc = "Field `USB_ENABLE` writer - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
pub type UsbEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    pub fn device_address(&self) -> DeviceAddressR {
        DeviceAddressR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    pub fn usb_enable(&self) -> UsbEnableR {
        UsbEnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    #[must_use]
    pub fn device_address(&mut self) -> DeviceAddressW<Cr0Spec> {
        DeviceAddressW::new(self, 0)
    }
    #[doc = "Bit 7 - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    #[must_use]
    pub fn usb_enable(&mut self) -> UsbEnableW<Cr0Spec> {
        UsbEnableW::new(self, 7)
    }
}
#[doc = "USB control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {
    const RESET_VALUE: u32 = 0;
}
