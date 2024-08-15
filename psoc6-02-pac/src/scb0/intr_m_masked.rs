#[doc = "Register `INTR_M_MASKED` reader"]
pub type R = crate::R<IntrMMaskedSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_NACK` reader - Logical and of corresponding request and mask bits."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_ACK` reader - Logical and of corresponding request and mask bits."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `SPI_DONE` reader - Logical and of corresponding request and mask bits."]
pub type SpiDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2cArbLostR {
        I2cArbLostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2cNackR {
        I2cNackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2cAckR {
        I2cAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_done(&self) -> SpiDoneR {
        SpiDoneR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Master interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrMMaskedSpec;
impl crate::RegisterSpec for IntrMMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_m_masked::R`](R) reader structure"]
impl crate::Readable for IntrMMaskedSpec {}
#[doc = "`reset()` method sets INTR_M_MASKED to value 0"]
impl crate::Resettable for IntrMMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
