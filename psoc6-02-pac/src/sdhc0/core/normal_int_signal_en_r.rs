#[doc = "Register `NORMAL_INT_SIGNAL_EN_R` reader"]
pub type R = crate::R<NormalIntSignalEnRSpec>;
#[doc = "Register `NORMAL_INT_SIGNAL_EN_R` writer"]
pub type W = crate::W<NormalIntSignalEnRSpec>;
#[doc = "Field `CMD_COMPLETE_SIGNAL_EN` reader - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCompleteSignalEnR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE_SIGNAL_EN` writer - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CmdCompleteSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE_SIGNAL_EN` reader - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XferCompleteSignalEnR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE_SIGNAL_EN` writer - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type XferCompleteSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BGAP_EVENT_SIGNAL_EN` reader - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BgapEventSignalEnR = crate::BitReader;
#[doc = "Field `BGAP_EVENT_SIGNAL_EN` writer - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BgapEventSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT_SIGNAL_EN` reader - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DmaInterruptSignalEnR = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT_SIGNAL_EN` writer - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type DmaInterruptSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY_SIGNAL_EN` reader - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufWrReadySignalEnR = crate::BitReader;
#[doc = "Field `BUF_WR_READY_SIGNAL_EN` writer - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufWrReadySignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY_SIGNAL_EN` reader - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufRdReadySignalEnR = crate::BitReader;
#[doc = "Field `BUF_RD_READY_SIGNAL_EN` writer - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type BufRdReadySignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTION_SIGNAL_EN` reader - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInsertionSignalEnR = crate::BitReader;
#[doc = "Field `CARD_INSERTION_SIGNAL_EN` writer - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInsertionSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REMOVAL_SIGNAL_EN` reader - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardRemovalSignalEnR = crate::BitReader;
#[doc = "Field `CARD_REMOVAL_SIGNAL_EN` writer - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardRemovalSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTERRUPT_SIGNAL_EN` reader - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInterruptSignalEnR = crate::BitReader;
#[doc = "Field `CARD_INTERRUPT_SIGNAL_EN` writer - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CardInterruptSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_A_SIGNAL_EN` reader - N/A"]
pub type IntASignalEnR = crate::BitReader;
#[doc = "Field `INT_A_SIGNAL_EN` writer - N/A"]
pub type IntASignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_B_SIGNAL_EN` reader - N/A"]
pub type IntBSignalEnR = crate::BitReader;
#[doc = "Field `INT_B_SIGNAL_EN` writer - N/A"]
pub type IntBSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_C_SIGNAL_EN` reader - N/A"]
pub type IntCSignalEnR = crate::BitReader;
#[doc = "Field `INT_C_SIGNAL_EN` writer - N/A"]
pub type IntCSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE_TUNE_EVENT_SIGNAL_EN` reader - N/A"]
pub type ReTuneEventSignalEnR = crate::BitReader;
#[doc = "Field `RE_TUNE_EVENT_SIGNAL_EN` writer - N/A"]
pub type ReTuneEventSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FX_EVENT_SIGNAL_EN` reader - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FxEventSignalEnR = crate::BitReader;
#[doc = "Field `FX_EVENT_SIGNAL_EN` writer - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type FxEventSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CQE_EVENT_SIGNAL_EN` reader - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CqeEventSignalEnR = crate::BitReader;
#[doc = "Field `CQE_EVENT_SIGNAL_EN` writer - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
pub type CqeEventSignalEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cmd_complete_signal_en(&self) -> CmdCompleteSignalEnR {
        CmdCompleteSignalEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn xfer_complete_signal_en(&self) -> XferCompleteSignalEnR {
        XferCompleteSignalEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn bgap_event_signal_en(&self) -> BgapEventSignalEnR {
        BgapEventSignalEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn dma_interrupt_signal_en(&self) -> DmaInterruptSignalEnR {
        DmaInterruptSignalEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_wr_ready_signal_en(&self) -> BufWrReadySignalEnR {
        BufWrReadySignalEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn buf_rd_ready_signal_en(&self) -> BufRdReadySignalEnR {
        BufRdReadySignalEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_insertion_signal_en(&self) -> CardInsertionSignalEnR {
        CardInsertionSignalEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_removal_signal_en(&self) -> CardRemovalSignalEnR {
        CardRemovalSignalEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn card_interrupt_signal_en(&self) -> CardInterruptSignalEnR {
        CardInterruptSignalEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    pub fn int_a_signal_en(&self) -> IntASignalEnR {
        IntASignalEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn int_b_signal_en(&self) -> IntBSignalEnR {
        IntBSignalEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn int_c_signal_en(&self) -> IntCSignalEnR {
        IntCSignalEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn re_tune_event_signal_en(&self) -> ReTuneEventSignalEnR {
        ReTuneEventSignalEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn fx_event_signal_en(&self) -> FxEventSignalEnR {
        FxEventSignalEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    pub fn cqe_event_signal_en(&self) -> CqeEventSignalEnR {
        CqeEventSignalEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete_signal_en(&mut self) -> CmdCompleteSignalEnW<NormalIntSignalEnRSpec> {
        CmdCompleteSignalEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Complete Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete_signal_en(&mut self) -> XferCompleteSignalEnW<NormalIntSignalEnRSpec> {
        XferCompleteSignalEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Block Gap Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bgap_event_signal_en(&mut self) -> BgapEventSignalEnW<NormalIntSignalEnRSpec> {
        BgapEventSignalEnW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt_signal_en(&mut self) -> DmaInterruptSignalEnW<NormalIntSignalEnRSpec> {
        DmaInterruptSignalEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Buffer Write Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready_signal_en(&mut self) -> BufWrReadySignalEnW<NormalIntSignalEnRSpec> {
        BufWrReadySignalEnW::new(self, 4)
    }
    #[doc = "Bit 5 - Buffer Read Ready Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready_signal_en(&mut self) -> BufRdReadySignalEnW<NormalIntSignalEnRSpec> {
        BufRdReadySignalEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Card Insertion Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_insertion_signal_en(&mut self) -> CardInsertionSignalEnW<NormalIntSignalEnRSpec> {
        CardInsertionSignalEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Card Removal Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_removal_signal_en(&mut self) -> CardRemovalSignalEnW<NormalIntSignalEnRSpec> {
        CardRemovalSignalEnW::new(self, 7)
    }
    #[doc = "Bit 8 - Card Interrupt Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn card_interrupt_signal_en(&mut self) -> CardInterruptSignalEnW<NormalIntSignalEnRSpec> {
        CardInterruptSignalEnW::new(self, 8)
    }
    #[doc = "Bit 9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_a_signal_en(&mut self) -> IntASignalEnW<NormalIntSignalEnRSpec> {
        IntASignalEnW::new(self, 9)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_b_signal_en(&mut self) -> IntBSignalEnW<NormalIntSignalEnRSpec> {
        IntBSignalEnW::new(self, 10)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_c_signal_en(&mut self) -> IntCSignalEnW<NormalIntSignalEnRSpec> {
        IntCSignalEnW::new(self, 11)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn re_tune_event_signal_en(&mut self) -> ReTuneEventSignalEnW<NormalIntSignalEnRSpec> {
        ReTuneEventSignalEnW::new(self, 12)
    }
    #[doc = "Bit 13 - FX Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fx_event_signal_en(&mut self) -> FxEventSignalEnW<NormalIntSignalEnRSpec> {
        FxEventSignalEnW::new(self, 13)
    }
    #[doc = "Bit 14 - Command Queuing Engine Event Signal Enable Values: - 0x0 (FALSE): Masked - 0x1 (TRUE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cqe_event_signal_en(&mut self) -> CqeEventSignalEnW<NormalIntSignalEnRSpec> {
        CqeEventSignalEnW::new(self, 14)
    }
}
#[doc = "Normal Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`normal_int_signal_en_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`normal_int_signal_en_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NormalIntSignalEnRSpec;
impl crate::RegisterSpec for NormalIntSignalEnRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`normal_int_signal_en_r::R`](R) reader structure"]
impl crate::Readable for NormalIntSignalEnRSpec {}
#[doc = "`write(|w| ..)` method takes [`normal_int_signal_en_r::W`](W) writer structure"]
impl crate::Writable for NormalIntSignalEnRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets NORMAL_INT_SIGNAL_EN_R to value 0"]
impl crate::Resettable for NormalIntSignalEnRSpec {
    const RESET_VALUE: u16 = 0;
}
