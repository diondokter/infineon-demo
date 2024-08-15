#[doc = "Register `RESULT_VAL2` reader"]
pub type R = crate::R<ResultVal2Spec>;
#[doc = "Field `VALUE` reader - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt), this counter counts when csd_sense is low."]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Only used in case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt), this counter counts when csd_sense is low."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Result CSX accumulation counter value 2\n\nYou can [`read`](crate::Reg::read) this register and get [`result_val2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultVal2Spec;
impl crate::RegisterSpec for ResultVal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_val2::R`](R) reader structure"]
impl crate::Readable for ResultVal2Spec {}
#[doc = "`reset()` method sets RESULT_VAL2 to value 0"]
impl crate::Resettable for ResultVal2Spec {
    const RESET_VALUE: u32 = 0;
}
