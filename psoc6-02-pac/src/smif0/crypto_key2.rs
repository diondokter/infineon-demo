#[doc = "Register `CRYPTO_KEY2` writer"]
pub type W = crate::W<CryptoKey2Spec>;
#[doc = "Field `KEY` writer - Four Bytes of the key KEY\\[95:64\\]
= CRYPTO_KEY2.KEY\\[31:0\\]."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the key KEY\\[95:64\\]
= CRYPTO_KEY2.KEY\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CryptoKey2Spec> {
        KeyW::new(self, 0)
    }
}
#[doc = "Cryptography key 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoKey2Spec;
impl crate::RegisterSpec for CryptoKey2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`crypto_key2::W`](W) writer structure"]
impl crate::Writable for CryptoKey2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_KEY2 to value 0"]
impl crate::Resettable for CryptoKey2Spec {
    const RESET_VALUE: u32 = 0;
}
