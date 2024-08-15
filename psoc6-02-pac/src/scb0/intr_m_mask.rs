#[doc = "Register `INTR_M_MASK` reader"]
pub type R = crate::R<IntrMMaskSpec>;
#[doc = "Register `INTR_M_MASK` writer"]
pub type W = crate::W<IntrMMaskSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_NACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ACK` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_ACK` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DONE` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SpiDoneR = crate::BitReader;
#[doc = "Field `SPI_DONE` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SpiDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2cArbLostR {
        I2cArbLostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2cNackR {
        I2cNackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2cAckR {
        I2cAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_done(&self) -> SpiDoneR {
        SpiDoneR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrMMaskSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrMMaskSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrMMaskSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrMMaskSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrMMaskSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_done(&mut self) -> SpiDoneW<IntrMMaskSpec> {
        SpiDoneW::new(self, 9)
    }
}
#[doc = "Master interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMMaskSpec;
impl crate::RegisterSpec for IntrMMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_m_mask::R`](R) reader structure"]
impl crate::Readable for IntrMMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_m_mask::W`](W) writer structure"]
impl crate::Writable for IntrMMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_M_MASK to value 0"]
impl crate::Resettable for IntrMMaskSpec {
    const RESET_VALUE: u32 = 0;
}
