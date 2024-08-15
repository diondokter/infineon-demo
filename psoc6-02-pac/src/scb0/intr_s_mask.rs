#[doc = "Register `INTR_S_MASK` reader"]
pub type R = crate::R<IntrSMaskSpec>;
#[doc = "Register `INTR_S_MASK` writer"]
pub type W = crate::W<IntrSMaskSpec>;
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
#[doc = "Field `I2C_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cWriteStopR = crate::BitReader;
#[doc = "Field `I2C_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_START` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStartR = crate::BitReader;
#[doc = "Field `I2C_START` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ADDR_MATCH` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cAddrMatchR = crate::BitReader;
#[doc = "Field `I2C_ADDR_MATCH` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_GENERAL` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cGeneralR = crate::BitReader;
#[doc = "Field `I2C_GENERAL` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cGeneralW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SpiEzWriteStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SpiEzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_STOP` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SpiEzStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_STOP` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SpiEzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BUS_ERROR` reader - Mask bit for corresponding bit in interrupt request register."]
pub type SpiBusErrorR = crate::BitReader;
#[doc = "Field `SPI_BUS_ERROR` writer - Mask bit for corresponding bit in interrupt request register."]
pub type SpiBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2cWriteStopR {
        I2cWriteStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2cStartR {
        I2cStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2cAddrMatchR {
        I2cAddrMatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2cGeneralR {
        I2cGeneralR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SpiEzWriteStopR {
        SpiEzWriteStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SpiEzStopR {
        SpiEzStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SpiBusErrorR {
        SpiBusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrSMaskSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrSMaskSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrSMaskSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_write_stop(&mut self) -> I2cWriteStopW<IntrSMaskSpec> {
        I2cWriteStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrSMaskSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 5 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_start(&mut self) -> I2cStartW<IntrSMaskSpec> {
        I2cStartW::new(self, 5)
    }
    #[doc = "Bit 6 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_addr_match(&mut self) -> I2cAddrMatchW<IntrSMaskSpec> {
        I2cAddrMatchW::new(self, 6)
    }
    #[doc = "Bit 7 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_general(&mut self) -> I2cGeneralW<IntrSMaskSpec> {
        I2cGeneralW::new(self, 7)
    }
    #[doc = "Bit 8 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrSMaskSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_write_stop(&mut self) -> SpiEzWriteStopW<IntrSMaskSpec> {
        SpiEzWriteStopW::new(self, 9)
    }
    #[doc = "Bit 10 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_stop(&mut self) -> SpiEzStopW<IntrSMaskSpec> {
        SpiEzStopW::new(self, 10)
    }
    #[doc = "Bit 11 - Mask bit for corresponding bit in interrupt request register."]
    #[inline(always)]
    #[must_use]
    pub fn spi_bus_error(&mut self) -> SpiBusErrorW<IntrSMaskSpec> {
        SpiBusErrorW::new(self, 11)
    }
}
#[doc = "Slave interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSMaskSpec;
impl crate::RegisterSpec for IntrSMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_s_mask::R`](R) reader structure"]
impl crate::Readable for IntrSMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_s_mask::W`](W) writer structure"]
impl crate::Writable for IntrSMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_S_MASK to value 0"]
impl crate::Resettable for IntrSMaskSpec {
    const RESET_VALUE: u32 = 0;
}
