#[doc = "Register `NORMAL_INT_STAT_R` reader"]
pub type R = crate::R<NormalIntStatRSpec>;
#[doc = "Register `NORMAL_INT_STAT_R` writer"]
pub type W = crate::W<NormalIntStatRSpec>;
#[doc = "Field `CMD_COMPLETE` reader - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
pub type CmdCompleteR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE` writer - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
pub type CmdCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE` reader - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
pub type XferCompleteR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE` writer - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
pub type XferCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGAP_EVENT` reader - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
pub type BgapEventR = crate::BitReader;
#[doc = "Field `BGAP_EVENT` writer - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
pub type BgapEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT` reader - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
pub type DmaInterruptR = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT` writer - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
pub type DmaInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY` reader - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
pub type BufWrReadyR = crate::BitReader;
#[doc = "Field `BUF_WR_READY` writer - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
pub type BufWrReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY` reader - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
pub type BufRdReadyR = crate::BitReader;
#[doc = "Field `BUF_RD_READY` writer - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
pub type BufRdReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION` reader - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
pub type CardInsertionR = crate::BitReader;
#[doc = "Field `CARD_INSERTION` writer - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
pub type CardInsertionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL` reader - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
pub type CardRemovalR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL` writer - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
pub type CardRemovalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTERRUPT` reader - Card Interrupt This bit reflects the synchronized value of: - DAT\\[1\\]
Interrupt Input for SD Mode Values: - 0x0 (FALSE): No Card Interrupt - 0x1 (TRUE): Generate Card Interrupt"]
pub type CardInterruptR = crate::BitReader;
#[doc = "Field `FX_EVENT` reader - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): FX Event is detected"]
pub type FxEventR = crate::BitReader;
#[doc = "Field `CQE_EVENT` reader - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
pub type CqeEventR = crate::BitReader;
#[doc = "Field `CQE_EVENT` writer - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
pub type CqeEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTERRUPT` reader - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: - 0x0 (FALSE): No Error - 0x1 (TRUE): Error"]
pub type ErrInterruptR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CmdCompleteR {
        CmdCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XferCompleteR {
        XferCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    pub fn bgap_event(&self) -> BgapEventR {
        BgapEventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(&self) -> DmaInterruptR {
        DmaInterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(&self) -> BufWrReadyR {
        BufWrReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(&self) -> BufRdReadyR {
        BufRdReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    pub fn card_insertion(&self) -> CardInsertionR {
        CardInsertionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    pub fn card_removal(&self) -> CardRemovalR {
        CardRemovalR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt This bit reflects the synchronized value of: - DAT\\[1\\]
Interrupt Input for SD Mode Values: - 0x0 (FALSE): No Card Interrupt - 0x1 (TRUE): Generate Card Interrupt"]
    #[inline(always)]
    pub fn card_interrupt(&self) -> CardInterruptR {
        CardInterruptR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event This status is set when R\\[14\\]
of response register is set to 1 and Response Type R1/R5 is set to 0 in Transfer Mode register. This interrupt is used with response check function. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): FX Event is detected"]
    #[inline(always)]
    pub fn fx_event(&self) -> FxEventR {
        FxEventR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    pub fn cqe_event(&self) -> CqeEventR {
        CqeEventR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Error Interrupt If any of the bits in the Error Interrupt Status register are set, then this bit is set. Values: - 0x0 (FALSE): No Error - 0x1 (TRUE): Error"]
    #[inline(always)]
    pub fn err_interrupt(&self) -> ErrInterruptR {
        ErrInterruptR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete In an SD/eMMC Mode, this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. Values: - 0x0 (FALSE): No command complete - 0x1 (TRUE): Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CmdCompleteW<NormalIntStatRSpec> {
        CmdCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete This bit is set when a read/write transfer and a command with status busy is completed. Values: - 0x0 (FALSE): Not complete - 0x1 (TRUE): Command execution is completed"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete(&mut self) -> XferCompleteW<NormalIntStatRSpec> {
        XferCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event This bit is set when both read/write transaction is stopped at block gap due to a Stop at Block Gap Request. Values: - 0x0 (FALSE): No Block Gap Event - 0x1 (TRUE): Transaction stopped at block gap"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event(&mut self) -> BgapEventW<NormalIntStatRSpec> {
        BgapEventW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA, by setting the Int field in the descriptor table, the Host controller generates this interrupt. This interrupt is not generated after a Transfer Complete. Values: - 0x0 (FALSE): No DMA Interrupt - 0x1 (TRUE): DMA Interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt(&mut self) -> DmaInterruptW<NormalIntStatRSpec> {
        DmaInterruptW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready This bit is set if the Buffer Write Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to write buffer - 0x1 (TRUE): Ready to write buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready(&mut self) -> BufWrReadyW<NormalIntStatRSpec> {
        BufWrReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready This bit is set if the Buffer Read Enable changes from 0 to 1. Values: - 0x0 (FALSE): Not ready to read buffer - 0x1 (TRUE): Ready to read buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready(&mut self) -> BufRdReadyW<NormalIntStatRSpec> {
        BufRdReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion This bit is set if the Card Inserted in the Present State register changes from 0 to 1. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion(&mut self) -> CardInsertionW<NormalIntStatRSpec> {
        CardInsertionW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal This bit is set if the Card Inserted in the Present State register changes from 1 to 0. Values: - 0x0 (FALSE): Card state stable or Debouncing - 0x1 (TRUE): Card Removed"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal(&mut self) -> CardRemovalW<NormalIntStatRSpec> {
        CardRemovalW::new(self, 7)
    }
    #[doc = "Bit 14 - Command Queuing Event This status is set if Command Queuing/Crypto related event has occurred in eMMC/SD mode. Read CQHCI's CQIS/CRNQIS register for more details. In UHS-II Mode, this bit is irrelevant. Values: - 0x0 (FALSE): No Event - 0x1 (TRUE): Command Queuing Event is detected"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event(&mut self) -> CqeEventW<NormalIntStatRSpec> {
        CqeEventW::new(self, 14)
    }
}
#[doc = "Normal Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_stat_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_stat_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NormalIntStatRSpec;
impl crate::RegisterSpec for NormalIntStatRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`normal_int_stat_r::R`](R) reader structure"]
impl crate::Readable for NormalIntStatRSpec {}
#[doc = "`write(|w| ..)` method takes [`normal_int_stat_r::W`](W) writer structure"]
impl crate::Writable for NormalIntStatRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_STAT_R to value 0"]
impl crate::Resettable for NormalIntStatRSpec {
    const RESET_VALUE: u16 = 0;
}
