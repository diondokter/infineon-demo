#[doc = "Register `TR_CTL0` reader"]
pub type R = crate::R<TrCtl0Spec>;
#[doc = "Register `TR_CTL0` writer"]
pub type W = crate::W<TrCtl0Spec>;
#[doc = "Field `SAMPLE_CLOCK_DIV` reader - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
pub type SampleClockDivR = crate::FieldReader;
#[doc = "Field `SAMPLE_CLOCK_DIV` writer - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
pub type SampleClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RED_CLOCK_DIV` reader - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
pub type RedClockDivR = crate::FieldReader;
#[doc = "Field `RED_CLOCK_DIV` writer - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
pub type RedClockDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INIT_DELAY` reader - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
pub type InitDelayR = crate::FieldReader;
#[doc = "Field `INIT_DELAY` writer - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
pub type InitDelayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VON_NEUMANN_CORR` reader - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
pub type VonNeumannCorrR = crate::BitReader;
#[doc = "Field `VON_NEUMANN_CORR` writer - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
pub type VonNeumannCorrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ON_AP_DETECT` reader - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type StopOnApDetectR = crate::BitReader;
#[doc = "Field `STOP_ON_AP_DETECT` writer - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type StopOnApDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_ON_RC_DETECT` reader - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type StopOnRcDetectR = crate::BitReader;
#[doc = "Field `STOP_ON_RC_DETECT` writer - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type StopOnRcDetectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
    #[inline(always)]
    pub fn sample_clock_div(&self) -> SampleClockDivR {
        SampleClockDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
    #[inline(always)]
    pub fn red_clock_div(&self) -> RedClockDivR {
        RedClockDivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
    #[inline(always)]
    pub fn init_delay(&self) -> InitDelayR {
        InitDelayR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
    #[inline(always)]
    pub fn von_neumann_corr(&self) -> VonNeumannCorrR {
        VonNeumannCorrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    pub fn stop_on_ap_detect(&self) -> StopOnApDetectR {
        StopOnApDetectR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    pub fn stop_on_rc_detect(&self) -> StopOnRcDetectR {
        StopOnRcDetectR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_div(&mut self) -> SampleClockDivW<TrCtl0Spec> {
        SampleClockDivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
    #[inline(always)]
    #[must_use]
    pub fn red_clock_div(&mut self) -> RedClockDivW<TrCtl0Spec> {
        RedClockDivW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
    #[inline(always)]
    #[must_use]
    pub fn init_delay(&mut self) -> InitDelayW<TrCtl0Spec> {
        InitDelayW::new(self, 16)
    }
    #[doc = "Bit 24 - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
    #[inline(always)]
    #[must_use]
    pub fn von_neumann_corr(&mut self) -> VonNeumannCorrW<TrCtl0Spec> {
        VonNeumannCorrW::new(self, 24)
    }
    #[doc = "Bit 28 - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_ap_detect(&mut self) -> StopOnApDetectW<TrCtl0Spec> {
        StopOnApDetectW::new(self, 28)
    }
    #[doc = "Bit 29 - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_rc_detect(&mut self) -> StopOnRcDetectW<TrCtl0Spec> {
        StopOnRcDetectW::new(self, 29)
    }
}
#[doc = "True random control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtl0Spec;
impl crate::RegisterSpec for TrCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl0::R`](R) reader structure"]
impl crate::Readable for TrCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl0::W`](W) writer structure"]
impl crate::Writable for TrCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTL0 to value 0x0003_0000"]
impl crate::Resettable for TrCtl0Spec {
    const RESET_VALUE: u32 = 0x0003_0000;
}
