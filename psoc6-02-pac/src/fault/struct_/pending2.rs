#[doc = "Register `PENDING2` reader"]
pub type R = crate::R<Pending2Spec>;
#[doc = "Field `SOURCE` reader - This field specifies the following sources: Bit 0 - 31: See STATUS register."]
pub type SourceR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This field specifies the following sources: Bit 0 - 31: See STATUS register."]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits)
    }
}
#[doc = "Fault pending 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pending2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pending2Spec;
impl crate::RegisterSpec for Pending2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending2::R`](R) reader structure"]
impl crate::Readable for Pending2Spec {}
#[doc = "`reset()` method sets PENDING2 to value 0"]
impl crate::Resettable for Pending2Spec {
    const RESET_VALUE: u32 = 0;
}
