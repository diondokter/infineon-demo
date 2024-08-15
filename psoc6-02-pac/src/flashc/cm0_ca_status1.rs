#[doc = "Register `CM0_CA_STATUS1` reader"]
pub type R = crate::R<Cm0CaStatus1Spec>;
#[doc = "Field `TAG` reader - Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
pub type TagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cache line address of the cache line specified by CM0_CA_CTL.WAY and CM0_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits)
    }
}
#[doc = "CM0+ cache status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CaStatus1Spec;
impl crate::RegisterSpec for Cm0CaStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_status1::R`](R) reader structure"]
impl crate::Readable for Cm0CaStatus1Spec {}
#[doc = "`reset()` method sets CM0_CA_STATUS1 to value 0"]
impl crate::Resettable for Cm0CaStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
