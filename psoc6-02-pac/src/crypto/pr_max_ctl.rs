#[doc = "Register `PR_MAX_CTL` reader"]
pub type R = crate::R<PrMaxCtlSpec>;
#[doc = "Register `PR_MAX_CTL` writer"]
pub type W = crate::W<PrMaxCtlSpec>;
#[doc = "Field `DATA32` reader - Maximum value of to be generated random number"]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Maximum value of to be generated random number"]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum value of to be generated random number"]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Maximum value of to be generated random number"]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<PrMaxCtlSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "Pseudo random maximum control\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_max_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_max_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrMaxCtlSpec;
impl crate::RegisterSpec for PrMaxCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_max_ctl::R`](R) reader structure"]
impl crate::Readable for PrMaxCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pr_max_ctl::W`](W) writer structure"]
impl crate::Writable for PrMaxCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_MAX_CTL to value 0xffff_ffff"]
impl crate::Resettable for PrMaxCtlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
