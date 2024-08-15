#[doc = "Register `USBIO_CR1` reader"]
pub type R = crate::R<UsbioCr1Spec>;
#[doc = "Register `USBIO_CR1` writer"]
pub type W = crate::W<UsbioCr1Spec>;
#[doc = "Field `DMO` reader - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
pub type DmoR = crate::BitReader;
#[doc = "Field `DPO` reader - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
pub type DpoR = crate::BitReader;
#[doc = "Field `RSVD_2` reader - N/A"]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `RSVD_2` writer - N/A"]
pub type Rsvd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMODE` reader - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub type IomodeR = crate::BitReader;
#[doc = "Field `IOMODE` writer - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub type IomodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dmo(&self) -> DmoR {
        DmoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dpo(&self) -> DpoR {
        DpoR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_2(&mut self) -> Rsvd2W<UsbioCr1Spec> {
        Rsvd2W::new(self, 2)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    #[must_use]
    pub fn iomode(&mut self) -> IomodeW<UsbioCr1Spec> {
        IomodeW::new(self, 5)
    }
}
#[doc = "USBIO control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbioCr1Spec;
impl crate::RegisterSpec for UsbioCr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr1::R`](R) reader structure"]
impl crate::Readable for UsbioCr1Spec {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr1::W`](W) writer structure"]
impl crate::Writable for UsbioCr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBIO_CR1 to value 0x20"]
impl crate::Resettable for UsbioCr1Spec {
    const RESET_VALUE: u32 = 0x20;
}
