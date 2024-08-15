#[doc = "Register `CM0_CA_CTL0` reader"]
pub type R = crate::R<Cm0CaCtl0Spec>;
#[doc = "Register `CM0_CA_CTL0` writer"]
pub type W = crate::W<Cm0CaCtl0Spec>;
#[doc = "Field `RAM_ECC_EN` reader - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
pub type RamEccEnR = crate::BitReader;
#[doc = "Field `RAM_ECC_EN` writer - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
pub type RamEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_ECC_INJ_EN` reader - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type RamEccInjEnR = crate::BitReader;
#[doc = "Field `RAM_ECC_INJ_EN` writer - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
pub type RamEccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAY` reader - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WayR = crate::FieldReader;
#[doc = "Field `WAY` writer - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type WayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SetAddrR = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
pub type SetAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CA_EN` reader - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type CaEnR = crate::BitReader;
#[doc = "Field `CA_EN` writer - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
pub type CaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    pub fn ram_ecc_en(&self) -> RamEccEnR {
        RamEccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    pub fn ram_ecc_inj_en(&self) -> RamEccInjEnR {
        RamEccInjEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&self) -> WayR {
        WayR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&self) -> SetAddrR {
        SetAddrR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    pub fn ca_en(&self) -> CaEnR {
        CaEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC checking for cache accesses: 0: Disabled. 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_en(&mut self) -> RamEccEnW<Cm0CaCtl0Spec> {
        RamEccEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable error injection for cache. When '1', the parity (ECC_CTL.PARITY\\[6:0\\]) is used when a refill is done from the FLASH macro to the ECC_CTL.WORD_ADDR\\[23:0\\]
word address."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_inj_en(&mut self) -> RamEccInjEnW<Cm0CaCtl0Spec> {
        RamEccInjEnW::new(self, 1)
    }
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WayW<Cm0CaCtl0Spec> {
        WayW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Specifies the cache set for which cache information is provided in CM0_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SetAddrW<Cm0CaCtl0Spec> {
        SetAddrW::new(self, 24)
    }
    #[doc = "Bit 30 - Prefetch enable: 0: Disabled. 1: Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<Cm0CaCtl0Spec> {
        PrefEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Cache enable: 0: Disabled. The cache tag valid bits are reset to '0's and the cache LRU information is set to '1's (making way 0 the LRU way and way 3 the MRU way). 1: Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ca_en(&mut self) -> CaEnW<Cm0CaCtl0Spec> {
        CaEnW::new(self, 31)
    }
}
#[doc = "CM0+ cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_ca_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_ca_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0CaCtl0Spec;
impl crate::RegisterSpec for Cm0CaCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_ca_ctl0::R`](R) reader structure"]
impl crate::Readable for Cm0CaCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cm0_ca_ctl0::W`](W) writer structure"]
impl crate::Writable for Cm0CaCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_CA_CTL0 to value 0xc000_0001"]
impl crate::Resettable for Cm0CaCtl0Spec {
    const RESET_VALUE: u32 = 0xc000_0001;
}
