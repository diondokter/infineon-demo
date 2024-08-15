#[doc = "Register `INTR_I2C_EC_MASKED` reader"]
pub type R = crate::R<IntrI2cEcMaskedSpec>;
#[doc = "Field `WAKE_UP` reader - Logical and of corresponding request and mask bits."]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EzStopR = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EzWriteStopR = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type EzReadStopR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EzStopR {
        EzStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EzWriteStopR {
        EzWriteStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EzReadStopR {
        EzReadStopR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Externally clocked I2C interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrI2cEcMaskedSpec;
impl crate::RegisterSpec for IntrI2cEcMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_i2c_ec_masked::R`](R) reader structure"]
impl crate::Readable for IntrI2cEcMaskedSpec {}
#[doc = "`reset()` method sets INTR_I2C_EC_MASKED to value 0"]
impl crate::Resettable for IntrI2cEcMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
