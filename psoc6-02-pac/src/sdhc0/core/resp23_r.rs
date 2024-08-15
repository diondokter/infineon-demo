#[doc = "Register `RESP23_R` reader"]
pub type R = crate::R<Resp23RSpec>;
#[doc = "Field `RESP23` reader - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
pub type Resp23R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 71-40 bits of the SD/eMMC Response"]
    #[inline(always)]
    pub fn resp23(&self) -> Resp23R {
        Resp23R::new(self.bits)
    }
}
#[doc = "Response Register 2/3\n\nYou can [`read`](crate::Reg::read) this register and get [`resp23_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp23RSpec;
impl crate::RegisterSpec for Resp23RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp23_r::R`](R) reader structure"]
impl crate::Readable for Resp23RSpec {}
#[doc = "`reset()` method sets RESP23_R to value 0"]
impl crate::Resettable for Resp23RSpec {
    const RESET_VALUE: u32 = 0;
}
