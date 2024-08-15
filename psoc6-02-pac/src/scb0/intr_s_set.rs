#[doc = "Register `INTR_S_SET` reader"]
pub type R = crate::R<IntrSSetSpec>;
#[doc = "Register `INTR_S_SET` writer"]
pub type W = crate::W<IntrSSetSpec>;
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
#[doc = "Field `I2C_WRITE_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cWriteStopR = crate::BitReader;
#[doc = "Field `I2C_WRITE_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_START` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStartR = crate::BitReader;
#[doc = "Field `I2C_START` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ADDR_MATCH` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cAddrMatchR = crate::BitReader;
#[doc = "Field `I2C_ADDR_MATCH` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_GENERAL` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cGeneralR = crate::BitReader;
#[doc = "Field `I2C_GENERAL` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cGeneralW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiEzWriteStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiEzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_STOP` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiEzStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_STOP` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiEzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BUS_ERROR` reader - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiBusErrorR = crate::BitReader;
#[doc = "Field `SPI_BUS_ERROR` writer - Write with '1' to set corresponding bit in interrupt request register."]
pub type SpiBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2cWriteStopR {
        I2cWriteStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2cStartR {
        I2cStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2cAddrMatchR {
        I2cAddrMatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2cGeneralR {
        I2cGeneralR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SpiEzWriteStopR {
        SpiEzWriteStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SpiEzStopR {
        SpiEzStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SpiBusErrorR {
        SpiBusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrSSetSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrSSetSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrSSetSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 3 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_write_stop(&mut self) -> I2cWriteStopW<IntrSSetSpec> {
        I2cWriteStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrSSetSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 5 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_start(&mut self) -> I2cStartW<IntrSSetSpec> {
        I2cStartW::new(self, 5)
    }
    #[doc = "Bit 6 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_addr_match(&mut self) -> I2cAddrMatchW<IntrSSetSpec> {
        I2cAddrMatchW::new(self, 6)
    }
    #[doc = "Bit 7 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_general(&mut self) -> I2cGeneralW<IntrSSetSpec> {
        I2cGeneralW::new(self, 7)
    }
    #[doc = "Bit 8 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrSSetSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_write_stop(&mut self) -> SpiEzWriteStopW<IntrSSetSpec> {
        SpiEzWriteStopW::new(self, 9)
    }
    #[doc = "Bit 10 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_stop(&mut self) -> SpiEzStopW<IntrSSetSpec> {
        SpiEzStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Write with '1' to set corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_bus_error(&mut self) -> SpiBusErrorW<IntrSSetSpec> {
        SpiBusErrorW::new(self, 11)
    }
}
#[doc = "Slave interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSSetSpec;
impl crate::RegisterSpec for IntrSSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_s_set::R`](R) reader structure"]
impl crate::Readable for IntrSSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_s_set::W`](W) writer structure"]
impl crate::Writable for IntrSSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_S_SET to value 0"]
impl crate::Resettable for IntrSSetSpec {
    const RESET_VALUE: u32 = 0;
}
