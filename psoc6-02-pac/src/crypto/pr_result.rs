#[doc = "Register `PR_RESULT` reader"]
pub type R = crate::R<PrResultSpec>;
#[doc = "Register `PR_RESULT` writer"]
pub type W = crate::W<PrResultSpec>;
#[doc = "Field `DATA32` reader - Result of a pseudo random number generation operation. The resulting value DATA is in the range \\[0, PR_MAX_CTL.DATA32\\]. The PR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Result of a pseudo random number generation operation. The resulting value DATA is in the range \\[0, PR_MAX_CTL.DATA32\\]. The PR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Result of a pseudo random number generation operation. The resulting value DATA is in the range \\[0, PR_MAX_CTL.DATA32\\]. The PR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Result of a pseudo random number generation operation. The resulting value DATA is in the range \\[0, PR_MAX_CTL.DATA32\\]. The PR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<PrResultSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "Pseudo random result\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrResultSpec;
impl crate::RegisterSpec for PrResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_result::R`](R) reader structure"]
impl crate::Readable for PrResultSpec {}
#[doc = "`write(|w| ..)` method takes [`pr_result::W`](W) writer structure"]
impl crate::Writable for PrResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_RESULT to value 0"]
impl crate::Resettable for PrResultSpec {
    const RESET_VALUE: u32 = 0;
}
