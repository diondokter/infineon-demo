#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `EOS_INTR` reader - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EosIntrR = crate::BitReader;
#[doc = "Field `EOS_INTR` writer - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
pub type EosIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW_INTR` reader - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OverflowIntrR = crate::BitReader;
#[doc = "Field `OVERFLOW_INTR` writer - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
pub type OverflowIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FW_COLLISION_INTR` reader - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FwCollisionIntrR = crate::BitReader;
#[doc = "Field `FW_COLLISION_INTR` writer - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type FwCollisionIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_COLLISION_INTR` reader - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DsiCollisionIntrR = crate::BitReader;
#[doc = "Field `DSI_COLLISION_INTR` writer - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
pub type DsiCollisionIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_EOC_INTR` reader - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type InjEocIntrR = crate::BitReader;
#[doc = "Field `INJ_EOC_INTR` writer - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
pub type InjEocIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_SATURATE_INTR` reader - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type InjSaturateIntrR = crate::BitReader;
#[doc = "Field `INJ_SATURATE_INTR` writer - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type InjSaturateIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_RANGE_INTR` reader - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type InjRangeIntrR = crate::BitReader;
#[doc = "Field `INJ_RANGE_INTR` writer - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
pub type InjRangeIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INJ_COLLISION_INTR` reader - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type InjCollisionIntrR = crate::BitReader;
#[doc = "Field `INJ_COLLISION_INTR` writer - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
pub type InjCollisionIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn eos_intr(&self) -> EosIntrR {
        EosIntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn overflow_intr(&self) -> OverflowIntrR {
        OverflowIntrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn fw_collision_intr(&self) -> FwCollisionIntrR {
        FwCollisionIntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn dsi_collision_intr(&self) -> DsiCollisionIntrR {
        DsiCollisionIntrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_eoc_intr(&self) -> InjEocIntrR {
        InjEocIntrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_saturate_intr(&self) -> InjSaturateIntrR {
        InjSaturateIntrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_range_intr(&self) -> InjRangeIntrR {
        InjRangeIntrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn inj_collision_intr(&self) -> InjCollisionIntrR {
        InjCollisionIntrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Of Scan Interrupt: hardware sets this interrupt after completing a scan of all the enabled channels. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn eos_intr(&mut self) -> EosIntrW<IntrSpec> {
        EosIntrW::new(self, 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt: hardware sets this interrupt when it sets a new EOS_INTR while that bit was not yet cleared by the firmware. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn overflow_intr(&mut self) -> OverflowIntrW<IntrSpec> {
        OverflowIntrW::new(self, 1)
    }
    #[doc = "Bit 2 - Firmware Collision Interrupt: hardware sets this interrupt when FW_TRIGGER is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the FW_TRIGGER has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn fw_collision_intr(&mut self) -> FwCollisionIntrW<IntrSpec> {
        FwCollisionIntrW::new(self, 2)
    }
    #[doc = "Bit 3 - DSI Collision Interrupt: hardware sets this interrupt when the DSI trigger signal is asserted while the SAR is BUSY. Raising this interrupt is delayed to when the scan caused by the DSI trigger has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the channels were sampled later than was intended (jitter). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_collision_intr(&mut self) -> DsiCollisionIntrW<IntrSpec> {
        DsiCollisionIntrW::new(self, 3)
    }
    #[doc = "Bit 4 - Injection End of Conversion Interrupt: hardware sets this interrupt after completing the conversion for the injection channel (irrespective of if tailgating was used). Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_eoc_intr(&mut self) -> InjEocIntrW<IntrSpec> {
        InjEocIntrW::new(self, 4)
    }
    #[doc = "Bit 5 - Injection Saturation Interrupt: hardware sets this interrupt if an injection conversion result (before averaging) is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_saturate_intr(&mut self) -> InjSaturateIntrW<IntrSpec> {
        InjSaturateIntrW::new(self, 5)
    }
    #[doc = "Bit 6 - Injection Range detect Interrupt: hardware sets this interrupt if the injection conversion result (after averaging) met the condition specified by the SAR_RANGE registers. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_range_intr(&mut self) -> InjRangeIntrW<IntrSpec> {
        InjRangeIntrW::new(self, 6)
    }
    #[doc = "Bit 7 - Injection Collision Interrupt: hardware sets this interrupt when the injection trigger signal is asserted (INJ_START_EN==1 &amp;&amp; INJ_TAILGATING==0) while the SAR is BUSY. Raising this interrupt is delayed to when the sampling of the injection channel has been completed, i.e. not when the preceding scan with which this trigger collided is completed. When this interrupt is set it implies that the injection channel was sampled later than was intended. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn inj_collision_intr(&mut self) -> InjCollisionIntrW<IntrSpec> {
        InjCollisionIntrW::new(self, 7)
    }
}
#[doc = "Interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
