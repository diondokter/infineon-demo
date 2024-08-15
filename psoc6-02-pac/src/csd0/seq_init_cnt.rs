#[doc = "Register `SEQ_INIT_CNT` reader"]
pub type R = crate::R<SeqInitCntSpec>;
#[doc = "Register `SEQ_INIT_CNT` writer"]
pub type W = crate::W<SeqInitCntSpec>;
#[doc = "Field `CONV_CNT` reader - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
pub type ConvCntR = crate::FieldReader<u16>;
#[doc = "Field `CONV_CNT` writer - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
pub type ConvCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    pub fn conv_cnt(&self) -> ConvCntR {
        ConvCntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of conversion per Initialization sample, if set to 0 the Sample_init state will be skipped."]
    #[inline(always)]
    #[must_use]
    pub fn conv_cnt(&mut self) -> ConvCntW<SeqInitCntSpec> {
        ConvCntW::new(self, 0)
    }
}
#[doc = "Sequencer Initial conversion and sample counts\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_init_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_init_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqInitCntSpec;
impl crate::RegisterSpec for SeqInitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_init_cnt::R`](R) reader structure"]
impl crate::Readable for SeqInitCntSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_init_cnt::W`](W) writer structure"]
impl crate::Writable for SeqInitCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_INIT_CNT to value 0"]
impl crate::Resettable for SeqInitCntSpec {
    const RESET_VALUE: u32 = 0;
}
