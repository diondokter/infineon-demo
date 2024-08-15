#[doc = "Register `CM4_CA_STATUS2` reader"]
pub type R = crate::R<Cm4CaStatus2Spec>;
#[doc = "Field `LRU` reader - See CM0_CA_STATUS2."]
pub type LruR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - See CM0_CA_STATUS2."]
    #[inline(always)]
    pub fn lru(&self) -> LruR {
        LruR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CM4 cache status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4CaStatus2Spec;
impl crate::RegisterSpec for Cm4CaStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_status2::R`](R) reader structure"]
impl crate::Readable for Cm4CaStatus2Spec {}
#[doc = "`reset()` method sets CM4_CA_STATUS2 to value 0"]
impl crate::Resettable for Cm4CaStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
