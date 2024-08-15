#[doc = "Register `ACT_DST` reader"]
pub type R = crate::R<ActDstSpec>;
#[doc = "Field `DST_ADDR` reader - Current address of destination location."]
pub type DstAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current address of destination location."]
    #[inline(always)]
    pub fn dst_addr(&self) -> DstAddrR {
        DstAddrR::new(self.bits)
    }
}
#[doc = "Active destination\n\nYou can [`read`](crate::Reg::read) this register and get [`act_dst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDstSpec;
impl crate::RegisterSpec for ActDstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_dst::R`](R) reader structure"]
impl crate::Readable for ActDstSpec {}
#[doc = "`reset()` method sets ACT_DST to value 0"]
impl crate::Resettable for ActDstSpec {
    const RESET_VALUE: u32 = 0;
}
