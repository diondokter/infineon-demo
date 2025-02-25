#[doc = "Register `CC` reader"]
pub type R = crate::R<CcSpec>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CcSpec>;
#[doc = "Field `CC` reader - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
pub type CcR = crate::FieldReader<u32>;
#[doc = "Field `CC` writer - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
pub type CcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - In CAPTURE mode, captures the counter value. In other modes, compared to counter value."]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CcW<CcSpec> {
        CcW::new(self, 0)
    }
}
#[doc = "Counter compare/capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcSpec;
impl crate::RegisterSpec for CcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC to value 0xffff_ffff"]
impl crate::Resettable for CcSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
