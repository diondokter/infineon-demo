#[doc = "Register `I2C_STATUS` reader"]
pub type R = crate::R<I2cStatusSpec>;
#[doc = "Field `BUS_BUSY` reader - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
pub type BusBusyR = crate::BitReader;
#[doc = "Field `I2C_EC_BUSY` reader - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
pub type I2cEcBusyR = crate::BitReader;
#[doc = "Field `S_READ` reader - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
pub type SReadR = crate::BitReader;
#[doc = "Field `M_READ` reader - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
pub type MReadR = crate::BitReader;
#[doc = "Field `CURR_EZ_ADDR` reader - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
pub type CurrEzAddrR = crate::FieldReader;
#[doc = "Field `BASE_EZ_ADDR` reader - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
pub type BaseEzAddrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - I2C bus is busy. The bus is considered busy ('1'), from the time a START is detected or from the time the SCL line is '0'. The bus is considered idle ('0'), from the time a STOP is detected. If the IP is disabled, BUS_BUSY is '0'. After enabling the IP, it takes time for the BUS_BUSY to detect a busy bus. This time is the maximum high time of the SCL line. For a 100 kHz interface frequency, this maximum high time may last roughly 5 us (half a bit period). For single master systems, BUS_BUSY does not have to be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START (no bus collisions). For multi-master systems, BUS_BUSY can be used to detect an idle bus before a master starts a transfer using I2C_M_CMD.M_START_ON_IDLE (to prevent bus collisions)."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BusBusyR {
        BusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_EZ_ADDR or CURR_EZ_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_EZ_ADDR and CURR_EZ_ADDR are reliable."]
    #[inline(always)]
    pub fn i2c_ec_busy(&self) -> I2cEcBusyR {
        I2cEcBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C slave read transfer ('1') or I2C slave write transfer ('0'). When the I2C slave is inactive/idle or receiving START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn s_read(&self) -> SReadR {
        SReadR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C master read transfer ('1') or I2C master write transfer ('0'). When the I2C master is inactive/idle or transmitting START, REPEATED START, STOP or an address, this field is '0''."]
    #[inline(always)]
    pub fn m_read(&self) -> MReadR {
        MReadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:15 - I2C slave current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when I2C_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CurrEzAddrR {
        CurrEzAddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - I2C slave base EZ address. Address as provided by an I2C write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BaseEzAddrR {
        BaseEzAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "I2C status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cStatusSpec;
impl crate::RegisterSpec for I2cStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_status::R`](R) reader structure"]
impl crate::Readable for I2cStatusSpec {}
#[doc = "`reset()` method sets I2C_STATUS to value 0"]
impl crate::Resettable for I2cStatusSpec {
    const RESET_VALUE: u32 = 0;
}
