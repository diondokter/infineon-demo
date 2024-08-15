#[doc = "Register `CHAN_WORK[%s]` reader"]
pub type R = crate::R<ChanWorkSpec>;
#[doc = "Field `WORK` reader - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
pub type WorkR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_WORK_NEWVALUE_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
pub type ChanWorkNewvalueMirR = crate::BitReader;
#[doc = "Field `CHAN_WORK_UPDATED_MIR` reader - mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
pub type ChanWorkUpdatedMirR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - SAR conversion working data of the channel. The data is written here right after sampling this channel."]
    #[inline(always)]
    pub fn work(&self) -> WorkR {
        WorkR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 27 - mirror bit of corresponding bit in SAR_CHAN_WORK_NEWVALUE register"]
    #[inline(always)]
    pub fn chan_work_newvalue_mir(&self) -> ChanWorkNewvalueMirR {
        ChanWorkNewvalueMirR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - mirror bit of corresponding bit in SAR_CHAN_WORK_UPDATED register"]
    #[inline(always)]
    pub fn chan_work_updated_mir(&self) -> ChanWorkUpdatedMirR {
        ChanWorkUpdatedMirR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel working data register\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_work::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanWorkSpec;
impl crate::RegisterSpec for ChanWorkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_work::R`](R) reader structure"]
impl crate::Readable for ChanWorkSpec {}
#[doc = "`reset()` method sets CHAN_WORK[%s]
to value 0"]
impl crate::Resettable for ChanWorkSpec {
    const RESET_VALUE: u32 = 0;
}
