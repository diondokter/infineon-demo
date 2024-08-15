#[doc = "Register `CRYPTO_BUFF_CTL` reader"]
pub type R = crate::R<CryptoBuffCtlSpec>;
#[doc = "Register `CRYPTO_BUFF_CTL` writer"]
pub type W = crate::W<CryptoBuffCtlSpec>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. A prefetch will be done when there is read 'hit' on the last 32-bit word of the buffer. For eCT work Flash, prefetch will not be done."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<CryptoBuffCtlSpec> {
        PrefEnW::new(self, 30)
    }
}
#[doc = "Cryptography buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_buff_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_buff_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CryptoBuffCtlSpec;
impl crate::RegisterSpec for CryptoBuffCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_buff_ctl::R`](R) reader structure"]
impl crate::Readable for CryptoBuffCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`crypto_buff_ctl::W`](W) writer structure"]
impl crate::Writable for CryptoBuffCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRYPTO_BUFF_CTL to value 0x4000_0000"]
impl crate::Resettable for CryptoBuffCtlSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
