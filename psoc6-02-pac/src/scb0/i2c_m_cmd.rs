#[doc = "Register `I2C_M_CMD` reader"]
pub type R = crate::R<I2cMCmdSpec>;
#[doc = "Register `I2C_M_CMD` writer"]
pub type W = crate::W<I2cMCmdSpec>;
#[doc = "Field `M_START` reader - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type MStartR = crate::BitReader;
#[doc = "Field `M_START` writer - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
pub type MStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_START_ON_IDLE` reader - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type MStartOnIdleR = crate::BitReader;
#[doc = "Field `M_START_ON_IDLE` writer - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
pub type MStartOnIdleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_ACK` reader - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
pub type MAckR = crate::BitReader;
#[doc = "Field `M_ACK` writer - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
pub type MAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_NACK` reader - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
pub type MNackR = crate::BitReader;
#[doc = "Field `M_NACK` writer - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
pub type MNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M_STOP` reader - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
pub type MStopR = crate::BitReader;
#[doc = "Field `M_STOP` writer - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
pub type MStopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start(&self) -> MStartR {
        MStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_start_on_idle(&self) -> MStartOnIdleR {
        MStartOnIdleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_ack(&self) -> MAckR {
        MAckR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    pub fn m_nack(&self) -> MNackR {
        MNackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    pub fn m_stop(&self) -> MStopR {
        MStopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', transmit a START or REPEATED START. Whether a START or REPEATED START is transmitted depends on the state of the master state machine. A START is only transmitted when the master state machine is in the default state. A REPEATED START is transmitted when the master state machine is not in the default state, but is working on an ongoing transaction. The REPEATED START can only be transmitted after a NACK or ACK has been received for a transmitted data element or after a NACK has been transmitted for a received data element. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_start(&mut self) -> MStartW<I2cMCmdSpec> {
        MStartW::new(self, 0)
    }
    #[doc = "Bit 1 - When '1', transmit a START as soon as the bus is idle (I2C_STATUS.BUS_BUSY is '0', note that BUSY has a default value of '0'). For bus idle detection the hardware relies on STOP detection. As a result, bus idle detection is only functional after at least one I2C bus transfer has been detected on the bus (default/reset value of BUSY is '0') . A START is only transmitted when the master state machine is in the default state. When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_start_on_idle(&mut self) -> MStartOnIdleW<I2cMCmdSpec> {
        MStartOnIdleW::new(self, 1)
    }
    #[doc = "Bit 2 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_ack(&mut self) -> MAckW<I2cMCmdSpec> {
        MAckW::new(self, 2)
    }
    #[doc = "Bit 3 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn m_nack(&mut self) -> MNackW<I2cMCmdSpec> {
        MNackW::new(self, 3)
    }
    #[doc = "Bit 4 - When '1', attempt to transmit a STOP. When this action is performed, the hardware sets this field to '0'. I2C_M_CMD.M_START has a higher priority than this command: in situations where both a STOP and a REPEATED START could be transmitted, M_START takes precedence over M_STOP."]
    #[inline(always)]
    #[must_use]
    pub fn m_stop(&mut self) -> MStopW<I2cMCmdSpec> {
        MStopW::new(self, 4)
    }
}
#[doc = "I2C master command\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_m_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_m_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cMCmdSpec;
impl crate::RegisterSpec for I2cMCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_m_cmd::R`](R) reader structure"]
impl crate::Readable for I2cMCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_m_cmd::W`](W) writer structure"]
impl crate::Writable for I2cMCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_M_CMD to value 0"]
impl crate::Resettable for I2cMCmdSpec {
    const RESET_VALUE: u32 = 0;
}
