#[doc = "Register `MSHC_CTRL_R` reader"]
pub type R = crate::R<MshcCtrlRSpec>;
#[doc = "Register `MSHC_CTRL_R` writer"]
pub type W = crate::W<MshcCtrlRSpec>;
#[doc = "Field `CMD_CONFLICT_CHECK` reader - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
pub type CmdConflictCheckR = crate::BitReader;
#[doc = "Field `CMD_CONFLICT_CHECK` writer - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
pub type CmdConflictCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CG_DIS` reader - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
pub type SwCgDisR = crate::BitReader;
#[doc = "Field `SW_CG_DIS` writer - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
pub type SwCgDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    pub fn cmd_conflict_check(&self) -> CmdConflictCheckR {
        CmdConflictCheckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    pub fn sw_cg_dis(&self) -> SwCgDisR {
        SwCgDisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command conflict check This bit enables command conflict check. Note: DWC_mshc controller monitors the CMD line whenever a command is issued and checks whether the value driven on sd_cmd_out matches the value on sd_cmd_in at next subsequent edge of cclk_tx to determine command conflict error. This bit is cleared only if the feed back delay (including IO Pad delay) is more than (t_card_clk_period - t_setup), where t_setup is the setup time of a flop in DWC_mshc. The I/O pad delay is consistent across CMD and DATA lines, and it is within the value: (2*t_card_clk_period - t_setup) Values: - 0x0 (DISABLE_CMD_CONFLICT_CHK): Disable command conflict check - 0x1 (CMD_CONFLICT_CHK_LAT1): Check for command conflict after 1 card clock cycle"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_conflict_check(&mut self) -> CmdConflictCheckW<MshcCtrlRSpec> {
        CmdConflictCheckW::new(self, 0)
    }
    #[doc = "Bit 4 - Internal clock gating disable control This bit must be used to disable IP's internal clock gating when required. when disabled clocks are not gated. Clocks to the core (except hclk) must be stopped when programming this bit. Values: - 0x0 (ENABLE): Internal clock gates are active and clock gating is controlled internally - 0x1 (DISABLE): Internal clock gating is disabled, clocks are not gated internally"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cg_dis(&mut self) -> SwCgDisW<MshcCtrlRSpec> {
        SwCgDisW::new(self, 4)
    }
}
#[doc = "MSHC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mshc_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mshc_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MshcCtrlRSpec;
impl crate::RegisterSpec for MshcCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mshc_ctrl_r::R`](R) reader structure"]
impl crate::Readable for MshcCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`mshc_ctrl_r::W`](W) writer structure"]
impl crate::Writable for MshcCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MSHC_CTRL_R to value 0x01"]
impl crate::Resettable for MshcCtrlRSpec {
    const RESET_VALUE: u8 = 0x01;
}
