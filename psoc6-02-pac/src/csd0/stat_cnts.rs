#[doc = "Register `STAT_CNTS` reader"]
pub type R = crate::R<StatCntsSpec>;
#[doc = "Field `NUM_CONV` reader - Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
pub type NumConvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current number of conversions remaining when in Sample_* states (note that in AutoZero* states the same down counter is reused to count the cycles)"]
    #[inline(always)]
    pub fn num_conv(&self) -> NumConvR {
        NumConvR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current status counts\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_cnts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatCntsSpec;
impl crate::RegisterSpec for StatCntsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_cnts::R`](R) reader structure"]
impl crate::Readable for StatCntsSpec {}
#[doc = "`reset()` method sets STAT_CNTS to value 0"]
impl crate::Resettable for StatCntsSpec {
    const RESET_VALUE: u32 = 0;
}
