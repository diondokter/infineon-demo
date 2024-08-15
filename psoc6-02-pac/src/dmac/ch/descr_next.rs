#[doc = "Register `DESCR_NEXT` reader"]
pub type R = crate::R<DescrNextSpec>;
#[doc = "Field `PTR` reader - Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
pub type PtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Channel descriptor next pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_next::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrNextSpec;
impl crate::RegisterSpec for DescrNextSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_next::R`](R) reader structure"]
impl crate::Readable for DescrNextSpec {}
#[doc = "`reset()` method sets DESCR_NEXT to value 0"]
impl crate::Resettable for DescrNextSpec {
    const RESET_VALUE: u32 = 0;
}
