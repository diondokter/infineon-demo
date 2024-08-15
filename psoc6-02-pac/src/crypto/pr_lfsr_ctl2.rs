#[doc = "Register `PR_LFSR_CTL2` reader"]
pub type R = crate::R<PrLfsrCtl2Spec>;
#[doc = "Register `PR_LFSR_CTL2` writer"]
pub type W = crate::W<PrLfsrCtl2Spec>;
#[doc = "Field `LFSR29` reader - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type Lfsr29R = crate::FieldReader<u32>;
#[doc = "Field `LFSR29` writer - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type Lfsr29W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    pub fn lfsr29(&self) -> Lfsr29R {
        Lfsr29R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr29(&mut self) -> Lfsr29W<PrLfsrCtl2Spec> {
        Lfsr29W::new(self, 0)
    }
}
#[doc = "Pseudo random LFSR control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrLfsrCtl2Spec;
impl crate::RegisterSpec for PrLfsrCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_lfsr_ctl2::R`](R) reader structure"]
impl crate::Readable for PrLfsrCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr_lfsr_ctl2::W`](W) writer structure"]
impl crate::Writable for PrLfsrCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL2 to value 0x060c_31b7"]
impl crate::Resettable for PrLfsrCtl2Spec {
    const RESET_VALUE: u32 = 0x060c_31b7;
}
