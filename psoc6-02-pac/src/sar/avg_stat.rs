#[doc = "Register `AVG_STAT` reader"]
pub type R = crate::R<AvgStatSpec>;
#[doc = "Field `CUR_AVG_ACCU` reader - the current value of the averaging accumulator"]
pub type CurAvgAccuR = crate::FieldReader<u32>;
#[doc = "Field `INTRLV_BUSY` reader - If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
pub type IntrlvBusyR = crate::BitReader;
#[doc = "Field `CUR_AVG_CNT` reader - the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
pub type CurAvgCntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - the current value of the averaging accumulator"]
    #[inline(always)]
    pub fn cur_avg_accu(&self) -> CurAvgAccuR {
        CurAvgAccuR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 23 - If high then the SAR is in the middle of Interleaved averaging spanning several scans. While this bit is high the Firmware should not make any changes to the configuration registers otherwise some results may be incorrect. Note that the CUR_AVG_CNT status register below gives an indication how many more scans need to be done to complete the Interleaved averaging. This bit can be cleared by changing the averaging mode to ACCUNDUMP or by disabling the SAR."]
    #[inline(always)]
    pub fn intrlv_busy(&self) -> IntrlvBusyR {
        IntrlvBusyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - the current value of the averaging counter. Note that the value shown is updated after the sampling time and therefore runs ahead of the accumulator update."]
    #[inline(always)]
    pub fn cur_avg_cnt(&self) -> CurAvgCntR {
        CurAvgCntR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Current averaging status (for debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`avg_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AvgStatSpec;
impl crate::RegisterSpec for AvgStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`avg_stat::R`](R) reader structure"]
impl crate::Readable for AvgStatSpec {}
#[doc = "`reset()` method sets AVG_STAT to value 0"]
impl crate::Resettable for AvgStatSpec {
    const RESET_VALUE: u32 = 0;
}
