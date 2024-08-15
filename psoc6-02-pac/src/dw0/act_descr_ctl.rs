#[doc = "Register `ACT_DESCR_CTL` reader"]
pub type R = crate::R<ActDescrCtlSpec>;
#[doc = "Field `DATA` reader - N/A"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Active descriptor control\n\nYou can [`read`](crate::Reg::read) this register and get [`act_descr_ctl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActDescrCtlSpec;
impl crate::RegisterSpec for ActDescrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`act_descr_ctl::R`](R) reader structure"]
impl crate::Readable for ActDescrCtlSpec {}
#[doc = "`reset()` method sets ACT_DESCR_CTL to value 0"]
impl crate::Resettable for ActDescrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
