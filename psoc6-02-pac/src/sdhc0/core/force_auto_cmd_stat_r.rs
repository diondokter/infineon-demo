#[doc = "Register `FORCE_AUTO_CMD_STAT_R` writer"]
pub type W = crate::W<ForceAutoCmdStatRSpec>;
#[doc = "Field `FORCE_AUTO_CMD12_NOT_EXEC` writer - Force Event for Auto CMD12 Not Executed Values: - 0x1 (TRUE): Auto CMD12 Not Executed Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmd12NotExecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_TOUT_ERR` writer - Force Event for Auto CMD Timeout Error Values: - 0x1 (TRUE): Auto CMD Timeout Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmdToutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_CRC_ERR` writer - Force Event for Auto CMD CRC Error Values: - 0x1 (TRUE): Auto CMD CRC Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_EBIT_ERR` writer - Force Event for Auto CMD End Bit Error Values: - 0x1 (TRUE): Auto CMD End Bit Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmdEbitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_IDX_ERR` writer - Force Event for Auto CMD Index Error Values: - 0x1 (TRUE): Auto CMD Index Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmdIdxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_RESP_ERR` writer - Force Event for Auto CMD Response Error Values: - 0x1 (TRUE): Auto CMD Response Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceAutoCmdRespErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CMD_NOT_ISSUED_AUTO_CMD12` writer - Force Event for Command Not Issued By Auto CMD12 Error Values: - 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set - 0x0 (FALSE): Not Affected"]
pub type ForceCmdNotIssuedAutoCmd12W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Force Event for Auto CMD12 Not Executed Values: - 0x1 (TRUE): Auto CMD12 Not Executed Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd12_not_exec(&mut self) -> ForceAutoCmd12NotExecW<ForceAutoCmdStatRSpec> {
        ForceAutoCmd12NotExecW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error Values: - 0x1 (TRUE): Auto CMD Timeout Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_tout_err(&mut self) -> ForceAutoCmdToutErrW<ForceAutoCmdStatRSpec> {
        ForceAutoCmdToutErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error Values: - 0x1 (TRUE): Auto CMD CRC Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_crc_err(&mut self) -> ForceAutoCmdCrcErrW<ForceAutoCmdStatRSpec> {
        ForceAutoCmdCrcErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error Values: - 0x1 (TRUE): Auto CMD End Bit Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_ebit_err(&mut self) -> ForceAutoCmdEbitErrW<ForceAutoCmdStatRSpec> {
        ForceAutoCmdEbitErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error Values: - 0x1 (TRUE): Auto CMD Index Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_idx_err(&mut self) -> ForceAutoCmdIdxErrW<ForceAutoCmdStatRSpec> {
        ForceAutoCmdIdxErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Event for Auto CMD Response Error Values: - 0x1 (TRUE): Auto CMD Response Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_resp_err(&mut self) -> ForceAutoCmdRespErrW<ForceAutoCmdStatRSpec> {
        ForceAutoCmdRespErrW::new(self, 5)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error Values: - 0x1 (TRUE): Command Not Issued By Auto CMD12 Error Status is set - 0x0 (FALSE): Not Affected"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_not_issued_auto_cmd12(
        &mut self,
    ) -> ForceCmdNotIssuedAutoCmd12W<ForceAutoCmdStatRSpec> {
        ForceCmdNotIssuedAutoCmd12W::new(self, 7)
    }
}
#[doc = "Force Event Register for Auto CMD Error Status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_auto_cmd_stat_r::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceAutoCmdStatRSpec;
impl crate::RegisterSpec for ForceAutoCmdStatRSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`force_auto_cmd_stat_r::W`](W) writer structure"]
impl crate::Writable for ForceAutoCmdStatRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FORCE_AUTO_CMD_STAT_R to value 0"]
impl crate::Resettable for ForceAutoCmdStatRSpec {
    const RESET_VALUE: u16 = 0;
}
