#[doc = "Register `NORMAL_INT_STAT_EN_R` reader"]
pub type R = crate::R<NormalIntStatEnRSpec>;
#[doc = "Register `NORMAL_INT_STAT_EN_R` writer"]
pub type W = crate::W<NormalIntStatEnRSpec>;
#[doc = "Field `CMD_COMPLETE_STAT_EN` reader - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCompleteStatEnR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_STAT_EN` writer - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCompleteStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE_STAT_EN` reader - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XferCompleteStatEnR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_STAT_EN` writer - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XferCompleteStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGAP_EVENT_STAT_EN` reader - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BgapEventStatEnR = crate::BitReader;
#[doc = "Field `BGAP_EVENT_STAT_EN` writer - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BgapEventStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` reader - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DmaInterruptStatEnR = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT_STAT_EN` writer - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DmaInterruptStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY_STAT_EN` reader - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufWrReadyStatEnR = crate::BitReader;
#[doc = "Field `BUF_WR_READY_STAT_EN` writer - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufWrReadyStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY_STAT_EN` reader - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufRdReadyStatEnR = crate::BitReader;
#[doc = "Field `BUF_RD_READY_STAT_EN` writer - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufRdReadyStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION_STAT_EN` reader - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInsertionStatEnR = crate::BitReader;
#[doc = "Field `CARD_INSERTION_STAT_EN` writer - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInsertionStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL_STAT_EN` reader - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardRemovalStatEnR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL_STAT_EN` writer - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardRemovalStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` reader - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInterruptStatEnR = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT_STAT_EN` writer - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInterruptStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_A_STAT_EN` reader - N/A"]
pub type IntAStatEnR = crate::BitReader;
#[doc = "Field `INT_A_STAT_EN` writer - N/A"]
pub type IntAStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_B_STAT_EN` reader - N/A"]
pub type IntBStatEnR = crate::BitReader;
#[doc = "Field `INT_B_STAT_EN` writer - N/A"]
pub type IntBStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_C_STAT_EN` reader - N/A"]
pub type IntCStatEnR = crate::BitReader;
#[doc = "Field `INT_C_STAT_EN` writer - N/A"]
pub type IntCStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` reader - N/A"]
pub type ReTuneEventStatEnR = crate::BitReader;
#[doc = "Field `RE_TUNE_EVENT_STAT_EN` writer - N/A"]
pub type ReTuneEventStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FX_EVENT_STAT_EN` reader - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FxEventStatEnR = crate::BitReader;
#[doc = "Field `FX_EVENT_STAT_EN` writer - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FxEventStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQE_EVENT_STAT_EN` reader - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CqeEventStatEnR = crate::BitReader;
#[doc = "Field `CQE_EVENT_STAT_EN` writer - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CqeEventStatEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(&self) -> CmdCompleteStatEnR {
        CmdCompleteStatEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(&self) -> XferCompleteStatEnR {
        XferCompleteStatEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_stat_en(&self) -> BgapEventStatEnR {
        BgapEventStatEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(&self) -> DmaInterruptStatEnR {
        DmaInterruptStatEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(&self) -> BufWrReadyStatEnR {
        BufWrReadyStatEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(&self) -> BufRdReadyStatEnR {
        BufRdReadyStatEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_stat_en(&self) -> CardInsertionStatEnR {
        CardInsertionStatEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_stat_en(&self) -> CardRemovalStatEnR {
        CardRemovalStatEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_stat_en(&self) -> CardInterruptStatEnR {
        CardInterruptStatEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_stat_en(&self) -> IntAStatEnR {
        IntAStatEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_stat_en(&self) -> IntBStatEnR {
        IntBStatEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_stat_en(&self) -> IntCStatEnR {
        IntCStatEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_stat_en(&self) -> ReTuneEventStatEnR {
        ReTuneEventStatEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_stat_en(&self) -> FxEventStatEnR {
        FxEventStatEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_stat_en(&self) -> CqeEventStatEnR {
        CqeEventStatEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_stat_en(&mut self) -> CmdCompleteStatEnW<NormalIntStatEnRSpec> {
        CmdCompleteStatEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_stat_en(&mut self) -> XferCompleteStatEnW<NormalIntStatEnRSpec> {
        XferCompleteStatEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event_stat_en(&mut self) -> BgapEventStatEnW<NormalIntStatEnRSpec> {
        BgapEventStatEnW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt_stat_en(&mut self) -> DmaInterruptStatEnW<NormalIntStatEnRSpec> {
        DmaInterruptStatEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready_stat_en(&mut self) -> BufWrReadyStatEnW<NormalIntStatEnRSpec> {
        BufWrReadyStatEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready_stat_en(&mut self) -> BufRdReadyStatEnW<NormalIntStatEnRSpec> {
        BufRdReadyStatEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion_stat_en(&mut self) -> CardInsertionStatEnW<NormalIntStatEnRSpec> {
        CardInsertionStatEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_stat_en(&mut self) -> CardRemovalStatEnW<NormalIntStatEnRSpec> {
        CardRemovalStatEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable If this bit is set to 0, the Host Controller clears the interrupt request to the System. The Card Interrupt detection is stopped when this bit is cleared and restarted when this bit is set to 1. The Host Driver may clear the Card Interrupt Status Enable before servicing the Card Interrupt and may set this bit again after all interrupt requests from the card are cleared to prevent inadvertent interrupts. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt_stat_en(&mut self) -> CardInterruptStatEnW<NormalIntStatEnRSpec> {
        CardInterruptStatEnW::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_a_stat_en(&mut self) -> IntAStatEnW<NormalIntStatEnRSpec> {
        IntAStatEnW::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_b_stat_en(&mut self) -> IntBStatEnW<NormalIntStatEnRSpec> {
        IntBStatEnW::new(self, 10)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_c_stat_en(&mut self) -> IntCStatEnW<NormalIntStatEnRSpec> {
        IntCStatEnW::new(self, 11)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn re_tune_event_stat_en(&mut self) -> ReTuneEventStatEnW<NormalIntStatEnRSpec> {
        ReTuneEventStatEnW::new(self, 12)
    }
    #[doc = "Bit 13 - FX Event Status Enable This bit is added from Version 4.10. Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fx_event_stat_en(&mut self) -> FxEventStatEnW<NormalIntStatEnRSpec> {
        FxEventStatEnW::new(self, 13)
    }
    #[doc = "Bit 14 - CQE Event Status Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event_stat_en(&mut self) -> CqeEventStatEnW<NormalIntStatEnRSpec> {
        CqeEventStatEnW::new(self, 14)
    }
}
#[doc = "Normal Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_stat_en_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_stat_en_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NormalIntStatEnRSpec;
impl crate::RegisterSpec for NormalIntStatEnRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`normal_int_stat_en_r::R`](R) reader structure"]
impl crate::Readable for NormalIntStatEnRSpec {}
#[doc = "`write(|w| ..)` method takes [`normal_int_stat_en_r::W`](W) writer structure"]
impl crate::Writable for NormalIntStatEnRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_STAT_EN_R to value 0"]
impl crate::Resettable for NormalIntStatEnRSpec {
    const RESET_VALUE: u16 = 0;
}
