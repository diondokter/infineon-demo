#[doc = "Register `USBIO_CR2` reader"]
pub type R = crate::R<UsbioCr2Spec>;
#[doc = "Register `USBIO_CR2` writer"]
pub type W = crate::W<UsbioCr2Spec>;
#[doc = "Field `RSVD_5_0` reader - N/A"]
pub type Rsvd5_0R = crate::FieldReader;
#[doc = "Field `TEST_PKT` reader - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TestPktR = crate::BitReader;
#[doc = "Field `TEST_PKT` writer - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub type TestPktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_7` reader - N/A"]
pub type Rsvd7R = crate::BitReader;
#[doc = "Field `RSVD_7` writer - N/A"]
pub type Rsvd7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5_0(&self) -> Rsvd5_0R {
        Rsvd5_0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TestPktR {
        TestPktR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> Rsvd7R {
        Rsvd7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    #[must_use]
    pub fn test_pkt(&mut self) -> TestPktW<UsbioCr2Spec> {
        TestPktW::new(self, 6)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_7(&mut self) -> Rsvd7W<UsbioCr2Spec> {
        Rsvd7W::new(self, 7)
    }
}
#[doc = "USBIO control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbioCr2Spec;
impl crate::RegisterSpec for UsbioCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr2::R`](R) reader structure"]
impl crate::Readable for UsbioCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr2::W`](W) writer structure"]
impl crate::Writable for UsbioCr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBIO_CR2 to value 0"]
impl crate::Resettable for UsbioCr2Spec {
    const RESET_VALUE: u32 = 0;
}
