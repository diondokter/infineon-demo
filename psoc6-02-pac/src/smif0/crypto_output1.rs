#[doc = "Register `CRYPTO_OUTPUT1` reader"]
pub type R = crate::R<CryptoOutput1Spec>;
#[doc = "Register `CRYPTO_OUTPUT1` writer"]
pub type W = crate::W<CryptoOutput1Spec>;
#[doc = "Field `OUTPUT` reader - Four Bytes of the ciphertext CT\\[63:32\\]
= CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
pub type OutputR = crate::FieldReader<u32>;
#[doc = "Field `OUTPUT` writer - Four Bytes of the ciphertext CT\\[63:32\\]
= CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
pub type OutputW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[63:32\\]
= CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    pub fn output(&self) -> OutputR {
        OutputR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four Bytes of the ciphertext CT\\[63:32\\]
= CRYPTO_OUTPUT1.OUTPUT\\[31:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn output(&mut self) -> OutputW<CryptoOutput1Spec> {
        OutputW::new(self, 0)
    }
}
#[doc = "Cryptography output 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_output1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_output1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoOutput1Spec;
impl crate::RegisterSpec for CryptoOutput1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_output1::R`](R) reader structure"]
impl crate::Readable for CryptoOutput1Spec {}
#[doc = "`write(|w| ..)` method takes [`crypto_output1::W`](W) writer structure"]
impl crate::Writable for CryptoOutput1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_OUTPUT1 to value 0"]
impl crate::Resettable for CryptoOutput1Spec {
    const RESET_VALUE: u32 = 0;
}
