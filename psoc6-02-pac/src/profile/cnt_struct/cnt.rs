#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNT` reader - This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - This field shows / specifies the actual value of the profiling counter. It allows reading as well as writing the profiling counter."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Profile counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {
    const RESET_VALUE: u32 = 0;
}
