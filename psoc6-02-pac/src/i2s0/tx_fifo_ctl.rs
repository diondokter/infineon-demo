#[doc = "Register `TX_FIFO_CTL` reader"]
pub type R = crate::R<TxFifoCtlSpec>;
#[doc = "Register `TX_FIFO_CTL` writer"]
pub type W = crate::W<TxFifoCtlSpec>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
pub type TriggerLevelR = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
pub type TriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearR = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TriggerLevelR {
        TriggerLevelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TriggerLevelW<TxFifoCtlSpec> {
        TriggerLevelW::new(self, 0)
    }
    #[doc = "Bit 16 - When '1', the TX FIFO and TX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<TxFifoCtlSpec> {
        ClearW::new(self, 16)
    }
    #[doc = "Bit 17 - When '1', hardware reads from the TX FIFO do not remove FIFO entries. Freeze will not advance the TX FIFO read pointer. This field is used only for debugging purposes."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<TxFifoCtlSpec> {
        FreezeW::new(self, 17)
    }
}
#[doc = "TX FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxFifoCtlSpec;
impl crate::RegisterSpec for TxFifoCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for TxFifoCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for TxFifoCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_FIFO_CTL to value 0"]
impl crate::Resettable for TxFifoCtlSpec {
    const RESET_VALUE: u32 = 0;
}
