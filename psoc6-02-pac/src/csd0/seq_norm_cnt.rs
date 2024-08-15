#[doc = "Register `SEQ_NORM_CNT` reader"]
pub type R = crate::R<SeqNormCntSpec>;
#[doc = "Register `SEQ_NORM_CNT` writer"]
pub type W = crate::W<SeqNormCntSpec>;
#[doc = "Field `CONV_CNT` reader - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type ConvCntR = crate::FieldReader<u16>;
#[doc = "Field `CONV_CNT` writer - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
pub type ConvCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    pub fn conv_cnt(&self) -> ConvCntR {
        ConvCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per sample, if set to 0 the Sample_norm state will be skipped. Sample window size = SEQ_NORM_CNT.CONV_CNT * (SENSE_PERIOD.SENSE_DIV+1). Note for CSDv1 Sample window size = PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn conv_cnt(&mut self) -> ConvCntW<SeqNormCntSpec> {
        ConvCntW::new(self, 0)
    }
}
#[doc = "Sequencer Normal conversion and sample counts\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_norm_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_norm_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqNormCntSpec;
impl crate::RegisterSpec for SeqNormCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_norm_cnt::R`](R) reader structure"]
impl crate::Readable for SeqNormCntSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_norm_cnt::W`](W) writer structure"]
impl crate::Writable for SeqNormCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_NORM_CNT to value 0"]
impl crate::Resettable for SeqNormCntSpec {
    const RESET_VALUE: u32 = 0;
}
