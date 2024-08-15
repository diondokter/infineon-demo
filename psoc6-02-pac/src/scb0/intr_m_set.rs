#[doc = "Register `INTR_M_SET` reader"]
pub type R = crate::R<IntrMSetSpec>;
#[doc = "Register `INTR_M_SET` writer"]
pub type W = crate::W<IntrMSetSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_NACK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ACK` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_ACK` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DONE` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiDoneR = crate::BitReader;
#[doc = "Field `SPI_DONE` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2cArbLostR {
        I2cArbLostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2cNackR {
        I2cNackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2cAckR {
        I2cAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_done(&self) -> SpiDoneR {
        SpiDoneR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrMSetSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrMSetSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrMSetSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrMSetSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrMSetSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_done(&mut self) -> SpiDoneW<IntrMSetSpec> {
        SpiDoneW::new(self, 9)
    }
}
#[doc = "Master interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMSetSpec;
impl crate::RegisterSpec for IntrMSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_m_set::R`](R) reader structure"]
impl crate::Readable for IntrMSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_m_set::W`](W) writer structure"]
impl crate::Writable for IntrMSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_M_SET to value 0"]
impl crate::Resettable for IntrMSetSpec {
    const RESET_VALUE: u32 = 0;
}
