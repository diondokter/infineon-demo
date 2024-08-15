#[doc = "Register `ERROR_INT_STAT_R` reader"]
pub type R = crate::R<ErrorIntStatRSpec>;
#[doc = "Register `ERROR_INT_STAT_R` writer"]
pub type W = crate::W<ErrorIntStatRSpec>;
#[doc = "Field `CMD_TOUT_ERR` reader - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type CmdToutErrR = crate::BitReader;
#[doc = "Field `CMD_TOUT_ERR` writer - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type CmdToutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_ERR` reader - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
pub type CmdCrcErrR = crate::BitReader;
#[doc = "Field `CMD_CRC_ERR` writer - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
pub type CmdCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_END_BIT_ERR` reader - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
pub type CmdEndBitErrR = crate::BitReader;
#[doc = "Field `CMD_END_BIT_ERR` writer - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
pub type CmdEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX_ERR` reader - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CmdIdxErrR = crate::BitReader;
#[doc = "Field `CMD_IDX_ERR` writer - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type CmdIdxErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TOUT_ERR` reader - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type DataToutErrR = crate::BitReader;
#[doc = "Field `DATA_TOUT_ERR` writer - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
pub type DataToutErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC_ERR` reader - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DataCrcErrR = crate::BitReader;
#[doc = "Field `DATA_CRC_ERR` writer - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DataCrcErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_END_BIT_ERR` reader - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DataEndBitErrR = crate::BitReader;
#[doc = "Field `DATA_END_BIT_ERR` writer - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type DataEndBitErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUR_LMT_ERR` reader - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
pub type CurLmtErrR = crate::BitReader;
#[doc = "Field `CUR_LMT_ERR` writer - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
pub type CurLmtErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_ERR` reader - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AutoCmdErrR = crate::BitReader;
#[doc = "Field `AUTO_CMD_ERR` writer - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AutoCmdErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA_ERR` reader - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AdmaErrR = crate::BitReader;
#[doc = "Field `ADMA_ERR` writer - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type AdmaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING_ERR` reader - N/A"]
pub type TuningErrR = crate::BitReader;
#[doc = "Field `TUNING_ERR` writer - N/A"]
pub type TuningErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR` reader - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type RespErrR = crate::BitReader;
#[doc = "Field `RESP_ERR` writer - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type RespErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ERR` reader - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type BootAckErrR = crate::BitReader;
#[doc = "Field `BOOT_ACK_ERR` writer - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
pub type BootAckErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn cmd_tout_err(&self) -> CmdToutErrR {
        CmdToutErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    pub fn cmd_crc_err(&self) -> CmdCrcErrR {
        CmdCrcErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    pub fn cmd_end_bit_err(&self) -> CmdEndBitErrR {
        CmdEndBitErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn cmd_idx_err(&self) -> CmdIdxErrR {
        CmdIdxErrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    pub fn data_tout_err(&self) -> DataToutErrR {
        DataToutErrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_crc_err(&self) -> DataCrcErrR {
        DataCrcErrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn data_end_bit_err(&self) -> DataEndBitErrR {
        DataEndBitErrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    pub fn cur_lmt_err(&self) -> CurLmtErrR {
        CurLmtErrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn auto_cmd_err(&self) -> AutoCmdErrR {
        AutoCmdErrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn adma_err(&self) -> AdmaErrR {
        AdmaErrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn tuning_err(&self) -> TuningErrR {
        TuningErrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn resp_err(&self) -> RespErrR {
        RespErrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn boot_ack_err(&self) -> BootAckErrR {
        BootAckErrR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Timeout Error In SD/eMMC Mode,this bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict, along with Command CRC Error bit, this bit is set to 1, without waiting for 64 SD/eMMC card clock cycles. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_tout_err(&mut self) -> CmdToutErrW<ErrorIntStatRSpec> {
        CmdToutErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Command CRC Error Command CRC Error is generated in SD/eMMC mode for following two cases. - If a response is returned and the Command Timeout Error is set to 0 (indicating no timeout), this bit is set to 1 when detecting a CRC error in the command response. - The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SD clock edge, then the Host Controller aborts the command (stop driving CMD line) and set this bit to 1. The Command Timeout Error is also set to 1 to distinguish a CMD line conflict. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): CRC error generated"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_err(&mut self) -> CmdCrcErrW<ErrorIntStatRSpec> {
        CmdCrcErrW::new(self, 1)
    }
    #[doc = "Bit 2 - Command End Bit Error This bit is set when detecting that the end bit of a command response is 0 in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): End Bit error generated"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_end_bit_err(&mut self) -> CmdEndBitErrW<ErrorIntStatRSpec> {
        CmdEndBitErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Command Index Error This bit is set if a Command Index error occurs in the command respons in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_err(&mut self) -> CmdIdxErrW<ErrorIntStatRSpec> {
        CmdIdxErrW::new(self, 3)
    }
    #[doc = "Bit 4 - Data Timeout Error This bit is set in SD/eMMC mode when detecting one of the following timeout conditions: - Busy timeout for R1b, R5b type - Busy timeout after Write CRC status - Write CRC Status timeout - Read Data timeout Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Time out"]
    #[inline(always)]
    #[must_use]
    pub fn data_tout_err(&mut self) -> DataToutErrW<ErrorIntStatRSpec> {
        DataToutErrW::new(self, 4)
    }
    #[doc = "Bit 5 - Data CRC Error This error occurs in SD/eMMC mode when detecting CRC error when transferring read data which uses the DAT line, when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_crc_err(&mut self) -> DataCrcErrW<ErrorIntStatRSpec> {
        DataCrcErrW::new(self, 5)
    }
    #[doc = "Bit 6 - Data End Bit Error This error occurs in SD/eMMC mode either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn data_end_bit_err(&mut self) -> DataEndBitErrW<ErrorIntStatRSpec> {
        DataEndBitErrW::new(self, 6)
    }
    #[doc = "Bit 7 - Current Limit Error By setting the SD Bus Power bit in the Power Control register, the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function, it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. DWC_mshc Host Controller does not support this function, this bit is always set to 0. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Power Fail"]
    #[inline(always)]
    #[must_use]
    pub fn cur_lmt_err(&mut self) -> CurLmtErrW<ErrorIntStatRSpec> {
        CurLmtErrW::new(self, 7)
    }
    #[doc = "Bit 8 - Auto CMD Error This error status is used by Auto CMD12 and Auto CMD23 in SD/eMMC mode. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_err(&mut self) -> AutoCmdErrW<ErrorIntStatRSpec> {
        AutoCmdErrW::new(self, 8)
    }
    #[doc = "Bit 9 - ADMA Error This bit is set when the Host Controller detects error during ADMA-based data transfer. The error could be due to following reasons: - Error response received from System bus (Master I/F) - ADMA3,ADMA2 Descriptors invalid - CQE Task or Transfer descriptors invalid When the error occurs, the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode: The Host Controller generates this Interrupt when it detects an invalid descriptor data (Valid=0) at the ST_FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST_FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn adma_err(&mut self) -> AdmaErrW<ErrorIntStatRSpec> {
        AdmaErrW::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_err(&mut self) -> TuningErrW<ErrorIntStatRSpec> {
        TuningErrW::new(self, 10)
    }
    #[doc = "Bit 11 - Response Error Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.This is applicable in SD/eMMC mode. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn resp_err(&mut self) -> RespErrW<ErrorIntStatRSpec> {
        RespErrW::new(self, 11)
    }
    #[doc = "Bit 12 - Boot Acknowledgement Error This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected in eMMC mode. In SD mode, this bit is irrelevant. Values: - 0x0 (FALSE): No error - 0x1 (TRUE): Error"]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_err(&mut self) -> BootAckErrW<ErrorIntStatRSpec> {
        BootAckErrW::new(self, 12)
    }
}
#[doc = "Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_int_stat_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_int_stat_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrorIntStatRSpec;
impl crate::RegisterSpec for ErrorIntStatRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`error_int_stat_r::R`](R) reader structure"]
impl crate::Readable for ErrorIntStatRSpec {}
#[doc = "`write(|w| ..)` method takes [`error_int_stat_r::W`](W) writer structure"]
impl crate::Writable for ErrorIntStatRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ERROR_INT_STAT_R to value 0"]
impl crate::Resettable for ErrorIntStatRSpec {
    const RESET_VALUE: u16 = 0;
}
