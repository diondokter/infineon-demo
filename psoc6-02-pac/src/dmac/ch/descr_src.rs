#[doc = "Register `DESCR_SRC` reader"]
pub type R = crate::R<DescrSrcSpec>;
#[doc = "Field `ADDR` reader - Base address of source location."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of source location."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Channel descriptor source\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrSrcSpec;
impl crate::RegisterSpec for DescrSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_src::R`](R) reader structure"]
impl crate::Readable for DescrSrcSpec {}
#[doc = "`reset()` method sets DESCR_SRC to value 0"]
impl crate::Resettable for DescrSrcSpec {
    const RESET_VALUE: u32 = 0;
}
