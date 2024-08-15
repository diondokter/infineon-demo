#[doc = "Register `PR_LFSR_CTL1` reader"]
pub type R = crate::R<PrLfsrCtl1Spec>;
#[doc = "Register `PR_LFSR_CTL1` writer"]
pub type W = crate::W<PrLfsrCtl1Spec>;
#[doc = "Field `LFSR31` reader - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type Lfsr31R = crate::FieldReader<u32>;
#[doc = "Field `LFSR31` writer - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type Lfsr31W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    pub fn lfsr31(&self) -> Lfsr31R {
        Lfsr31R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr31(&mut self) -> Lfsr31W<PrLfsrCtl1Spec> {
        Lfsr31W::new(self, 0)
    }
}
#[doc = "Pseudo random LFSR control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrLfsrCtl1Spec;
impl crate::RegisterSpec for PrLfsrCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_lfsr_ctl1::R`](R) reader structure"]
impl crate::Readable for PrLfsrCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr_lfsr_ctl1::W`](W) writer structure"]
impl crate::Writable for PrLfsrCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL1 to value 0x2bb9_11f8"]
impl crate::Resettable for PrLfsrCtl1Spec {
    const RESET_VALUE: u32 = 0x2bb9_11f8;
}
