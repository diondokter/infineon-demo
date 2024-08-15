#[doc = "Register `INTR_M` reader"]
pub type R = crate::R<IntrMSpec>;
#[doc = "Register `INTR_M` writer"]
pub type W = crate::W<IntrMSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` writer - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
pub type I2cArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK` reader - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_NACK` writer - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
pub type I2cNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ACK` reader - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_ACK` writer - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
pub type I2cAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - I2C master bus error (unexpected detection of START or STOP condition)."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - I2C master bus error (unexpected detection of START or STOP condition)."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DONE` reader - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
pub type SpiDoneR = crate::BitReader;
#[doc = "Field `SPI_DONE` writer - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
pub type SpiDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2cArbLostR {
        I2cArbLostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2cNackR {
        I2cNackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2cAckR {
        I2cAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
    #[inline(always)]
    pub fn spi_done(&self) -> SpiDoneR {
        SpiDoneR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C master lost arbitration: the value driven by the master on the SDA line is not the same as the value observed on the SDA line."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrMSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C master negative acknowledgement. Set to '1', when the master receives a NACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrMSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C master acknowledgement. Set to '1', when the master receives a ACK (typically after the master transmitted the slave address or TX data)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrMSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 4 - I2C master STOP. Set to '1', when the master has transmitted a STOP."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrMSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 8 - I2C master bus error (unexpected detection of START or STOP condition)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrMSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - SPI master transfer done event: all data frames in the transmit FIFO are sent, the transmit FIFO is empty (both TX FIFO and transmit shifter register are empty), and SPI select output pin is deselected."]
    #[inline(always)]
    #[must_use]
    pub fn spi_done(&mut self) -> SpiDoneW<IntrMSpec> {
        SpiDoneW::new(self, 9)
    }
}
#[doc = "Master interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMSpec;
impl crate::RegisterSpec for IntrMSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_m::R`](R) reader structure"]
impl crate::Readable for IntrMSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_m::W`](W) writer structure"]
impl crate::Writable for IntrMSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_M to value 0"]
impl crate::Resettable for IntrMSpec {
    const RESET_VALUE: u32 = 0;
}
