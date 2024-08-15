#[doc = "Register `CRYPTO_INPUT3` reader"]
pub type R = crate::R<CryptoInput3Spec>;
#[doc = "Register `CRYPTO_INPUT3` writer"]
pub type W = crate::W<CryptoInput3Spec>;
#[doc = "Field `INPUT` reader - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type InputR = crate::FieldReader<u32>;
#[doc = "Field `INPUT` writer - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
pub type InputW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn input(&self) -> InputR {
        InputR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the plaintext PT\\[127:96\\]
= CRYPTO_INPUT3.INPUT\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn input(&mut self) -> InputW<CryptoInput3Spec> {
        InputW::new(self, 0)
    }
}
#[doc = "Cryptography input 3\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_input3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_input3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoInput3Spec;
impl crate::RegisterSpec for CryptoInput3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_input3::R`](R) reader structure"]
impl crate::Readable for CryptoInput3Spec {}
#[doc = "`write(|w| ..)` method takes [`crypto_input3::W`](W) writer structure"]
impl crate::Writable for CryptoInput3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_INPUT3 to value 0"]
impl crate::Resettable for CryptoInput3Spec {
    const RESET_VALUE: u32 = 0;
}
