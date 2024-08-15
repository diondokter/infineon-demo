#[doc = "Register `I2C_S_CMD` reader"]
pub type R = crate::R<I2cSCmdSpec>;
#[doc = "Register `I2C_S_CMD` writer"]
pub type W = crate::W<I2cSCmdSpec>;
#[doc = "Field `S_ACK` reader - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
pub type SAckR = crate::BitReader;
#[doc = "Field `S_ACK` writer - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
pub type SAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_NACK` reader - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type SNackR = crate::BitReader;
#[doc = "Field `S_NACK` writer - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
pub type SNackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    pub fn s_ack(&self) -> SAckR {
        SAckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    pub fn s_nack(&self) -> SNackR {
        SNackR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When '1', attempt to transmit an acknowledgement (ACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode)."]
    #[inline(always)]
    #[must_use]
    pub fn s_ack(&mut self) -> SAckW<I2cSCmdSpec> {
        SAckW::new(self, 0)
    }
    #[doc = "Bit 1 - When '1', attempt to transmit a negative acknowledgement (NACK). When this action is performed, the hardware sets this field to '0'. In EZ mode, this field should be set to '0' (it is only to be used in non EZ mode). This command has a higher priority than I2C_S_CMD.S_ACK, I2C_CTRL.S_READY_ADDR_ACK or I2C_CTRL.S_READY_DATA_ACK."]
    #[inline(always)]
    #[must_use]
    pub fn s_nack(&mut self) -> SNackW<I2cSCmdSpec> {
        SNackW::new(self, 1)
    }
}
#[doc = "I2C slave command\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_s_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_s_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cSCmdSpec;
impl crate::RegisterSpec for I2cSCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_s_cmd::R`](R) reader structure"]
impl crate::Readable for I2cSCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_s_cmd::W`](W) writer structure"]
impl crate::Writable for I2cSCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_S_CMD to value 0"]
impl crate::Resettable for I2cSCmdSpec {
    const RESET_VALUE: u32 = 0;
}
