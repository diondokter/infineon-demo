#[doc = "Register `STAT_SEQ` reader"]
pub type R = crate::R<StatSeqSpec>;
#[doc = "Field `SEQ_STATE` reader - CSD sequencer state"]
pub type SeqStateR = crate::FieldReader;
#[doc = "Field `ADC_STATE` reader - ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
pub type AdcStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - CSD sequencer state"]
    #[inline(always)]
    pub fn seq_state(&self) -> SeqStateR {
        SeqStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - ADC sequencer state (only relevant after SEQ_STATE has reached SAMPLE_NORM and ADC sequencer has started)"]
    #[inline(always)]
    pub fn adc_state(&self) -> AdcStateR {
        AdcStateR::new(((self.bits >> 16) & 7) as u8)
    }
}
#[doc = "Current Sequencer status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_seq::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSeqSpec;
impl crate::RegisterSpec for StatSeqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_seq::R`](R) reader structure"]
impl crate::Readable for StatSeqSpec {}
#[doc = "`reset()` method sets STAT_SEQ to value 0"]
impl crate::Resettable for StatSeqSpec {
    const RESET_VALUE: u32 = 0;
}
