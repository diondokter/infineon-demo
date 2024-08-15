#[doc = "Register `RESP45_R` reader"]
pub type R = crate::R<Resp45RSpec>;
#[doc = "Field `RESP45` reader - Command Response These bits reflect 103-72 bits of the Response Field."]
pub type Resp45R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 103-72 bits of the Response Field."]
    #[inline(always)]
    pub fn resp45(&self) -> Resp45R {
        Resp45R::new(self.bits)
    }
}
#[doc = "Response Register 4/5\n\nYou can [`read`](crate::Reg::read) this register and get [`resp45_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp45RSpec;
impl crate::RegisterSpec for Resp45RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp45_r::R`](R) reader structure"]
impl crate::Readable for Resp45RSpec {}
#[doc = "`reset()` method sets RESP45_R to value 0"]
impl crate::Resettable for Resp45RSpec {
    const RESET_VALUE: u32 = 0;
}
