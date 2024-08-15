#[doc = "Register `ACT_SRC` reader"]
pub type R = crate::R<ActSrcSpec>;
#[doc = "Field `SRC_ADDR` reader - Current address of source location."]
pub type SrcAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of source location."]
    #[inline(always)]
    pub fn src_addr(&self) -> SrcAddrR {
        SrcAddrR::new(self.bits)
    }
}
#[doc = "Active source\n\nYou can [`read`](crate::Reg::read) this register and get [`act_src::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActSrcSpec;
impl crate::RegisterSpec for ActSrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_src::R`](R) reader structure"]
impl crate::Readable for ActSrcSpec {}
#[doc = "`reset()` method sets ACT_SRC to value 0"]
impl crate::Resettable for ActSrcSpec {
    const RESET_VALUE: u32 = 0;
}
