#[doc = "Register `RESP01_R` reader"]
pub type R = crate::R<Resp01RSpec>;
#[doc = "Field `RESP01` reader - Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
pub type Resp01R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect 39-8 bits of SD/eMMC Response Field. Note: For Auto CMD, the 32-bit response (bits 39-8 of the Response Field) is updated in the RESP67_R register."]
    #[inline(always)]
    pub fn resp01(&self) -> Resp01R {
        Resp01R::new(self.bits)
    }
}
#[doc = "Response Register 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`resp01_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp01RSpec;
impl crate::RegisterSpec for Resp01RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp01_r::R`](R) reader structure"]
impl crate::Readable for Resp01RSpec {}
#[doc = "`reset()` method sets RESP01_R to value 0"]
impl crate::Resettable for Resp01RSpec {
    const RESET_VALUE: u32 = 0;
}
