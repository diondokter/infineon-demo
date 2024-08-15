#[doc = "Register `CM4_CA_STATUS1` reader"]
pub type R = crate::R<Cm4CaStatus1Spec>;
#[doc = "Field `TAG` reader - See CM0_CA_STATUS1."]
pub type TagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS1."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits)
    }
}
#[doc = "CM4 cache status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4CaStatus1Spec;
impl crate::RegisterSpec for Cm4CaStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_status1::R`](R) reader structure"]
impl crate::Readable for Cm4CaStatus1Spec {}
#[doc = "`reset()` method sets CM4_CA_STATUS1 to value 0"]
impl crate::Resettable for Cm4CaStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
