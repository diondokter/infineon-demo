#[doc = "Register `CHAN_RESULT_UPDATED` reader"]
pub type R = crate::R<ChanResultUpdatedSpec>;
#[doc = "Field `CHAN_RESULT_UPDATED` reader - If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
pub type ChanResultUpdatedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT register was updated, i.e. was sampled during the previous scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_result_updated(&self) -> ChanResultUpdatedR {
        ChanResultUpdatedR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel result data register 'updated' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result_updated::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanResultUpdatedSpec;
impl crate::RegisterSpec for ChanResultUpdatedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_result_updated::R`](R) reader structure"]
impl crate::Readable for ChanResultUpdatedSpec {}
#[doc = "`reset()` method sets CHAN_RESULT_UPDATED to value 0"]
impl crate::Resettable for ChanResultUpdatedSpec {
    const RESET_VALUE: u32 = 0;
}
