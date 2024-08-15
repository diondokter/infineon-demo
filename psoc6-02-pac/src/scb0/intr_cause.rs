#[doc = "Register `INTR_CAUSE` reader"]
pub type R = crate::R<IntrCauseSpec>;
#[doc = "Field `M` reader - Master interrupt active ('interrupt_master'): INTR_M_MASKED != 0."]
pub type MR = crate::BitReader;
#[doc = "Field `S` reader - Slave interrupt active ('interrupt_slave'): INTR_S_MASKED != 0."]
pub type SR = crate::BitReader;
#[doc = "Field `TX` reader - Transmitter interrupt active ('interrupt_tx'): INTR_TX_MASKED != 0."]
pub type TxR = crate::BitReader;
#[doc = "Field `RX` reader - Receiver interrupt active ('interrupt_rx'): INTR_RX_MASKED != 0."]
pub type RxR = crate::BitReader;
#[doc = "Field `I2C_EC` reader - Externally clock I2C interrupt active ('interrupt_i2c_ec'): INTR_I2C_EC_MASKED != 0."]
pub type I2cEcR = crate::BitReader;
#[doc = "Field `SPI_EC` reader - Externally clocked SPI interrupt active ('interrupt_spi_ec'): INTR_SPI_EC_MASKED != 0."]
pub type SpiEcR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master interrupt active ('interrupt_master'): INTR_M_MASKED != 0."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave interrupt active ('interrupt_slave'): INTR_S_MASKED != 0."]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter interrupt active ('interrupt_tx'): INTR_TX_MASKED != 0."]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver interrupt active ('interrupt_rx'): INTR_RX_MASKED != 0."]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Externally clock I2C interrupt active ('interrupt_i2c_ec'): INTR_I2C_EC_MASKED != 0."]
    #[inline(always)]
    pub fn i2c_ec(&self) -> I2cEcR {
        I2cEcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Externally clocked SPI interrupt active ('interrupt_spi_ec'): INTR_SPI_EC_MASKED != 0."]
    #[inline(always)]
    pub fn spi_ec(&self) -> SpiEcR {
        SpiEcR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Active clocked interrupt signal\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCauseSpec;
impl crate::RegisterSpec for IntrCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause::R`](R) reader structure"]
impl crate::Readable for IntrCauseSpec {}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for IntrCauseSpec {
    const RESET_VALUE: u32 = 0;
}
