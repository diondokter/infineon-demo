#[doc = "Register `CRYPTO_KEY0` writer"]
pub type W = crate::W<CryptoKey0Spec>;
#[doc = "Field `KEY` writer - Four Bytes of the key KEY\\[31:0\\]
= CRYPTO_KEY0.KEY\\[31:0\\]."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[31:0\\]
= CRYPTO_KEY0.KEY\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CryptoKey0Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Cryptography key 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoKey0Spec;
impl crate::RegisterSpec for CryptoKey0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_key0::W`](W) writer structure"]
impl crate::Writable for CryptoKey0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_KEY0 to value 0"]
impl crate::Resettable for CryptoKey0Spec {
    const RESET_VALUE: u32 = 0;
}
