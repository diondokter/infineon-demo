#[doc = "Register `INTR_I2C_EC` reader"]
pub type R = crate::R<IntrI2cEcSpec>;
#[doc = "Register `INTR_I2C_EC` writer"]
pub type W = crate::W<IntrI2cEcSpec>;
#[doc = "Field `WAKE_UP` reader - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `WAKE_UP` writer - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
pub type WakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_STOP` reader - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzStopR = crate::BitReader;
#[doc = "Field `EZ_STOP` writer - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_WRITE_STOP` reader - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzWriteStopR = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` writer - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_READ_STOP` reader - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzReadStopR = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` writer - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
pub type EzReadStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EzStopR {
        EzStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EzWriteStopR {
        EzWriteStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EzReadStopR {
        EzReadStopR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wake up request. Active on incoming slave request (with address match). Only used when EC_AM is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WakeUpW<IntrI2cEcSpec> {
        WakeUpW::new(self, 0)
    }
    #[doc = "Bit 1 - STOP detection. Activated on the end of a every transfer (I2C STOP). Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_stop(&mut self) -> EzStopW<IntrI2cEcSpec> {
        EzStopW::new(self, 1)
    }
    #[doc = "Bit 2 - STOP detection after a write transfer occurred. Activated on the end of a write transfer (I2C STOP). This event is an indication that a buffer memory location has been written to. For EZ mode: a transfer that only writes the base address does NOT activate this event. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_write_stop(&mut self) -> EzWriteStopW<IntrI2cEcSpec> {
        EzWriteStopW::new(self, 2)
    }
    #[doc = "Bit 3 - STOP detection after a read transfer occurred. Activated on the end of a read transfer (I2C STOP). This event is an indication that a buffer memory location has been read from. Only available for a slave request with an address match, in EZ and CMD_RESP modes, when EC_OP is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_read_stop(&mut self) -> EzReadStopW<IntrI2cEcSpec> {
        EzReadStopW::new(self, 3)
    }
}
#[doc = "Externally clocked I2C interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_i2c_ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrI2cEcSpec;
impl crate::RegisterSpec for IntrI2cEcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_i2c_ec::R`](R) reader structure"]
impl crate::Readable for IntrI2cEcSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_i2c_ec::W`](W) writer structure"]
impl crate::Writable for IntrI2cEcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_I2C_EC to value 0"]
impl crate::Resettable for IntrI2cEcSpec {
    const RESET_VALUE: u32 = 0;
}
