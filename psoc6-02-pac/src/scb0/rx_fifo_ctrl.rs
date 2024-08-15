#[doc = "Register `RX_FIFO_CTRL` reader"]
pub type R = crate::R<RxFifoCtrlSpec>;
#[doc = "Register `RX_FIFO_CTRL` writer"]
pub type W = crate::W<RxFifoCtrlSpec>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
pub type TriggerLevelR = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
pub type TriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearR = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TriggerLevelR {
        TriggerLevelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has more entries than the number of this field, a receiver trigger event INTR_RX.TRIGGER is generated."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TriggerLevelW<RxFifoCtrlSpec> {
        TriggerLevelW::new(self, 0)
    }
    #[doc = "Bit 16 - When '1', the receiver FIFO and receiver shift register are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<RxFifoCtrlSpec> {
        ClearW::new(self, 16)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the receiver FIFO have no effect. Freeze will not advance the RX FIFO write pointer."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<RxFifoCtrlSpec> {
        FreezeW::new(self, 17)
    }
}
#[doc = "Receiver FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoCtrlSpec;
impl crate::RegisterSpec for RxFifoCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_ctrl::R`](R) reader structure"]
impl crate::Readable for RxFifoCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for RxFifoCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_FIFO_CTRL to value 0"]
impl crate::Resettable for RxFifoCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
