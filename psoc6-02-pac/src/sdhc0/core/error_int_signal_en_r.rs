#[doc = "Register `ERROR_INT_SIGNAL_EN_R` reader"]
pub type R = crate::R<ErrorIntSignalEnRSpec>;
#[doc = "Register `ERROR_INT_SIGNAL_EN_R` writer"]
pub type W = crate::W<ErrorIntSignalEnRSpec>;
#[doc = "Field `CMD_TOUT_ERR_SIGNAL_EN` reader - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdToutErrSignalEnR = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR_SIGNAL_EN` writer - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdToutErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR_SIGNAL_EN` reader - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCrcErrSignalEnR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR_SIGNAL_EN` writer - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCrcErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR_SIGNAL_EN` reader - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdEndBitErrSignalEnR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR_SIGNAL_EN` writer - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdEndBitErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX_ERR_SIGNAL_EN` reader - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CmdIdxErrSignalEnR = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR_SIGNAL_EN` writer - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CmdIdxErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TOUT_ERR_SIGNAL_EN` reader - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataToutErrSignalEnR = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR_SIGNAL_EN` writer - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataToutErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC_ERR_SIGNAL_EN` reader - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataCrcErrSignalEnR = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR_SIGNAL_EN` writer - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataCrcErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT_ERR_SIGNAL_EN` reader - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataEndBitErrSignalEnR = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR_SIGNAL_EN` writer - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DataEndBitErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LMT_ERR_SIGNAL_EN` reader - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CurLmtErrSignalEnR = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR_SIGNAL_EN` writer - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CurLmtErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_ERR_SIGNAL_EN` reader - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AutoCmdErrSignalEnR = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR_SIGNAL_EN` writer - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AutoCmdErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR_SIGNAL_EN` reader - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AdmaErrSignalEnR = crate::BitReader;
#[doc = "Field `ADMA_ERR_SIGNAL_EN` writer - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type AdmaErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_ERR_SIGNAL_EN` reader - N/A"]
pub type TuningErrSignalEnR = crate::BitReader;
#[doc = "Field `TUNING_ERR_SIGNAL_EN` writer - N/A"]
pub type TuningErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_SIGNAL_EN` reader - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RespErrSignalEnR = crate::BitReader;
#[doc = "Field `RESP_ERR_SIGNAL_EN` writer - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type RespErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ERR_SIGNAL_EN` reader - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BootAckErrSignalEnR = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR_SIGNAL_EN` writer - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BootAckErrSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN1` reader - N/A"]
pub type VendorErrSignalEn1R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN1` writer - N/A"]
pub type VendorErrSignalEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN2` reader - N/A"]
pub type VendorErrSignalEn2R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN2` writer - N/A"]
pub type VendorErrSignalEn2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN3` reader - N/A"]
pub type VendorErrSignalEn3R = crate::BitReader;
#[doc = "Field `VENDOR_ERR_SIGNAL_EN3` writer - N/A"]
pub type VendorErrSignalEn3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_tout_err_signal_en(&self) -> CmdToutErrSignalEnR {
        CmdToutErrSignalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(&self) -> CmdCrcErrSignalEnR {
        CmdCrcErrSignalEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(&self) -> CmdEndBitErrSignalEnR {
        CmdEndBitErrSignalEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(&self) -> CmdIdxErrSignalEnR {
        CmdIdxErrSignalEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(&self) -> DataToutErrSignalEnR {
        DataToutErrSignalEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(&self) -> DataCrcErrSignalEnR {
        DataCrcErrSignalEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(&self) -> DataEndBitErrSignalEnR {
        DataEndBitErrSignalEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(&self) -> CurLmtErrSignalEnR {
        CurLmtErrSignalEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(&self) -> AutoCmdErrSignalEnR {
        AutoCmdErrSignalEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn adma_err_signal_en(&self) -> AdmaErrSignalEnR {
        AdmaErrSignalEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err_signal_en(&self) -> TuningErrSignalEnR {
        TuningErrSignalEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn resp_err_signal_en(&self) -> RespErrSignalEnR {
        RespErrSignalEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(&self) -> BootAckErrSignalEnR {
        BootAckErrSignalEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en1(&self) -> VendorErrSignalEn1R {
        VendorErrSignalEn1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en2(&self) -> VendorErrSignalEn2R {
        VendorErrSignalEn2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn vendor_err_signal_en3(&self) -> VendorErrSignalEn3R {
        VendorErrSignalEn3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err_signal_en(&mut self) -> CmdToutErrSignalEnW<ErrorIntSignalEnRSpec> {
        CmdToutErrSignalEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err_signal_en(&mut self) -> CmdCrcErrSignalEnW<ErrorIntSignalEnRSpec> {
        CmdCrcErrSignalEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err_signal_en(&mut self) -> CmdEndBitErrSignalEnW<ErrorIntSignalEnRSpec> {
        CmdEndBitErrSignalEnW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err_signal_en(&mut self) -> CmdIdxErrSignalEnW<ErrorIntSignalEnRSpec> {
        CmdIdxErrSignalEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err_signal_en(&mut self) -> DataToutErrSignalEnW<ErrorIntSignalEnRSpec> {
        DataToutErrSignalEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err_signal_en(&mut self) -> DataCrcErrSignalEnW<ErrorIntSignalEnRSpec> {
        DataCrcErrSignalEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err_signal_en(&mut self) -> DataEndBitErrSignalEnW<ErrorIntSignalEnRSpec> {
        DataEndBitErrSignalEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err_signal_en(&mut self) -> CurLmtErrSignalEnW<ErrorIntSignalEnRSpec> {
        CurLmtErrSignalEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error Signal Enable (SD/eMMC Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err_signal_en(&mut self) -> AutoCmdErrSignalEnW<ErrorIntSignalEnRSpec> {
        AutoCmdErrSignalEnW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err_signal_en(&mut self) -> AdmaErrSignalEnW<ErrorIntSignalEnRSpec> {
        AdmaErrSignalEnW::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err_signal_en(&mut self) -> TuningErrSignalEnW<ErrorIntSignalEnRSpec> {
        TuningErrSignalEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Response Error Signal Enable (SD Mode only) Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_signal_en(&mut self) -> RespErrSignalEnW<ErrorIntSignalEnRSpec> {
        RespErrSignalEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Boot Acknowledgment Error (eMMC Mode only). Setting this bit to 1 enables generating interrupt signal when Boot Acknowledgement Error in Error Interrupt Status register is set. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err_signal_en(&mut self) -> BootAckErrSignalEnW<ErrorIntSignalEnRSpec> {
        BootAckErrSignalEnW::new(self, 12)
    }
    #[doc = "Bit 13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en1(&mut self) -> VendorErrSignalEn1W<ErrorIntSignalEnRSpec> {
        VendorErrSignalEn1W::new(self, 13)
    }
    #[doc = "Bit 14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en2(&mut self) -> VendorErrSignalEn2W<ErrorIntSignalEnRSpec> {
        VendorErrSignalEn2W::new(self, 14)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn vendor_err_signal_en3(&mut self) -> VendorErrSignalEn3W<ErrorIntSignalEnRSpec> {
        VendorErrSignalEn3W::new(self, 15)
    }
}
#[doc = "Error Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_signal_en_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_signal_en_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorIntSignalEnRSpec;
impl crate::RegisterSpec for ErrorIntSignalEnRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`error_int_signal_en_r::R`](R) reader structure"]
impl crate::Readable for ErrorIntSignalEnRSpec {}
#[doc = "`write(|w| ..)` method takes [`error_int_signal_en_r::W`](W) writer structure"]
impl crate::Writable for ErrorIntSignalEnRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ERROR_INT_SIGNAL_EN_R to value 0"]
impl crate::Resettable for ErrorIntSignalEnRSpec {
    const RESET_VALUE: u16 = 0;
}
