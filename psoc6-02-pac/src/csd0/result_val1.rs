#[doc = "Register `RESULT_VAL1` reader"]
pub type R = crate::R<ResultVal1Spec>;
#[doc = "Field `VALUE` reader - Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt) this counter counts when csd_sense is high."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `BAD_CONVS` reader - Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
pub type BadConvsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Accumulated counter value for this result. In case of Mutual cap with two counters (CSX = config.mutual_cap &amp; config.csx_dual_cnt) this counter counts when csd_sense is high."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Number of 'bad' conversion for which the CSD comparator did not trigger within the normal time window, either because Vref was not crossed at all, or if the Vref was already crossed before the window started. This counter is reset when the sequencer is started and will saturate at 255 when more than 255 conversions are bad."]
    #[inline(always)]
    pub fn bad_convs(&self) -> BadConvsR {
        BadConvsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Result CSD/CSX accumulation counter value 1\n\nYou can [`read`](crate::Reg::read) this register and get [`result_val1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultVal1Spec;
impl crate::RegisterSpec for ResultVal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_val1::R`](R) reader structure"]
impl crate::Readable for ResultVal1Spec {}
#[doc = "`reset()` method sets RESULT_VAL1 to value 0"]
impl crate::Resettable for ResultVal1Spec {
    const RESET_VALUE: u32 = 0;
}
