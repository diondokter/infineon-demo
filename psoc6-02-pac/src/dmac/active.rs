#[doc = "Register `ACTIVE` reader"]
pub type R = crate::R<ActiveSpec>;
#[doc = "Field `ACTIVE` reader - Specifies active channels; i.e. enabled channels whose trigger got activated."]
pub type ActiveR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Specifies active channels; i.e. enabled channels whose trigger got activated."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Active channels\n\nYou can [`read`](crate::Reg::read) this register and get [`active::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActiveSpec;
impl crate::RegisterSpec for ActiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`active::R`](R) reader structure"]
impl crate::Readable for ActiveSpec {}
#[doc = "`reset()` method sets ACTIVE to value 0"]
impl crate::Resettable for ActiveSpec {
    const RESET_VALUE: u32 = 0;
}
