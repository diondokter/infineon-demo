#[doc = "Register `PR_LFSR_CTL0` reader"]
pub type R = crate::R<PrLfsrCtl0Spec>;
#[doc = "Register `PR_LFSR_CTL0` writer"]
pub type W = crate::W<PrLfsrCtl0Spec>;
#[doc = "Field `LFSR32` reader - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
pub type Lfsr32R = crate::FieldReader<u32>;
#[doc = "Field `LFSR32` writer - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
pub type Lfsr32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
    #[inline(always)]
    pub fn lfsr32(&self) -> Lfsr32R {
        Lfsr32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr32(&mut self) -> Lfsr32W<PrLfsrCtl0Spec> {
        Lfsr32W::new(self, 0)
    }
}
#[doc = "Pseudo random LFSR control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pr_lfsr_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr_lfsr_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrLfsrCtl0Spec;
impl crate::RegisterSpec for PrLfsrCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr_lfsr_ctl0::R`](R) reader structure"]
impl crate::Readable for PrLfsrCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr_lfsr_ctl0::W`](W) writer structure"]
impl crate::Writable for PrLfsrCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL0 to value 0xd895_9bc9"]
impl crate::Resettable for PrLfsrCtl0Spec {
    const RESET_VALUE: u32 = 0xd895_9bc9;
}
