#[doc = "Register `SEQ_START` reader"]
pub type R = crate::R<SeqStartSpec>;
#[doc = "Register `SEQ_START` writer"]
pub type W = crate::W<SeqStartSpec>;
#[doc = "Field `START` reader - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_MODE` reader - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub type SeqModeR = crate::BitReader;
#[doc = "Field `SEQ_MODE` writer - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
pub type SeqModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_START_EN` reader - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub type DsiStartEnR = crate::BitReader;
#[doc = "Field `DSI_START_EN` writer - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
pub type DsiStartEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ0_SKIP` reader - When set the AutoZero_0 state will be skipped"]
pub type Az0SkipR = crate::BitReader;
#[doc = "Field `AZ0_SKIP` writer - When set the AutoZero_0 state will be skipped"]
pub type Az0SkipW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AZ1_SKIP` reader - When set the AutoZero_1 state will be skipped"]
pub type Az1SkipR = crate::BitReader;
#[doc = "Field `AZ1_SKIP` writer - When set the AutoZero_1 state will be skipped"]
pub type Az1SkipW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    pub fn seq_mode(&self) -> SeqModeR {
        SeqModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    pub fn dsi_start_en(&self) -> DsiStartEnR {
        DsiStartEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    pub fn az0_skip(&self) -> Az0SkipR {
        Az0SkipR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    pub fn az1_skip(&self) -> Az1SkipR {
        Az1SkipR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start the CSD sequencer. The sequencer will clear this bit when it is done. Depending on the mode the sequencer is done when a sample has been accumulated, when the high speed comparator trips or if the sequencer is aborted. When the ADC is enabled the ADC sequencer will start when the CSD sequencer reaches the Sample_norm state (only with the regular CSD scan mode)."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<SeqStartSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = regular CSD scan + optional ADC 1 = coarse initialization, the Sequencer will go to the INIT_COARSE state."]
    #[inline(always)]
    #[must_use]
    pub fn seq_mode(&mut self) -> SeqModeW<SeqStartSpec> {
        SeqModeW::new(self, 1)
    }
    #[doc = "Bit 3 - When a 1 is written the CSD and ADC sequencers will be aborted (if they are running) and the START bit will be cleared. This bit always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<SeqStartSpec> {
        AbortW::new(self, 3)
    }
    #[doc = "Bit 4 - When this bit is set a positive edge on dsi_start will start the CSD sequencer and if enabled also the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_start_en(&mut self) -> DsiStartEnW<SeqStartSpec> {
        DsiStartEnW::new(self, 4)
    }
    #[doc = "Bit 8 - When set the AutoZero_0 state will be skipped"]
    #[inline(always)]
    #[must_use]
    pub fn az0_skip(&mut self) -> Az0SkipW<SeqStartSpec> {
        Az0SkipW::new(self, 8)
    }
    #[doc = "Bit 9 - When set the AutoZero_1 state will be skipped"]
    #[inline(always)]
    #[must_use]
    pub fn az1_skip(&mut self) -> Az1SkipW<SeqStartSpec> {
        Az1SkipW::new(self, 9)
    }
}
#[doc = "Sequencer start\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqStartSpec;
impl crate::RegisterSpec for SeqStartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_start::R`](R) reader structure"]
impl crate::Readable for SeqStartSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_start::W`](W) writer structure"]
impl crate::Writable for SeqStartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_START to value 0"]
impl crate::Resettable for SeqStartSpec {
    const RESET_VALUE: u32 = 0;
}
