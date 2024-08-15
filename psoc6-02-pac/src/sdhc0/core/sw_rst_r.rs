#[doc = "Register `SW_RST_R` reader"]
pub type R = crate::R<SwRstRSpec>;
#[doc = "Register `SW_RST_R` writer"]
pub type W = crate::W<SwRstRSpec>;
#[doc = "Field `SW_RST_ALL` reader - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstAllR = crate::BitReader;
#[doc = "Field `SW_RST_ALL` writer - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstAllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_CMD` reader - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstCmdR = crate::BitReader;
#[doc = "Field `SW_RST_CMD` writer - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_RST_DAT` reader - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstDatR = crate::BitReader;
#[doc = "Field `SW_RST_DAT` writer - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
pub type SwRstDatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_all(&self) -> SwRstAllR {
        SwRstAllR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_cmd(&self) -> SwRstCmdR {
        SwRstCmdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    pub fn sw_rst_dat(&self) -> SwRstDatR {
        SwRstDatR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset For All This reset affects the entire Host Controller except for the card detection circuit. During its initialization, the Host Driver sets this bit to 1 to reset the Host Controller. All registers are reset except the capabilities register. If this bit is set to 1, the Host Driver must issue reset command and reinitialize the card. Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_all(&mut self) -> SwRstAllW<SwRstRSpec> {
        SwRstAllW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset For CMD line This bit resets only a part of the command circuit to be able to issue a command. This reset is effective only for a command issuing circuit (including response error statuses related to Command Inhibit (CMD) control) and does not affect the data transfer circuit. Host Controller can continue data transfer even after this reset is executed while handling subcommand-response errors. The following registers and bits are cleared by this bit: - Present State register - Command Inhibit (CMD) bit - Normal Interrupt Status register - Command Complete bit - Error Interrupt Status - Response error statuses related to Command Inhibit (CMD) bit Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_cmd(&mut self) -> SwRstCmdW<SwRstRSpec> {
        SwRstCmdW::new(self, 1)
    }
    #[doc = "Bit 2 - Software Reset For DAT line This bit is used in SD/eMMC mode and it resets only a part of the data circuit and the DMA circuit is also reset. The following registers and bits are cleared by this bit: - Buffer Data Port register - Buffer is cleared and initialized. - Present state register - Buffer Read Enable - Buffer Write Enable - Read Transfer Active - Write Transfer Active - DAT Line Active - Command Inhibit (DAT) - Block Gap Control register - Continue Request - Stop At Block Gap Request - Normal Interrupt status register - Buffer Read Ready - Buffer Write Ready - DMA Interrupt - Block Gap Event - Transfer Complete Values: - 0x0 (FALSE): Work - 0x1 (TRUE): Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sw_rst_dat(&mut self) -> SwRstDatW<SwRstRSpec> {
        SwRstDatW::new(self, 2)
    }
}
#[doc = "Software Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_rst_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_rst_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwRstRSpec;
impl crate::RegisterSpec for SwRstRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sw_rst_r::R`](R) reader structure"]
impl crate::Readable for SwRstRSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_rst_r::W`](W) writer structure"]
impl crate::Writable for SwRstRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SW_RST_R to value 0"]
impl crate::Resettable for SwRstRSpec {
    const RESET_VALUE: u8 = 0;
}
