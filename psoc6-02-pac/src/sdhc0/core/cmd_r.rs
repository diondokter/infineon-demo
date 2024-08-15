#[doc = "Register `CMD_R` reader"]
pub type R = crate::R<CmdRSpec>;
#[doc = "Register `CMD_R` writer"]
pub type W = crate::W<CmdRSpec>;
#[doc = "Field `RESP_TYPE_SELECT` reader - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RespTypeSelectR = crate::FieldReader;
#[doc = "Field `RESP_TYPE_SELECT` writer - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
pub type RespTypeSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUB_CMD_FLAG` reader - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
pub type SubCmdFlagR = crate::BitReader;
#[doc = "Field `SUB_CMD_FLAG` writer - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
pub type SubCmdFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_CHK_ENABLE` reader - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CmdCrcChkEnableR = crate::BitReader;
#[doc = "Field `CMD_CRC_CHK_ENABLE` writer - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CmdCrcChkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_IDX_CHK_ENABLE` reader - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CmdIdxChkEnableR = crate::BitReader;
#[doc = "Field `CMD_IDX_CHK_ENABLE` writer - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
pub type CmdIdxChkEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_PRESENT_SEL` reader - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
pub type DataPresentSelR = crate::BitReader;
#[doc = "Field `DATA_PRESENT_SEL` writer - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
pub type DataPresentSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TYPE` reader - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
pub type CmdTypeR = crate::FieldReader;
#[doc = "Field `CMD_TYPE` writer - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
pub type CmdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD_INDEX` reader - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    pub fn resp_type_select(&self) -> RespTypeSelectR {
        RespTypeSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    pub fn sub_cmd_flag(&self) -> SubCmdFlagR {
        SubCmdFlagR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(&self) -> CmdCrcChkEnableR {
        CmdCrcChkEnableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(&self) -> CmdIdxChkEnableR {
        CmdIdxChkEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    pub fn data_present_sel(&self) -> DataPresentSelR {
        DataPresentSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Response Type Select This bit indicates the type of response expected from the card. Values: - 0x0 (NO_RESP): No Response - 0x1 (RESP_LEN_136): Response Length 136 - 0x2 (RESP_LEN_48): Response Length 48 - 0x3 (RESP_LEN_48B): Response Length 48; Check Busy after response"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type_select(&mut self) -> RespTypeSelectW<CmdRSpec> {
        RespTypeSelectW::new(self, 0)
    }
    #[doc = "Bit 2 - Sub Command Flag This bit distinguishes between a main command and a sub command. Values: - 0x0 (MAIN): Main Command - 0x1 (SUB): Sub Command"]
    #[inline(always)]
    #[must_use]
    pub fn sub_cmd_flag(&mut self) -> SubCmdFlagW<CmdRSpec> {
        SubCmdFlagW::new(self, 2)
    }
    #[doc = "Bit 3 - Command CRC Check Enable This bit enables the Host Controller to check the CRC field in the response. If an error is detected, it is reported as a Command CRC error. Note: - CRC Check enable must be set to 0 for the command with no response, R3 response, and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_chk_enable(&mut self) -> CmdCrcChkEnableW<CmdRSpec> {
        CmdCrcChkEnableW::new(self, 3)
    }
    #[doc = "Bit 4 - Command Index Check Enable This bit enables the Host Controller to check the index field in the response to verify if it has the same value as the command index. If the value is not the same, it is reported as a Command Index error. Note: - Index Check enable must be set to 0 for the command with no response, R2 response, R3 response and R4 response. Values: - 0x0 (DISABLED): Disable - 0x1 (ENABLED): Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_idx_chk_enable(&mut self) -> CmdIdxChkEnableW<CmdRSpec> {
        CmdIdxChkEnableW::new(self, 4)
    }
    #[doc = "Bit 5 - Data Present Select This bit is set to 1 to indicate that data is present and that the data is transferred using the DAT line. This bit is set to 0 in the following instances: - Command using the CMD line - Command with no data transfer but using busy signal on the DAT\\[0\\]
line - Resume Command Values: - 0x0 (NO_DATA): No Data Present - 0x1 (DATA): Data Present"]
    #[inline(always)]
    #[must_use]
    pub fn data_present_sel(&mut self) -> DataPresentSelW<CmdRSpec> {
        DataPresentSelW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Command Type These bits indicate the command type. Note: While issuing Abort CMD using CMD12/CMD52 or reset CMD using CMD0/CMD52, CMD_TYPE field shall be set to 0x3. Values: - 0x3 (ABORT_CMD): Abort - 0x2 (RESUME_CMD): Resume - 0x1 (SUSPEND_CMD): Suspend - 0x0 (NORMAL_CMD): Normal"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CmdTypeW<CmdRSpec> {
        CmdTypeW::new(self, 6)
    }
    #[doc = "Bits 8:13 - Command Index These bits are set to the command number that is specified in bits 45-40 of the Command Format."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<CmdRSpec> {
        CmdIndexW::new(self, 8)
    }
}
#[doc = "Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdRSpec;
impl crate::RegisterSpec for CmdRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmd_r::R`](R) reader structure"]
impl crate::Readable for CmdRSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd_r::W`](W) writer structure"]
impl crate::Writable for CmdRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CMD_R to value 0"]
impl crate::Resettable for CmdRSpec {
    const RESET_VALUE: u16 = 0;
}
