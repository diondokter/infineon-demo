#[doc = "Register `INTR_S` reader"]
pub type R = crate::R<IntrSSpec>;
#[doc = "Register `INTR_S` writer"]
pub type W = crate::W<IntrSSpec>;
#[doc = "Field `I2C_ARB_LOST` reader - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2cArbLostR = crate::BitReader;
#[doc = "Field `I2C_ARB_LOST` writer - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2cArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_NACK` reader - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub type I2cNackR = crate::BitReader;
#[doc = "Field `I2C_NACK` writer - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
pub type I2cNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ACK` reader - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub type I2cAckR = crate::BitReader;
#[doc = "Field `I2C_ACK` writer - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
pub type I2cAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_WRITE_STOP` reader - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
pub type I2cWriteStopR = crate::BitReader;
#[doc = "Field `I2C_WRITE_STOP` writer - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
pub type I2cWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_STOP` reader - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
pub type I2cStopR = crate::BitReader;
#[doc = "Field `I2C_STOP` writer - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
pub type I2cStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_START` reader - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub type I2cStartR = crate::BitReader;
#[doc = "Field `I2C_START` writer - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
pub type I2cStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_ADDR_MATCH` reader - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2cAddrMatchR = crate::BitReader;
#[doc = "Field `I2C_ADDR_MATCH` writer - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2cAddrMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_GENERAL` reader - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2cGeneralR = crate::BitReader;
#[doc = "Field `I2C_GENERAL` writer - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
pub type I2cGeneralW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_BUS_ERROR` reader - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2cBusErrorR = crate::BitReader;
#[doc = "Field `I2C_BUS_ERROR` writer - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type I2cBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_WRITE_STOP` reader - SPI slave deselected after a write EZ SPI transfer occurred."]
pub type SpiEzWriteStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_WRITE_STOP` writer - SPI slave deselected after a write EZ SPI transfer occurred."]
pub type SpiEzWriteStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_EZ_STOP` reader - SPI slave deselected after any EZ SPI transfer occurred."]
pub type SpiEzStopR = crate::BitReader;
#[doc = "Field `SPI_EZ_STOP` writer - SPI slave deselected after any EZ SPI transfer occurred."]
pub type SpiEzStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_BUS_ERROR` reader - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SpiBusErrorR = crate::BitReader;
#[doc = "Field `SPI_BUS_ERROR` writer - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
pub type SpiBusErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_arb_lost(&self) -> I2cArbLostR {
        I2cArbLostR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_nack(&self) -> I2cNackR {
        I2cNackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    pub fn i2c_ack(&self) -> I2cAckR {
        I2cAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    pub fn i2c_write_stop(&self) -> I2cWriteStopR {
        I2cWriteStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    pub fn i2c_stop(&self) -> I2cStopR {
        I2cStopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    pub fn i2c_start(&self) -> I2cStartR {
        I2cStartR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_addr_match(&self) -> I2cAddrMatchR {
        I2cAddrMatchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    pub fn i2c_general(&self) -> I2cGeneralR {
        I2cGeneralR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn i2c_bus_error(&self) -> I2cBusErrorR {
        I2cBusErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_write_stop(&self) -> SpiEzWriteStopR {
        SpiEzWriteStopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    pub fn spi_ez_stop(&self) -> SpiEzStopR {
        SpiEzStopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    pub fn spi_bus_error(&self) -> SpiBusErrorR {
        SpiBusErrorR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C slave lost arbitration: the value driven on the SDA line is not the same as the value observed on the SDA line (while the SCL line is '1'). This should not occur, it represents erroneous I2C bus behavior. In case of lost arbitration, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_arb_lost(&mut self) -> I2cArbLostW<IntrSSpec> {
        I2cArbLostW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C slave negative acknowledgement received. Set to '1', when the slave receives a NACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_nack(&mut self) -> I2cNackW<IntrSSpec> {
        I2cNackW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C slave acknowledgement received. Set to '1', when the slave receives a ACK (typically after the slave transmitted TX data)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_ack(&mut self) -> I2cAckW<IntrSSpec> {
        I2cAckW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C STOP event for I2C write transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. In non EZ mode, the event is detected on any I2C write transfer intended for this slave. Note that a I2C write address intended for the slave (address is matching and a it is a write transfer) will result in a I2C_WRITE_STOP event independent of whether the I2C address is ACK'd or NACK'd. In EZ mode, the event is detected only on I2C write transfers that have EZ data written to the memory structure (an I2C write transfer that only communicates an I2C address and EZ address, will not result in this event being detected)."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_write_stop(&mut self) -> I2cWriteStopW<IntrSSpec> {
        I2cWriteStopW::new(self, 3)
    }
    #[doc = "Bit 4 - I2C STOP event for I2C (read or write) transfer intended for this slave (address matching is performed). Set to '1', when STOP or REPEATED START event is detected. The REPEATED START event is included in this interrupt cause such that the I2C transfers separated by a REPEATED START can be distinguished and potentially treated separately by the Firmware. Note that the second I2C transfer (after a REPEATED START) may be to a different slave address. The event is detected on any I2C transfer intended for this slave. Note that a I2C address intended for the slave (address is matching) will result in a I2C_STOP event independent of whether the I2C address is ACK'd or NACK'd."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_stop(&mut self) -> I2cStopW<IntrSSpec> {
        I2cStopW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C slave START received. Set to '1', when START or REPEATED START event is detected. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') AND clock stretching is performed (till the internally clocked logic takes over) (I2C_CTRL.S_NOT_READY_ADDR_NACK is '0'), this field is NOT set. The Firmware should use INTR_S_EC.WAKE_UP, INTR_S.I2C_ADDR_MATCH and INTR_S.I2C_GENERAL."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_start(&mut self) -> I2cStartW<IntrSSpec> {
        I2cStartW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C slave matching address received. If CTRL.ADDR_ACCEPT, the received address (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_addr_match(&mut self) -> I2cAddrMatchW<IntrSSpec> {
        I2cAddrMatchW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C slave general call address received. If CTRL.ADDR_ACCEPT, the received address 0x00 (including the R/W bit) is available in the RX FIFO. In the case of externally clocked address matching (CTRL.EC_AM_MODE is '1') and internally clocked operation (CTRL.EC_OP_MODE is '0'), this field is set when the event is detected."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_general(&mut self) -> I2cGeneralW<IntrSSpec> {
        I2cGeneralW::new(self, 7)
    }
    #[doc = "Bit 8 - I2C slave bus error (unexpected detection of START or STOP condition). This should not occur, it represents erroneous I2C bus behavior. In case of a bus error, the I2C slave state machine abort the ongoing transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_bus_error(&mut self) -> I2cBusErrorW<IntrSSpec> {
        I2cBusErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - SPI slave deselected after a write EZ SPI transfer occurred."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_write_stop(&mut self) -> SpiEzWriteStopW<IntrSSpec> {
        SpiEzWriteStopW::new(self, 9)
    }
    #[doc = "Bit 10 - SPI slave deselected after any EZ SPI transfer occurred."]
    #[inline(always)]
    #[must_use]
    pub fn spi_ez_stop(&mut self) -> SpiEzStopW<IntrSSpec> {
        SpiEzStopW::new(self, 10)
    }
    #[doc = "Bit 11 - SPI slave deselected at an unexpected time in the SPI transfer. The Firmware may decide to clear the TX and RX FIFOs in case of this error."]
    #[inline(always)]
    #[must_use]
    pub fn spi_bus_error(&mut self) -> SpiBusErrorW<IntrSSpec> {
        SpiBusErrorW::new(self, 11)
    }
}
#[doc = "Slave interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSSpec;
impl crate::RegisterSpec for IntrSSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_s::R`](R) reader structure"]
impl crate::Readable for IntrSSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_s::W`](W) writer structure"]
impl crate::Writable for IntrSSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_S to value 0"]
impl crate::Resettable for IntrSSpec {
    const RESET_VALUE: u32 = 0;
}
