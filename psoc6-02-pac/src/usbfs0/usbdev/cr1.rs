#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `REG_ENABLE` reader - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
pub type RegEnableR = crate::BitReader;
#[doc = "Field `REG_ENABLE` writer - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
pub type RegEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_LOCK` reader - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
pub type EnableLockR = crate::BitReader;
#[doc = "Field `ENABLE_LOCK` writer - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
pub type EnableLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_ACTIVITY` reader - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
pub type BusActivityR = crate::BitReader;
#[doc = "Field `BUS_ACTIVITY` writer - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
pub type BusActivityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_3` reader - N/A"]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD_3` writer - N/A"]
pub type Rsvd3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn reg_enable(&self) -> RegEnableR {
        RegEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn enable_lock(&self) -> EnableLockR {
        EnableLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn bus_activity(&self) -> BusActivityR {
        BusActivityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    #[must_use]
    pub fn reg_enable(&mut self) -> RegEnableW<Cr1Spec> {
        RegEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    #[must_use]
    pub fn enable_lock(&mut self) -> EnableLockW<Cr1Spec> {
        EnableLockW::new(self, 1)
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    #[must_use]
    pub fn bus_activity(&mut self) -> BusActivityW<Cr1Spec> {
        BusActivityW::new(self, 2)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_3(&mut self) -> Rsvd3W<Cr1Spec> {
        Rsvd3W::new(self, 3)
    }
}
#[doc = "USB control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0;
}
