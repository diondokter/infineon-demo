#[doc = "Register `INTR_S_MASKED` reader"]
pub type R = crate::R<IntrSMaskedSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - Logical and of corresponding request and mask bits."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_NACK` reader - Logical and of corresponding request and mask bits."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_ACK` reader - Logical and of corresponding request and mask bits."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2cWriteStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` reader - Logical and of corresponding request and mask bits."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_START` reader - Logical and of corresponding request and mask bits."]
pub type I2cStartR = crate::BitReader;
#[doc = "Field `I2C_ADDR_MATCH` reader - Logical and of corresponding request and mask bits."]
pub type I2cAddrMatchR = crate::BitReader;
#[doc = "Field `I2C_GENERAL` reader - Logical and of corresponding request and mask bits."]
pub type I2cGeneralR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - Logical and of corresponding request and mask bits."]
pub type SpiEzWriteStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_STOP` reader - Logical and of corresponding request and mask bits."]
pub type SpiEzStopR = crate::BitReader;
#[doc = "Field `SPI_BUS_ERROR` reader - Logical and of corresponding request and mask bits."]
pub type SpiBusErrorR = crate::BitReader;
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
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2cWriteStopR {
        I2cWriteStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2cStartR {
        I2cStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2cAddrMatchR {
        I2cAddrMatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2cGeneralR {
        I2cGeneralR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SpiEzWriteStopR {
        SpiEzWriteStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SpiEzStopR {
        SpiEzStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SpiBusErrorR {
        SpiBusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Slave interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSMaskedSpec;
impl crate::RegisterSpec for IntrSMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_s_masked::R`](R) reader structure"]
impl crate::Readable for IntrSMaskedSpec {}
#[doc = "`reset()` method sets INTR_S_MASKED to value 0"]
impl crate::Resettable for IntrSMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
