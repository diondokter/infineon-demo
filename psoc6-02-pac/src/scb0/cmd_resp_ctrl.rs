#[doc = "Register `CMD_RESP_CTRL` reader"]
pub type R = crate::R<CmdRespCtrlSpec>;
#[doc = "Register `CMD_RESP_CTRL` writer"]
pub type W = crate::W<CmdRespCtrlSpec>;
#[doc = "Field `BASE_RD_ADDR` reader - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BaseRdAddrR = crate::FieldReader<u16>;
#[doc = "Field `BASE_RD_ADDR` writer - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BaseRdAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BASE_WR_ADDR` reader - I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BaseWrAddrR = crate::FieldReader<u16>;
#[doc = "Field `BASE_WR_ADDR` writer - I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
pub type BaseWrAddrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_rd_addr(&self) -> BaseRdAddrR {
        BaseRdAddrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    pub fn base_wr_addr(&self) -> BaseWrAddrR {
        BaseWrAddrR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - I2C/SPI read base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode read transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode read transfer (CTRL.MODE is SPI): at the start of a read transfer BASE_RD_ADDR is copied to CMD_RESP_STATUS.CURR_RD_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    #[must_use]
    pub fn base_rd_addr(&mut self) -> BaseRdAddrW<CmdRespCtrlSpec> {
        BaseRdAddrW::new(self, 0)
    }
    #[doc = "Bits 16:24 - I2C/SPI write base address for CMD_RESP mode. Address is used by a I2C CMD_RESP mode write transfer (CTRL.MODE is I2C) or a SPI CMD_RESP mode write transfer (CTRL.MODE is SPI): at the start of a write transfer BASE_WE_ADDR is copied to CMD_RESP_STATUS.CURR_WR_ADDR. This field should not be modified during ongoing bus transfers."]
    #[inline(always)]
    #[must_use]
    pub fn base_wr_addr(&mut self) -> BaseWrAddrW<CmdRespCtrlSpec> {
        BaseWrAddrW::new(self, 16)
    }
}
#[doc = "Command/response control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_resp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_resp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRespCtrlSpec;
impl crate::RegisterSpec for CmdRespCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_resp_ctrl::R`](R) reader structure"]
impl crate::Readable for CmdRespCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_resp_ctrl::W`](W) writer structure"]
impl crate::Writable for CmdRespCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_RESP_CTRL to value 0"]
impl crate::Resettable for CmdRespCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
