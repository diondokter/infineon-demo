#[doc = "Register `SRC` reader"]
pub type R = crate::R<SrcSpec>;
#[doc = "Field `ADDR` reader - Current address of source location."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Channel current source address\n\nYou can [`read`](crate::Reg::read) this register and get [`src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcSpec;
impl crate::RegisterSpec for SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`src::R`](R) reader structure"]
impl crate::Readable for SrcSpec {}
#[doc = "`reset()` method sets SRC to value 0"]
impl crate::Resettable for SrcSpec {
    const RESET_VALUE: u32 = 0;
}
