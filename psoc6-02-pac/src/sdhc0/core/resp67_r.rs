#[doc = "Register `RESP67_R` reader"]
pub type R = crate::R<Resp67RSpec>;
#[doc = "Field `RESP67` reader - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
pub type Resp67R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response These bits reflect bits 135-104 of SD/EMMC Response Field. Note: For Auto CMD, this register also reflects the 32-bit response (bits 39-8 of the Response Field)."]
    #[inline(always)]
    pub fn resp67(&self) -> Resp67R {
        Resp67R::new(self.bits)
    }
}
#[doc = "Response Register 6/7\n\nYou can [`read`](crate::Reg::read) this register and get [`resp67_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Resp67RSpec;
impl crate::RegisterSpec for Resp67RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resp67_r::R`](R) reader structure"]
impl crate::Readable for Resp67RSpec {}
#[doc = "`reset()` method sets RESP67_R to value 0"]
impl crate::Resettable for Resp67RSpec {
    const RESET_VALUE: u32 = 0;
}
