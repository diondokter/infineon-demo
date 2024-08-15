#[doc = "Register `CM0_CA_STATUS2` reader"]
pub type R = crate::R<Cm0CaStatus2Spec>;
#[doc = "Field `LRU` reader - Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
pub type LruR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Six bit LRU representation of the cache set specified by CM0_CA_CTL.SET_ADDR. The encoding of the field is as follows ('X_LRU_Y' indicates that way X is Less Recently Used than way Y): Bit 5: 0_LRU_1: way 0 less recently used than way 1. Bit 4: 0_LRU_2. Bit 3: 0_LRU_3. Bit 2: 1_LRU_2. Bit 1: 1_LRU_3. Bit 0: 2_LRU_3."]
    #[inline(always)]
    pub fn lru(&self) -> LruR {
        LruR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "CM0+ cache status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CaStatus2Spec;
impl crate::RegisterSpec for Cm0CaStatus2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_status2::R`](R) reader structure"]
impl crate::Readable for Cm0CaStatus2Spec {}
#[doc = "`reset()` method sets CM0_CA_STATUS2 to value 0"]
impl crate::Resettable for Cm0CaStatus2Spec {
    const RESET_VALUE: u32 = 0;
}
