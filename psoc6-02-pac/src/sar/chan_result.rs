#[doc = "Register `CHAN_RESULT[%s]` reader"]
pub type R = crate::R<ChanResultSpec>;
#[doc = "Field `RESULT` reader - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
pub type ResultR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_RESULT_NEWVALUE_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
pub type ChanResultNewvalueMirR = crate::BitReader;
#[doc = "Field `SATURATE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
pub type SaturateIntrMirR = crate::BitReader;
#[doc = "Field `RANGE_INTR_MIR` reader - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
pub type RangeIntrMirR = crate::BitReader;
#[doc = "Field `CHAN_RESULT_UPDATED_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
pub type ChanResultUpdatedMirR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - SAR conversion result of the channel. The data is copied here from the WORK field after all enabled channels in this scan have been sampled."]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_RESULT_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_result_newvalue_mir(&self) -> ChanResultNewvalueMirR {
        ChanResultNewvalueMirR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - mirror bit of corresponding bit in SAR_SATURATE_INTR register"]
    #[inline(always)]
    pub fn saturate_intr_mir(&self) -> SaturateIntrMirR {
        SaturateIntrMirR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - mirror bit of corresponding bit in SAR_RANGE_INTR register"]
    #[inline(always)]
    pub fn range_intr_mir(&self) -> RangeIntrMirR {
        RangeIntrMirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_RESULT_UPDATED register"]
    #[inline(always)]
    pub fn chan_result_updated_mir(&self) -> ChanResultUpdatedMirR {
        ChanResultUpdatedMirR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel result data register\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanResultSpec;
impl crate::RegisterSpec for ChanResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_result::R`](R) reader structure"]
impl crate::Readable for ChanResultSpec {}
#[doc = "`reset()` method sets CHAN_RESULT[%s]
to value 0"]
impl crate::Resettable for ChanResultSpec {
    const RESET_VALUE: u32 = 0;
}
