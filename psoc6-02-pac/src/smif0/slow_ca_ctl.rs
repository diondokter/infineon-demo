#[doc = "Register `SLOW_CA_CTL` reader"]
pub type R = crate::R<SlowCaCtlSpec>;
#[doc = "Register `SLOW_CA_CTL` writer"]
pub type W = crate::W<SlowCaCtlSpec>;
#[doc = "Field `WAY` reader - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type WayR = crate::FieldReader;
#[doc = "Field `WAY` writer - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type WayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type SetAddrR = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
pub type SetAddrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREF_EN` reader - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Cache enable: '0': Disabled. '1': Enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Cache enable: '0': Disabled. '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn way(&self) -> WayR {
        WayR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    pub fn set_addr(&self) -> SetAddrR {
        SetAddrR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - Specifies the cache way for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WayW<SlowCaCtlSpec> {
        WayW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Specifies the cache set for which cache information is provided in SLOW_CA_STATUS0/1/2."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SetAddrW<SlowCaCtlSpec> {
        SetAddrW::new(self, 24)
    }
    #[doc = "Bit 30 - Prefetch enable: '0': Disabled. '1': Enabled. Prefetching requires the cache to be enabled; i.e. ENABLED is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<SlowCaCtlSpec> {
        PrefEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Cache enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<SlowCaCtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Slow cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`slow_ca_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slow_ca_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlowCaCtlSpec;
impl crate::RegisterSpec for SlowCaCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slow_ca_ctl::R`](R) reader structure"]
impl crate::Readable for SlowCaCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`slow_ca_ctl::W`](W) writer structure"]
impl crate::Writable for SlowCaCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLOW_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for SlowCaCtlSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
