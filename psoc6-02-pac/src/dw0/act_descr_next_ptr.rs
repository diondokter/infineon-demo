#[doc = "Register `ACT_DESCR_NEXT_PTR` reader"]
pub type R = crate::R<ActDescrNextPtrSpec>;
#[doc = "Field `ADDR` reader - Copy of DESCR_NEXT_PTR of the currently active descriptor. \\[31:2\\]
ADDR Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - Copy of DESCR_NEXT_PTR of the currently active descriptor. \\[31:2\\]
ADDR Address of next descriptor in descriptor list. When this field is '0', this is the last descriptor in the descriptor list."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Active descriptor next pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_next_ptr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrNextPtrSpec;
impl crate::RegisterSpec for ActDescrNextPtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_next_ptr::R`](R) reader structure"]
impl crate::Readable for ActDescrNextPtrSpec {}
#[doc = "`reset()` method sets ACT_DESCR_NEXT_PTR to value 0"]
impl crate::Resettable for ActDescrNextPtrSpec {
    const RESET_VALUE: u32 = 0;
}
