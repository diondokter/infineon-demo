#[doc = "Register `RX_FIFO_CTL` reader"]
pub type R = crate::R<RxFifoCtlSpec>;
#[doc = "Register `RX_FIFO_CTL` writer"]
pub type W = crate::W<RxFifoCtlSpec>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
pub type TriggerLevelR = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
pub type TriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLEAR` reader - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearR = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEZE` reader - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
pub type FreezeR = crate::BitReader;
#[doc = "Field `FREEZE` writer - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
pub type FreezeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TriggerLevelR {
        TriggerLevelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
    #[inline(always)]
    pub fn freeze(&self) -> FreezeR {
        FreezeR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated. Note: software can configure up to 253 in I2S mode or Left Justified (RX_CTL.I2S_MODE = '0' or '1'). In TDM mode (RX_CTL.I2S_MODE = '2' or '3'), it can configure up to \\[256 - (RX_CTL.CH_NR+2)\\]."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TriggerLevelW<RxFifoCtlSpec> {
        TriggerLevelW::new(self, 0)
    }
    #[doc = "Bit 16 - When '1', the RX FIFO and RX_BUF are cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<RxFifoCtlSpec> {
        ClearW::new(self, 16)
    }
    #[doc = "Bit 17 - When '1', hardware writes to the RX FIFO have no effect. Freeze will not advance the RX FIFO write pointer. This field is used only for debugging purposee."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FreezeW<RxFifoCtlSpec> {
        FreezeW::new(self, 17)
    }
}
#[doc = "RX FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxFifoCtlSpec;
impl crate::RegisterSpec for RxFifoCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for RxFifoCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for RxFifoCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_FIFO_CTL to value 0"]
impl crate::Resettable for RxFifoCtlSpec {
    const RESET_VALUE: u32 = 0;
}
