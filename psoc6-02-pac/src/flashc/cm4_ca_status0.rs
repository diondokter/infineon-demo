#[doc = "Register `CM4_CA_STATUS0` reader"]
pub type R = crate::R<Cm4CaStatus0Spec>;
#[doc = "Field `VALID32` reader - See CM0_CA_STATUS0."]
pub type Valid32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - See CM0_CA_STATUS0."]
    #[inline(always)]
    pub fn valid32(&self) -> Valid32R {
        Valid32R::new(self.bits)
    }
}
#[doc = "CM4 cache status 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_status0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4CaStatus0Spec;
impl crate::RegisterSpec for Cm4CaStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_status0::R`](R) reader structure"]
impl crate::Readable for Cm4CaStatus0Spec {}
#[doc = "`reset()` method sets CM4_CA_STATUS0 to value 0"]
impl crate::Resettable for Cm4CaStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
