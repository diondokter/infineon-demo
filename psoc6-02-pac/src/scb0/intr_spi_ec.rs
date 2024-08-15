#[doc = "Register `INTR_SPI_EC` reader"]
pub type R = crate::R<IntrSpiEcSpec>;
#[doc = "Register `INTR_SPI_EC` writer"]
pub type W = crate::W<IntrSpiEcSpec>;
#[doc = "Field `WAKE_UP` reader - Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `WAKE_UP` writer - Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
pub type WakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_STOP` reader - STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
pub type EzStopR = crate::BitReader;
#[doc = "Field `EZ_STOP` writer - STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
pub type EzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_WRITE_STOP` reader - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EzWriteStopR = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` writer - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_READ_STOP` reader - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EzReadStopR = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` writer - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
pub type EzReadStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EzStopR {
        EzStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EzWriteStopR {
        EzWriteStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EzReadStopR {
        EzReadStopR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request when externally clocked selection is '1'. Only used when EC_AM is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WakeUpW<IntrSpiEcSpec> {
        WakeUpW::new(self, 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (SPI deselection). Only available in EZ and CMD_RESP mode and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_stop(&mut self) -> EzStopW<IntrSpiEcSpec> {
        EzStopW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (SPI deselection). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_write_stop(&mut self) -> EzWriteStopW<IntrSpiEcSpec> {
        EzWriteStopW::new(self, 2)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (SPI deselection). This event is an indication that a buffer memory location has been read from. Only used in EZ and CMD_RESP modes and when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_read_stop(&mut self) -> EzReadStopW<IntrSpiEcSpec> {
        EzReadStopW::new(self, 3)
    }
}
#[doc = "Externally clocked SPI interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_spi_ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_spi_ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpiEcSpec;
impl crate::RegisterSpec for IntrSpiEcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_spi_ec::R`](R) reader structure"]
impl crate::Readable for IntrSpiEcSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_spi_ec::W`](W) writer structure"]
impl crate::Writable for IntrSpiEcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SPI_EC to value 0"]
impl crate::Resettable for IntrSpiEcSpec {
    const RESET_VALUE: u32 = 0;
}
