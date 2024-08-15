#[doc = "Register `DESCR_DST` reader"]
pub type R = crate::R<DescrDstSpec>;
#[doc = "Field `ADDR` reader - Base address of destination location."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of destination location."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "Channel descriptor destination\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrDstSpec;
impl crate::RegisterSpec for DescrDstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_dst::R`](R) reader structure"]
impl crate::Readable for DescrDstSpec {}
#[doc = "`reset()` method sets DESCR_DST to value 0"]
impl crate::Resettable for DescrDstSpec {
    const RESET_VALUE: u32 = 0;
}
