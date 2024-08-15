#[doc = "Register `INTR_I2C_EC_MASK` reader"]
pub type R = crate::R<IntrI2cEcMaskSpec>;
#[doc = "Register `INTR_I2C_EC_MASK` writer"]
pub type W = crate::W<IntrI2cEcMaskSpec>;
#[doc = "Field `WAKE_UP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type WakeUpR = crate::BitReader;
#[doc = "Field `WAKE_UP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type WakeUpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EzStopR = crate::BitReader;
#[doc = "Field `EZ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EzWriteStopR = crate::BitReader;
#[doc = "Field `EZ_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_READ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type EzReadStopR = crate::BitReader;
#[doc = "Field `EZ_READ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type EzReadStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn wake_up(&self) -> WakeUpR {
        WakeUpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_stop(&self) -> EzStopR {
        EzStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_write_stop(&self) -> EzWriteStopR {
        EzWriteStopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn ez_read_stop(&self) -> EzReadStopR {
        EzReadStopR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn wake_up(&mut self) -> WakeUpW<IntrI2cEcMaskSpec> {
        WakeUpW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ez_stop(&mut self) -> EzStopW<IntrI2cEcMaskSpec> {
        EzStopW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ez_write_stop(&mut self) -> EzWriteStopW<IntrI2cEcMaskSpec> {
        EzWriteStopW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn ez_read_stop(&mut self) -> EzReadStopW<IntrI2cEcMaskSpec> {
        EzReadStopW::new(self, 3)
    }
}
#[doc = "Externally clocked I2C interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_i2c_ec_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrI2cEcMaskSpec;
impl crate::RegisterSpec for IntrI2cEcMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_i2c_ec_mask::R`](R) reader structure"]
impl crate::Readable for IntrI2cEcMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_i2c_ec_mask::W`](W) writer structure"]
impl crate::Writable for IntrI2cEcMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_I2C_EC_MASK to value 0"]
impl crate::Resettable for IntrI2cEcMaskSpec {
    const RESET_VALUE: u32 = 0;
}
