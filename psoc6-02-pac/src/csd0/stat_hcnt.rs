#[doc = "Register `STAT_HCNT` reader"]
pub type R = crate::R<StatHcntSpec>;
#[doc = "Field `CNT` reader - Current value of HSCMP counter"]
pub type CntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of HSCMP counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Current count of the HSCMP counter\n\nYou can [`read`](crate::Reg::read) this register and get [`stat_hcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatHcntSpec;
impl crate::RegisterSpec for StatHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat_hcnt::R`](R) reader structure"]
impl crate::Readable for StatHcntSpec {}
#[doc = "`reset()` method sets STAT_HCNT to value 0"]
impl crate::Resettable for StatHcntSpec {
    const RESET_VALUE: u32 = 0;
}
