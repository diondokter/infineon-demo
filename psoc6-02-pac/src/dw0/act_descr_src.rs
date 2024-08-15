#[doc = "Register `ACT_DESCR_SRC` reader"]
pub type R = crate::R<ActDescrSrcSpec>;
#[doc = "Field `DATA` reader - Copy of DESCR_SRC of the currently active descriptor. Base address of source location."]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Copy of DESCR_SRC of the currently active descriptor. Base address of source location."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Active descriptor source\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrSrcSpec;
impl crate::RegisterSpec for ActDescrSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_src::R`](R) reader structure"]
impl crate::Readable for ActDescrSrcSpec {}
#[doc = "`reset()` method sets ACT_DESCR_SRC to value 0"]
impl crate::Resettable for ActDescrSrcSpec {
    const RESET_VALUE: u32 = 0;
}
