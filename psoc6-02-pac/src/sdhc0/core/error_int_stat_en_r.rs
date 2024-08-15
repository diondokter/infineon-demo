#[doc = "Register `ERROR_INT_STAT_EN_R` reader"]
pub type R = crate::R<ErrorIntStatEnRSpec>;
#[doc = "Register `ERROR_INT_STAT_EN_R` writer"]
pub type W = crate::W<ErrorIntStatEnRSpec>;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` reader - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdToutErrStatEnR = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR_STAT_EN` writer - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdToutErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` reader - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCrcErrStatEnR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_STAT_EN` writer - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCrcErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` reader - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdEndBitErrStatEnR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_STAT_EN` writer - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdEndBitErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` reader - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdIdxErrStatEnR = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR_STAT_EN` writer - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdIdxErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` reader - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataToutErrStatEnR = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR_STAT_EN` writer - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataToutErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` reader - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataCrcErrStatEnR = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR_STAT_EN` writer - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataCrcErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` reader - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataEndBitErrStatEnR = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR_STAT_EN` writer - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataEndBitErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` reader - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CurLmtErrStatEnR = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR_STAT_EN` writer - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CurLmtErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` reader - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AutoCmdErrStatEnR = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR_STAT_EN` writer - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AutoCmdErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR_STAT_EN` reader - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AdmaErrStatEnR = crate::BitReader;
#[doc = "Field `ADMA_ERR_STAT_EN` writer - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AdmaErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_ERR_STAT_EN` reader - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type TuningErrStatEnR = crate::BitReader;
#[doc = "Field `TUNING_ERR_STAT_EN` writer - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type TuningErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_STAT_EN` reader - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RespErrStatEnR = crate::BitReader;
#[doc = "Field `RESP_ERR_STAT_EN` writer - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RespErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` reader - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BootAckErrStatEnR = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR_STAT_EN` writer - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BootAckErrStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN1` reader - N/A"]
pub type VendorErrStatEn1R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN1` writer - N/A"]
pub type VendorErrStatEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN2` reader - N/A"]
pub type VendorErrStatEn2R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN2` writer - N/A"]
pub type VendorErrStatEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_STAT_EN3` reader - N/A"]
pub type VendorErrStatEn3R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_STAT_EN3` writer - N/A"]
pub type VendorErrStatEn3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_stat_en(&self) -> CmdToutErrStatEnR {
        CmdToutErrStatEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_stat_en(&self) -> CmdCrcErrStatEnR {
        CmdCrcErrStatEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(&self) -> CmdEndBitErrStatEnR {
        CmdEndBitErrStatEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_idx_err_stat_en(&self) -> CmdIdxErrStatEnR {
        CmdIdxErrStatEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_stat_en(&self) -> DataToutErrStatEnR {
        DataToutErrStatEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_stat_en(&self) -> DataCrcErrStatEnR {
        DataCrcErrStatEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_stat_en(&self) -> DataEndBitErrStatEnR {
        DataEndBitErrStatEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_stat_en(&self) -> CurLmtErrStatEnR {
        CurLmtErrStatEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_stat_en(&self) -> AutoCmdErrStatEnR {
        AutoCmdErrStatEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_stat_en(&self) -> AdmaErrStatEnR {
        AdmaErrStatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn tuning_err_stat_en(&self) -> TuningErrStatEnR {
        TuningErrStatEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_stat_en(&self) -> RespErrStatEnR {
        RespErrStatEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_stat_en(&self) -> BootAckErrStatEnR {
        BootAckErrStatEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en1(&self) -> VendorErrStatEn1R {
        VendorErrStatEn1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en2(&self) -> VendorErrStatEn2R {
        VendorErrStatEn2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_stat_en3(&self) -> VendorErrStatEn3R {
        VendorErrStatEn3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err_stat_en(&mut self) -> CmdToutErrStatEnW<ErrorIntStatEnRSpec> {
        CmdToutErrStatEnW::new(self, 0)
    }
    #[doc = "Bit 1 - ommand CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_stat_en(&mut self) -> CmdCrcErrStatEnW<ErrorIntStatEnRSpec> {
        CmdCrcErrStatEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_stat_en(&mut self) -> CmdEndBitErrStatEnW<ErrorIntStatEnRSpec> {
        CmdEndBitErrStatEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err_stat_en(&mut self) -> CmdIdxErrStatEnW<ErrorIntStatEnRSpec> {
        CmdIdxErrStatEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err_stat_en(&mut self) -> DataToutErrStatEnW<ErrorIntStatEnRSpec> {
        DataToutErrStatEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Status Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_stat_en(&mut self) -> DataCrcErrStatEnW<ErrorIntStatEnRSpec> {
        DataCrcErrStatEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_stat_en(&mut self) -> DataEndBitErrStatEnW<ErrorIntStatEnRSpec> {
        DataEndBitErrStatEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err_stat_en(&mut self) -> CurLmtErrStatEnW<ErrorIntStatEnRSpec> {
        CurLmtErrStatEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error Status Enable (SD/eMMC Mode only). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err_stat_en(&mut self) -> AutoCmdErrStatEnW<ErrorIntStatEnRSpec> {
        AutoCmdErrStatEnW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_stat_en(&mut self) -> AdmaErrStatEnW<ErrorIntStatEnRSpec> {
        AdmaErrStatEnW::new(self, 9)
    }
    #[doc = "Bit 10 - Tuning Error Status Enable (UHS-I Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err_stat_en(&mut self) -> TuningErrStatEnW<ErrorIntStatEnRSpec> {
        TuningErrStatEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Response Error Status Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_stat_en(&mut self) -> RespErrStatEnW<ErrorIntStatEnRSpec> {
        RespErrStatEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only) Setting this bit to 1 enables setting of Boot Acknowledgment Error in Error Interrupt Status register (ERROR_INT_STAT_R). Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err_stat_en(&mut self) -> BootAckErrStatEnW<ErrorIntStatEnRSpec> {
        BootAckErrStatEnW::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en1(&mut self) -> VendorErrStatEn1W<ErrorIntStatEnRSpec> {
        VendorErrStatEn1W::new(self, 13)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en2(&mut self) -> VendorErrStatEn2W<ErrorIntStatEnRSpec> {
        VendorErrStatEn2W::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_stat_en3(&mut self) -> VendorErrStatEn3W<ErrorIntStatEnRSpec> {
        VendorErrStatEn3W::new(self, 15)
    }
}
#[doc = "Error Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_stat_en_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_stat_en_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorIntStatEnRSpec;
impl crate::RegisterSpec for ErrorIntStatEnRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`error_int_stat_en_r::R`](R) reader structure"]
impl crate::Readable for ErrorIntStatEnRSpec {}
#[doc = "`write(|w| ..)` method takes [`error_int_stat_en_r::W`](W) writer structure"]
impl crate::Writable for ErrorIntStatEnRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ERROR_INT_STAT_EN_R to value 0"]
impl crate::Resettable for ErrorIntStatEnRSpec {
    const RESET_VALUE: u16 = 0;
}
