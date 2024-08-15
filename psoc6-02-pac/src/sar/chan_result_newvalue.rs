#[doc = "Register `CHAN_RESULT_NEWVALUE` reader"]
pub type R = crate::R<ChanResultNewvalueSpec>;
#[doc = "Field `CHAN_RESULT_NEWVALUE` reader - If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
pub type ChanResultNewvalueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - If set the corresponding RESULT data received a new value, i.e. was sampled during the last scan and data was valid. In case of a UAB this New Value bit reflects the value of UAB.valid output, for anything else the data is always valid. In case of averaging this New Value bit is an OR of all the valid bits received by each conversion."]
    #[inline(always)]
    pub fn chan_result_newvalue(&self) -> ChanResultNewvalueR {
        ChanResultNewvalueR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channel result data register 'new value' bits\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result_newvalue::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanResultNewvalueSpec;
impl crate::RegisterSpec for ChanResultNewvalueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_result_newvalue::R`](R) reader structure"]
impl crate::Readable for ChanResultNewvalueSpec {}
#[doc = "`reset()` method sets CHAN_RESULT_NEWVALUE to value 0"]
impl crate::Resettable for ChanResultNewvalueSpec {
    const RESET_VALUE: u32 = 0;
}
