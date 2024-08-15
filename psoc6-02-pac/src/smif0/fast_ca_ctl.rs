#[doc = "Register `FAST_CA_CTL` reader"]
pub type R = crate::R<FastCaCtlSpec>;
#[doc = "Register `FAST_CA_CTL` writer"]
pub type W = crate::W<FastCaCtlSpec>;
#[doc = "Field `WAY` reader - See SLOW_CA_CTL.WAY."]
pub type WayR = crate::FieldReader;
#[doc = "Field `WAY` writer - See SLOW_CA_CTL.WAY."]
pub type WayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - See SLOW_CA_CTL.SET_ADDR."]
pub type SetAddrR = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - See SLOW_CA_CTL.SET_ADDR."]
pub type SetAddrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PREF_EN` reader - See SLOW_CA_CTL.PREF_EN."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - See SLOW_CA_CTL.PREF_EN."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - See SLOW_CA_CTL.ENABLED."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - See SLOW_CA_CTL.ENABLED."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    pub fn way(&self) -> WayR {
        WayR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    pub fn set_addr(&self) -> SetAddrR {
        SetAddrR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - See SLOW_CA_CTL.WAY."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WayW<FastCaCtlSpec> {
        WayW::new(self, 16)
    }
    #[doc = "Bits 24:25 - See SLOW_CA_CTL.SET_ADDR."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SetAddrW<FastCaCtlSpec> {
        SetAddrW::new(self, 24)
    }
    #[doc = "Bit 30 - See SLOW_CA_CTL.PREF_EN."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<FastCaCtlSpec> {
        PrefEnW::new(self, 30)
    }
    #[doc = "Bit 31 - See SLOW_CA_CTL.ENABLED."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<FastCaCtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Fast cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_ca_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_ca_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FastCaCtlSpec;
impl crate::RegisterSpec for FastCaCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fast_ca_ctl::R`](R) reader structure"]
impl crate::Readable for FastCaCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`fast_ca_ctl::W`](W) writer structure"]
impl crate::Writable for FastCaCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FAST_CA_CTL to value 0xc000_0000"]
impl crate::Resettable for FastCaCtlSpec {
    const RESET_VALUE: u32 = 0xc000_0000;
}
