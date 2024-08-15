#[doc = "Register `FORCE_ERROR_INT_STAT_R` reader"]
pub type R = crate::R<ForceErrorIntStatRSpec>;
#[doc = "Register `FORCE_ERROR_INT_STAT_R` writer"]
pub type W = crate::W<ForceErrorIntStatRSpec>;
#[doc = "Field `FORCE_CMD_TOUT_ERR` reader - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
pub type ForceCmdToutErrR = crate::BitReader;
#[doc = "Field `FORCE_CMD_TOUT_ERR` writer - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
pub type ForceCmdToutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CMD_CRC_ERR` reader - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
pub type ForceCmdCrcErrR = crate::BitReader;
#[doc = "Field `FORCE_CMD_CRC_ERR` writer - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
pub type ForceCmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CMD_END_BIT_ERR` reader - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
pub type ForceCmdEndBitErrR = crate::BitReader;
#[doc = "Field `FORCE_CMD_END_BIT_ERR` writer - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
pub type ForceCmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CMD_IDX_ERR` reader - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
pub type ForceCmdIdxErrR = crate::BitReader;
#[doc = "Field `FORCE_CMD_IDX_ERR` writer - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
pub type ForceCmdIdxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DATA_TOUT_ERR` reader - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
pub type ForceDataToutErrR = crate::BitReader;
#[doc = "Field `FORCE_DATA_TOUT_ERR` writer - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
pub type ForceDataToutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DATA_CRC_ERR` reader - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
pub type ForceDataCrcErrR = crate::BitReader;
#[doc = "Field `FORCE_DATA_CRC_ERR` writer - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
pub type ForceDataCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DATA_END_BIT_ERR` reader - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
pub type ForceDataEndBitErrR = crate::BitReader;
#[doc = "Field `FORCE_DATA_END_BIT_ERR` writer - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
pub type ForceDataEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_CUR_LMT_ERR` reader - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
pub type ForceCurLmtErrR = crate::BitReader;
#[doc = "Field `FORCE_CUR_LMT_ERR` writer - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
pub type ForceCurLmtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_AUTO_CMD_ERR` reader - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
pub type ForceAutoCmdErrR = crate::BitReader;
#[doc = "Field `FORCE_AUTO_CMD_ERR` writer - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
pub type ForceAutoCmdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_ADMA_ERR` reader - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
pub type ForceAdmaErrR = crate::BitReader;
#[doc = "Field `FORCE_ADMA_ERR` writer - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
pub type ForceAdmaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_TUNING_ERR` reader - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
pub type ForceTuningErrR = crate::BitReader;
#[doc = "Field `FORCE_TUNING_ERR` writer - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
pub type ForceTuningErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_RESP_ERR` reader - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
pub type ForceRespErrR = crate::BitReader;
#[doc = "Field `FORCE_RESP_ERR` writer - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
pub type ForceRespErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_BOOT_ACK_ERR` reader - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
pub type ForceBootAckErrR = crate::BitReader;
#[doc = "Field `FORCE_BOOT_ACK_ERR` writer - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
pub type ForceBootAckErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VENDOR_ERR1` reader - N/A"]
pub type ForceVendorErr1R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR1` writer - N/A"]
pub type ForceVendorErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VENDOR_ERR2` reader - N/A"]
pub type ForceVendorErr2R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR2` writer - N/A"]
pub type ForceVendorErr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VENDOR_ERR3` reader - N/A"]
pub type ForceVendorErr3R = crate::BitReader;
#[doc = "Field `FORCE_VENDOR_ERR3` writer - N/A"]
pub type ForceVendorErr3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_tout_err(&self) -> ForceCmdToutErrR {
        ForceCmdToutErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_crc_err(&self) -> ForceCmdCrcErrR {
        ForceCmdCrcErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(&self) -> ForceCmdEndBitErrR {
        ForceCmdEndBitErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    pub fn force_cmd_idx_err(&self) -> ForceCmdIdxErrR {
        ForceCmdIdxErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    pub fn force_data_tout_err(&self) -> ForceDataToutErrR {
        ForceDataToutErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    pub fn force_data_crc_err(&self) -> ForceDataCrcErrR {
        ForceDataCrcErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    pub fn force_data_end_bit_err(&self) -> ForceDataEndBitErrR {
        ForceDataEndBitErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    pub fn force_cur_lmt_err(&self) -> ForceCurLmtErrR {
        ForceCurLmtErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    pub fn force_auto_cmd_err(&self) -> ForceAutoCmdErrR {
        ForceAutoCmdErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    pub fn force_adma_err(&self) -> ForceAdmaErrR {
        ForceAdmaErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    pub fn force_tuning_err(&self) -> ForceTuningErrR {
        ForceTuningErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    pub fn force_resp_err(&self) -> ForceRespErrR {
        ForceRespErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    pub fn force_boot_ack_err(&self) -> ForceBootAckErrR {
        ForceBootAckErrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err1(&self) -> ForceVendorErr1R {
        ForceVendorErr1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err2(&self) -> ForceVendorErr2R {
        ForceVendorErr2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn force_vendor_err3(&self) -> ForceVendorErr3R {
        ForceVendorErr3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Timeout Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_tout_err(&mut self) -> ForceCmdToutErrW<ForceErrorIntStatRSpec> {
        ForceCmdToutErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Force Event for Command CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command CRC Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_crc_err(&mut self) -> ForceCmdCrcErrW<ForceErrorIntStatRSpec> {
        ForceCmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Force Event for Command End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command End Bit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_end_bit_err(&mut self) -> ForceCmdEndBitErrW<ForceErrorIntStatRSpec> {
        ForceCmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Force Event for Command Index Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Command Index Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cmd_idx_err(&mut self) -> ForceCmdIdxErrW<ForceErrorIntStatRSpec> {
        ForceCmdIdxErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Event for Data Timeout Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data Timeout Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_tout_err(&mut self) -> ForceDataToutErrW<ForceErrorIntStatRSpec> {
        ForceDataToutErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Event for Data CRC Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data CRC Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_crc_err(&mut self) -> ForceDataCrcErrW<ForceErrorIntStatRSpec> {
        ForceDataCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Force Event for Data End Bit Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Data End Bit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_data_end_bit_err(&mut self) -> ForceDataEndBitErrW<ForceErrorIntStatRSpec> {
        ForceDataEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Force Event for Current Limit Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Current Limit Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_cur_lmt_err(&mut self) -> ForceCurLmtErrW<ForceErrorIntStatRSpec> {
        ForceCurLmtErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Force Event for Auto CMD Error (SD/eMMC Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Auto CMD Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_auto_cmd_err(&mut self) -> ForceAutoCmdErrW<ForceErrorIntStatRSpec> {
        ForceAutoCmdErrW::new(self, 8)
    }
    #[doc = "Bit 9 - Force Event for ADMA Error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): ADMA Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_adma_err(&mut self) -> ForceAdmaErrW<ForceErrorIntStatRSpec> {
        ForceAdmaErrW::new(self, 9)
    }
    #[doc = "Bit 10 - Force Event for Tuning Error (UHS-I Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Tuning Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_tuning_err(&mut self) -> ForceTuningErrW<ForceErrorIntStatRSpec> {
        ForceTuningErrW::new(self, 10)
    }
    #[doc = "Bit 11 - Force Event for Response Error (SD Mode only) Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Response Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_resp_err(&mut self) -> ForceRespErrW<ForceErrorIntStatRSpec> {
        ForceRespErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Force Event for Boot Ack error Values: - 0x0 (FALSE): Not Affected - 0x1 (TRUE): Boot ack Error Status is set"]
    #[inline(always)]
    #[must_use]
    pub fn force_boot_ack_err(&mut self) -> ForceBootAckErrW<ForceErrorIntStatRSpec> {
        ForceBootAckErrW::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err1(&mut self) -> ForceVendorErr1W<ForceErrorIntStatRSpec> {
        ForceVendorErr1W::new(self, 13)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err2(&mut self) -> ForceVendorErr2W<ForceErrorIntStatRSpec> {
        ForceVendorErr2W::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn force_vendor_err3(&mut self) -> ForceVendorErr3W<ForceErrorIntStatRSpec> {
        ForceVendorErr3W::new(self, 15)
    }
}
#[doc = "Force Event Register for Error Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`force_error_int_stat_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force_error_int_stat_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceErrorIntStatRSpec;
impl crate::RegisterSpec for ForceErrorIntStatRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`force_error_int_stat_r::R`](R) reader structure"]
impl crate::Readable for ForceErrorIntStatRSpec {}
#[doc = "`write(|w| ..)` method takes [`force_error_int_stat_r::W`](W) writer structure"]
impl crate::Writable for ForceErrorIntStatRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FORCE_ERROR_INT_STAT_R to value 0"]
impl crate::Resettable for ForceErrorIntStatRSpec {
    const RESET_VALUE: u16 = 0;
}
