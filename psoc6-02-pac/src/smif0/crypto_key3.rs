#[doc = "Register `CRYPTO_KEY3` writer"]
pub type W = crate::W<CryptoKey3Spec>;
#[doc = "Field `KEY` writer - Four Bytes of the key KEY\\[127:96\\]
= CRYPTO_KEY3.KEY\\[31:0\\]."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[127:96\\]
= CRYPTO_KEY3.KEY\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CryptoKey3Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Cryptography key 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoKey3Spec;
impl crate::RegisterSpec for CryptoKey3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_key3::W`](W) writer structure"]
impl crate::Writable for CryptoKey3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_KEY3 to value 0"]
impl crate::Resettable for CryptoKey3Spec {
    const RESET_VALUE: u32 = 0;
}
