#[doc = "Register `USBIO_CR0` reader"]
pub type R = crate::R<UsbioCr0Spec>;
#[doc = "Register `USBIO_CR0` writer"]
pub type W = crate::W<UsbioCr0Spec>;
#[doc = "Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rd {
    #[doc = "0: D+ &lt; D- (K state)"]
    DiffLow = 0,
    #[doc = "1: D+ > D- (J state)"]
    DiffHigh = 1,
}
impl From<Rd> for bool {
    #[inline(always)]
    fn from(variant: Rd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD` reader - Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
pub type RdR = crate::BitReader<Rd>;
impl RdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rd {
        match self.bits {
            false => Rd::DiffLow,
            true => Rd::DiffHigh,
        }
    }
    #[doc = "D+ &lt; D- (K state)"]
    #[inline(always)]
    pub fn is_diff_low(&self) -> bool {
        *self == Rd::DiffLow
    }
    #[doc = "D+ > D- (J state)"]
    #[inline(always)]
    pub fn is_diff_high(&self) -> bool {
        *self == Rd::DiffHigh
    }
}
#[doc = "Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Td {
    #[doc = "0: Force USB K state (D+ is low D- is high)."]
    DiffK = 0,
    #[doc = "1: Force USB J state (D+ is high D- is low)."]
    DiffJ = 1,
}
impl From<Td> for bool {
    #[inline(always)]
    fn from(variant: Td) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TD` reader - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
pub type TdR = crate::BitReader<Td>;
impl TdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Td {
        match self.bits {
            false => Td::DiffK,
            true => Td::DiffJ,
        }
    }
    #[doc = "Force USB K state (D+ is low D- is high)."]
    #[inline(always)]
    pub fn is_diff_k(&self) -> bool {
        *self == Td::DiffK
    }
    #[doc = "Force USB J state (D+ is high D- is low)."]
    #[inline(always)]
    pub fn is_diff_j(&self) -> bool {
        *self == Td::DiffJ
    }
}
#[doc = "Field `TD` writer - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
pub type TdW<'a, REG> = crate::BitWriter<'a, REG, Td>;
impl<'a, REG> TdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force USB K state (D+ is low D- is high)."]
    #[inline(always)]
    pub fn diff_k(self) -> &'a mut crate::W<REG> {
        self.variant(Td::DiffK)
    }
    #[doc = "Force USB J state (D+ is high D- is low)."]
    #[inline(always)]
    pub fn diff_j(self) -> &'a mut crate::W<REG> {
        self.variant(Td::DiffJ)
    }
}
#[doc = "Field `TSE0` reader - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
pub type Tse0R = crate::BitReader;
#[doc = "Field `TSE0` writer - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
pub type Tse0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Received Data. This read only bit gives the state of the USB differential receiver when IOMODE bit is '0' and USB doesn't transmit. This bit is valid if USB Device. If D+=D- (SE0), this value is undefined."]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    pub fn td(&self) -> TdR {
        TdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    pub fn tse0(&self) -> Tse0R {
        Tse0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Transmit Data. Transmit a USB J or K state on the USB bus. No effect if TEN=0 or TSE0=1."]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TdW<UsbioCr0Spec> {
        TdW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Single-Ended Zero. SE0: both D+ and D- low. No effect if TEN=0."]
    #[inline(always)]
    #[must_use]
    pub fn tse0(&mut self) -> Tse0W<UsbioCr0Spec> {
        Tse0W::new(self, 6)
    }
    #[doc = "Bit 7 - USB Transmit Enable. This is used to manually transmit on the D+ and D- pins. Normally this bit should be cleared to allow the internal SIE to drive the pins. The most common reason for manually transmitting is to force a resume state on the bus."]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<UsbioCr0Spec> {
        TenW::new(self, 7)
    }
}
#[doc = "USBIO Control 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbio_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbio_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbioCr0Spec;
impl crate::RegisterSpec for UsbioCr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbio_cr0::R`](R) reader structure"]
impl crate::Readable for UsbioCr0Spec {}
#[doc = "`write(|w| ..)` method takes [`usbio_cr0::W`](W) writer structure"]
impl crate::Writable for UsbioCr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBIO_CR0 to value 0"]
impl crate::Resettable for UsbioCr0Spec {
    const RESET_VALUE: u32 = 0;
}
