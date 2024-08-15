#[doc = "Register `I2C_CTRL` reader"]
pub type R = crate::R<I2cCtrlSpec>;
#[doc = "Register `I2C_CTRL` writer"]
pub type W = crate::W<I2cCtrlSpec>;
#[doc = "Field `HIGH_PHASE_OVS` reader - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and &lt;= 16 IP clock cycles."]
pub type HighPhaseOvsR = crate::FieldReader;
#[doc = "Field `HIGH_PHASE_OVS` writer - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and &lt;= 16 IP clock cycles."]
pub type HighPhaseOvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOW_PHASE_OVS` reader - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and &lt;= 16 IP clock cycles."]
pub type LowPhaseOvsR = crate::FieldReader;
#[doc = "Field `LOW_PHASE_OVS` writer - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and &lt;= 16 IP clock cycles."]
pub type LowPhaseOvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `M_READY_DATA_ACK` reader - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub type MReadyDataAckR = crate::BitReader;
#[doc = "Field `M_READY_DATA_ACK` writer - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
pub type MReadyDataAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_NOT_READY_DATA_NACK` reader - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub type MNotReadyDataNackR = crate::BitReader;
#[doc = "Field `M_NOT_READY_DATA_NACK` writer - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
pub type MNotReadyDataNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_GENERAL_IGNORE` reader - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
pub type SGeneralIgnoreR = crate::BitReader;
#[doc = "Field `S_GENERAL_IGNORE` writer - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
pub type SGeneralIgnoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_READY_ADDR_ACK` reader - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub type SReadyAddrAckR = crate::BitReader;
#[doc = "Field `S_READY_ADDR_ACK` writer - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub type SReadyAddrAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_READY_DATA_ACK` reader - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub type SReadyDataAckR = crate::BitReader;
#[doc = "Field `S_READY_DATA_ACK` writer - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
pub type SReadyDataAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_NOT_READY_ADDR_NACK` reader - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
pub type SNotReadyAddrNackR = crate::BitReader;
#[doc = "Field `S_NOT_READY_ADDR_NACK` writer - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
pub type SNotReadyAddrNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_NOT_READY_DATA_NACK` reader - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type SNotReadyDataNackR = crate::BitReader;
#[doc = "Field `S_NOT_READY_DATA_NACK` writer - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
pub type SNotReadyDataNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLAVE_MODE` reader - Slave mode enabled ('1') or not ('0')."]
pub type SlaveModeR = crate::BitReader;
#[doc = "Field `SLAVE_MODE` writer - Slave mode enabled ('1') or not ('0')."]
pub type SlaveModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_MODE` reader - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub type MasterModeR = crate::BitReader;
#[doc = "Field `MASTER_MODE` writer - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
pub type MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and &lt;= 16 IP clock cycles."]
    #[inline(always)]
    pub fn high_phase_ovs(&self) -> HighPhaseOvsR {
        HighPhaseOvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and &lt;= 16 IP clock cycles."]
    #[inline(always)]
    pub fn low_phase_ovs(&self) -> LowPhaseOvsR {
        LowPhaseOvsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    pub fn m_ready_data_ack(&self) -> MReadyDataAckR {
        MReadyDataAckR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn m_not_ready_data_nack(&self) -> MNotReadyDataNackR {
        MNotReadyDataNackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    pub fn s_general_ignore(&self) -> SGeneralIgnoreR {
        SGeneralIgnoreR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_addr_ack(&self) -> SReadyAddrAckR {
        SReadyAddrAckR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    pub fn s_ready_data_ack(&self) -> SReadyDataAckR {
        SReadyDataAckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    pub fn s_not_ready_addr_nack(&self) -> SNotReadyAddrNackR {
        SNotReadyAddrNackR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    pub fn s_not_ready_data_nack(&self) -> SNotReadyDataNackR {
        SNotReadyDataNackR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SlaveModeR {
        SlaveModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    pub fn master_mode(&self) -> MasterModeR {
        MasterModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Serial I2C interface high phase oversampling factor. HIGH_PHASE_OVS + 1 peripheral clock periods constitute the high phase of a bit period. The valid range is \\[5, 15\\]
with input signal median filtering and \\[4, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular interface (IF) high time to guarantee functional correct behavior. With input signal median filtering, the IF high time should be >= 6 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF high time should be >= 5 IP clock cycles and &lt;= 16 IP clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn high_phase_ovs(&mut self) -> HighPhaseOvsW<I2cCtrlSpec> {
        HighPhaseOvsW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Serial I2C interface low phase oversampling factor. LOW_PHASE_OVS + 1 peripheral clock periods constitute the low phase of a bit period. The valid range is \\[7, 15\\]
with input signal median filtering and \\[6, 15\\]
without input signal median filtering. The field is only used in master mode. In slave mode, the field is NOT used. However, there is a frequency requirement for the IP clock wrt. the regular (no stretching) interface (IF) low time to guarantee functional correct behavior. With input signal median filtering, the IF low time should be >= 8 IP clock cycles and &lt;= 16 IP clock cycles. Without input signal median filtering, the IF low time should be >= 7 IP clock cycles and &lt;= 16 IP clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn low_phase_ovs(&mut self) -> LowPhaseOvsW<I2cCtrlSpec> {
        LowPhaseOvsW::new(self, 4)
    }
    #[doc = "Bit 8 - When '1', a received data element by the master is immediately ACK'd when the receiver FIFO is not full."]
    #[inline(always)]
    #[must_use]
    pub fn m_ready_data_ack(&mut self) -> MReadyDataAckW<I2cCtrlSpec> {
        MReadyDataAckW::new(self, 8)
    }
    #[doc = "Bit 9 - When '1', a received data element byte the master is immediately NACK'd when the receiver FIFO is full. When '0', clock stretching is used instead (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    #[must_use]
    pub fn m_not_ready_data_nack(&mut self) -> MNotReadyDataNackW<I2cCtrlSpec> {
        MNotReadyDataNackW::new(self, 9)
    }
    #[doc = "Bit 11 - When '1', a received general call slave address is immediately NACK'd (no ACK or clock stretching) and treated as a non matching slave address. This is useful for slaves that do not need any data supplied within the general call structure."]
    #[inline(always)]
    #[must_use]
    pub fn s_general_ignore(&mut self) -> SGeneralIgnoreW<I2cCtrlSpec> {
        SGeneralIgnoreW::new(self, 11)
    }
    #[doc = "Bit 12 - When '1', a received (matching) slave address is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn s_ready_addr_ack(&mut self) -> SReadyAddrAckW<I2cCtrlSpec> {
        SReadyAddrAckW::new(self, 12)
    }
    #[doc = "Bit 13 - When '1', a received data element by the slave is immediately ACK'd when the receiver FIFO is not full. In EZ mode, this field should be set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn s_ready_data_ack(&mut self) -> SReadyDataAckW<I2cCtrlSpec> {
        SReadyDataAckW::new(self, 13)
    }
    #[doc = "Bit 14 - For internally clocked logic (EC_AM is '0' and EC_OP is '0') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when: - EC_AM is '0', EC_OP is '0' and non EZ mode. Functionality is as follows: - 1: a received (matching) slave address is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full). For externally clocked logic (EC_AM is '1') on an address match or general call address (and S_GENERAL_IGNORE is '0'). Only used when (NOT used when EC_AM is '1' and EC_OP is '1' and address match and EZ mode): - EC_AM is '1' and EC_OP is '0'. - EC_AM is '1' and general call address match. - EC_AM is '1' and non EZ mode. Functionality is as follows: - 1: a received (matching or general) slave address is always immediately NACK'd. There are two possibilities: 1). the internally clocked logic is enabled (we are in Active system power mode) and it handles the rest of the current transfer. In this case the I2C master will not observe the NACK. 2). the internally clocked logic is not enabled (we are in DeepSleep system power mode). In this case the I2C master will observe the NACK and may retry the transfer in the future (which gives the internally clocked logic the time to wake up from DeepSleep system power mode). - 0: clock stretching is performed (till the internally clocked logic takes over). The internally clocked logic will handle the ongoing transfer as soon as it is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn s_not_ready_addr_nack(&mut self) -> SNotReadyAddrNackW<I2cCtrlSpec> {
        SNotReadyAddrNackW::new(self, 14)
    }
    #[doc = "Bit 15 - For internally clocked logic only. Only used when: - non EZ mode. Functionality is as follows: - 1: a received data element byte the slave is immediately NACK'd when the receiver FIFO is full. - 0: clock stretching is performed (till the receiver FIFO is no longer full)."]
    #[inline(always)]
    #[must_use]
    pub fn s_not_ready_data_nack(&mut self) -> SNotReadyDataNackW<I2cCtrlSpec> {
        SNotReadyDataNackW::new(self, 15)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only applicable in master/slave mode. When '0', the I2C SCL and SDA lines are connected to the I2C SCL and SDA pins. When '1', I2C SCL and SDA lines are routed internally in the peripheral, and as a result unaffected by other I2C devices. This allows a SCB I2C peripheral to address itself."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<I2cCtrlSpec> {
        LoopbackW::new(self, 16)
    }
    #[doc = "Bit 30 - Slave mode enabled ('1') or not ('0')."]
    #[inline(always)]
    #[must_use]
    pub fn slave_mode(&mut self) -> SlaveModeW<I2cCtrlSpec> {
        SlaveModeW::new(self, 30)
    }
    #[doc = "Bit 31 - Master mode enabled ('1') or not ('0'). Note that both master and slave modes can be enabled at the same time. This allows the IP to address itself."]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MasterModeW<I2cCtrlSpec> {
        MasterModeW::new(self, 31)
    }
}
#[doc = "I2C control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCtrlSpec;
impl crate::RegisterSpec for I2cCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for I2cCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_ctrl::W`](W) writer structure"]
impl crate::Writable for I2cCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_CTRL to value 0xfb88"]
impl crate::Resettable for I2cCtrlSpec {
    const RESET_VALUE: u32 = 0xfb88;
}
