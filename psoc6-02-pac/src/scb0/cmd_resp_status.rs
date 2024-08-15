#[doc = "Register `CMD_RESP_STATUS` reader"]
pub type R = crate::R<CmdRespStatusSpec>;
#[doc = "Field `CURR_RD_ADDR` reader - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub type CurrRdAddrR = crate::FieldReader<u16>;
#[doc = "Field `CURR_WR_ADDR` reader - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
pub type CurrWrAddrR = crate::FieldReader<u16>;
#[doc = "Field `CMD_RESP_EC_BUS_BUSY` reader - Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
pub type CmdRespEcBusBusyR = crate::BitReader;
#[doc = "Field `CMD_RESP_EC_BUSY` reader - Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (when CMD_RESP_EC_BUSY is '0') or not reliable (when CMD_RESP_EC_BUSY is '1'). Note: - When there is no ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable). - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable), when the CURR_RD_ADDR and CURR_WR_ADDR are not being updated by the HW. - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '1' (not reliable), when the CURR_RD_ADDR or CURR_WR_ADDR are being updated by the HW. Note that this update lasts one I2C clock cycle, or two SPI clock cycles."]
pub type CmdRespEcBusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been read (# bytes = CURR_RD_ADDR - CMD_RESP_CTRL.BASE_RD_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_rd_addr(&self) -> CurrRdAddrR {
        CurrRdAddrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write current address for CMD_RESP mode. HW increments the field after a read access to the memory buffer. However, when the last memory buffer address is reached, the address is NOT incremented (but remains at the maximum memory buffer address). The field is used to determine how many bytes have been written (# bytes = CURR_WR_ADDR - CMD_RESP_CTRL.BASE_WR_ADDR). This field is reliable during when there is no bus transfer. This field is potentially unreliable when there is a bus transfer bus transfer: when CMD_RESP_EC_BUSY is '0', the field is reliable."]
    #[inline(always)]
    pub fn curr_wr_addr(&self) -> CurrWrAddrR {
        CurrWrAddrR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Indicates whether there is an ongoing bus transfer to the IP. '0': no ongoing bus transfer. '1': ongoing bus transfer. For SPI, the field is '1' when the slave is selected. For I2C, the field is set to '1' at a I2C START/RESTART. In case of an address match, the field is set to '0' on a I2C STOP. In case of NO address match, the field is set to '0' after the failing address match."]
    #[inline(always)]
    pub fn cmd_resp_ec_bus_busy(&self) -> CmdRespEcBusBusyR {
        CmdRespEcBusBusyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Indicates whether the CURR_RD_ADDR and CURR_WR_ADDR fields in this register are reliable (when CMD_RESP_EC_BUSY is '0') or not reliable (when CMD_RESP_EC_BUSY is '1'). Note: - When there is no ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable). - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '0' (reliable), when the CURR_RD_ADDR and CURR_WR_ADDR are not being updated by the HW. - When there is a ongoing bus transfer, CMD_RESP_EC_BUSY is '1' (not reliable), when the CURR_RD_ADDR or CURR_WR_ADDR are being updated by the HW. Note that this update lasts one I2C clock cycle, or two SPI clock cycles."]
    #[inline(always)]
    pub fn cmd_resp_ec_busy(&self) -> CmdRespEcBusyR {
        CmdRespEcBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Command/response status\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_resp_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRespStatusSpec;
impl crate::RegisterSpec for CmdRespStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_resp_status::R`](R) reader structure"]
impl crate::Readable for CmdRespStatusSpec {}
#[doc = "`reset()` method sets CMD_RESP_STATUS to value 0"]
impl crate::Resettable for CmdRespStatusSpec {
    const RESET_VALUE: u32 = 0;
}
