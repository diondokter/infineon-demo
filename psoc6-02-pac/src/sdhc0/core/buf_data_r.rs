#[doc = "Register `BUF_DATA_R` reader"]
pub type R = crate::R<BufDataRSpec>;
#[doc = "Register `BUF_DATA_R` writer"]
pub type W = crate::W<BufDataRSpec>;
#[doc = "Field `BUF_DATA` reader - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BufDataR = crate::FieldReader<u32>;
#[doc = "Field `BUF_DATA` writer - Buffer Data These bits enable access to the Host Controller packet buffer."]
pub type BufDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    pub fn buf_data(&self) -> BufDataR {
        BufDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data These bits enable access to the Host Controller packet buffer."]
    #[inline(always)]
    #[must_use]
    pub fn buf_data(&mut self) -> BufDataW<BufDataRSpec> {
        BufDataW::new(self, 0)
    }
}
#[doc = "Buffer Data Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_data_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_data_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufDataRSpec;
impl crate::RegisterSpec for BufDataRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_data_r::R`](R) reader structure"]
impl crate::Readable for BufDataRSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_data_r::W`](W) writer structure"]
impl crate::Writable for BufDataRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF_DATA_R to value 0"]
impl crate::Resettable for BufDataRSpec {
    const RESET_VALUE: u32 = 0;
}
