#[doc = "Register `DST` reader"]
pub type R = crate::R<DstSpec>;
#[doc = "Field `ADDR` reader - Current address of destination location."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Channel current destination address\n\nYou can [`read`](crate::Reg::read) this register and get [`dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DstSpec;
impl crate::RegisterSpec for DstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dst::R`](R) reader structure"]
impl crate::Readable for DstSpec {}
#[doc = "`reset()` method sets DST to value 0"]
impl crate::Resettable for DstSpec {
    const RESET_VALUE: u32 = 0;
}
