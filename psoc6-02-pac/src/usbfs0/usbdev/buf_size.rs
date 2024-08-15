#[doc = "Register `BUF_SIZE` reader"]
pub type R = crate::R<BufSizeSpec>;
#[doc = "Register `BUF_SIZE` writer"]
pub type W = crate::W<BufSizeSpec>;
#[doc = "Field `IN_BUF` reader - Buffer size for IN Endpoints."]
pub type InBufR = crate::FieldReader;
#[doc = "Field `IN_BUF` writer - Buffer size for IN Endpoints."]
pub type InBufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_BUF` reader - Buffer size for OUT Endpoints."]
pub type OutBufR = crate::FieldReader;
#[doc = "Field `OUT_BUF` writer - Buffer size for OUT Endpoints."]
pub type OutBufW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    pub fn in_buf(&self) -> InBufR {
        InBufR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    pub fn out_buf(&self) -> OutBufR {
        OutBufR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Buffer size for IN Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn in_buf(&mut self) -> InBufW<BufSizeSpec> {
        InBufW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Buffer size for OUT Endpoints."]
    #[inline(always)]
    #[must_use]
    pub fn out_buf(&mut self) -> OutBufW<BufSizeSpec> {
        OutBufW::new(self, 4)
    }
}
#[doc = "Dedicated Endpoint Buffer Size Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`buf_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufSizeSpec;
impl crate::RegisterSpec for BufSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf_size::R`](R) reader structure"]
impl crate::Readable for BufSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`buf_size::W`](W) writer structure"]
impl crate::Writable for BufSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF_SIZE to value 0"]
impl crate::Resettable for BufSizeSpec {
    const RESET_VALUE: u32 = 0;
}
