#[doc = "Register `DMAC_BUFF_CTL` reader"]
pub type R = crate::R<DmacBuffCtlSpec>;
#[doc = "Register `DMAC_BUFF_CTL` writer"]
pub type W = crate::W<DmacBuffCtlSpec>;
#[doc = "Field `PREF_EN` reader - See CRYPTO_BUFF_CTL."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - See CRYPTO_BUFF_CTL."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - See CRYPTO_BUFF_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<DmacBuffCtlSpec> {
        PrefEnW::new(self, 30)
    }
}
#[doc = "DMA controller buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmac_buff_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac_buff_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacBuffCtlSpec;
impl crate::RegisterSpec for DmacBuffCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_buff_ctl::R`](R) reader structure"]
impl crate::Readable for DmacBuffCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_buff_ctl::W`](W) writer structure"]
impl crate::Writable for DmacBuffCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_BUFF_CTL to value 0x4000_0000"]
impl crate::Resettable for DmacBuffCtlSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
