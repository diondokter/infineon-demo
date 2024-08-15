#[doc = "Register `CHAN_WORK_UPDATED` reader"]
pub type R = crate::R<ChanWorkUpdatedSpec>;
#[doc = "Field `CHAN_WORK_UPDATED` reader - If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
pub type ChanWorkUpdatedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding WORK register was updated, i.e. was already sampled during the current scan and, in case of Interleaved averaging, reached the averaging count. If this bit is low then either the channel is not enabled or the averaging count is not yet reached for Interleaved averaging."]
    #[inline(always)]
    pub fn chan_work_updated(&self) -> ChanWorkUpdatedR {
        ChanWorkUpdatedR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel working data register 'updated' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_work_updated::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanWorkUpdatedSpec;
impl crate::RegisterSpec for ChanWorkUpdatedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_work_updated::R`](R) reader structure"]
impl crate::Readable for ChanWorkUpdatedSpec {}
#[doc = "`reset()` method sets CHAN_WORK_UPDATED to value 0"]
impl crate::Resettable for ChanWorkUpdatedSpec {
    const RESET_VALUE: u32 = 0;
}
