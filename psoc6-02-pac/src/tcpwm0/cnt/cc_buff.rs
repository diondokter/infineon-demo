#[doc = "Register `CC_BUFF` reader"]
pub type R = crate::R<CcBuffSpec>;
#[doc = "Register `CC_BUFF` writer"]
pub type W = crate::W<CcBuffSpec>;
#[doc = "Field `CC` reader - Additional buffer for counter CC register."]
pub type CcR = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - Additional buffer for counter CC register."]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Additional buffer for counter CC register."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional buffer for counter CC register."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<CcBuffSpec> {
        CcW::new(self, 0)
    }
}
#[doc = "Counter buffered compare/capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_buff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_buff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcBuffSpec;
impl crate::RegisterSpec for CcBuffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_buff::R`](R) reader structure"]
impl crate::Readable for CcBuffSpec {}
#[doc = "`write(|w| ..)` method takes [`cc_buff::W`](W) writer structure"]
impl crate::Writable for CcBuffSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_BUFF to value 0xffff_ffff"]
impl crate::Resettable for CcBuffSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
