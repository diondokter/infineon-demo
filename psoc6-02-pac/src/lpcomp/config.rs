#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `LPREF_EN` reader - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
pub type LprefEnR = crate::BitReader;
#[doc = "Field `LPREF_EN` writer - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
pub type LprefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    pub fn lpref_en(&self) -> LprefEnR {
        LprefEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Enable the local reference generator circuit to generate the local Vref and ibias. This bit must be set for DeepSleep or Hibernate operation."]
    #[inline(always)]
    #[must_use]
    pub fn lpref_en(&mut self) -> LprefEnW<ConfigSpec> {
        LprefEnW::new(self, 30)
    }
    #[doc = "Bit 31 - - 0: IP disabled (put analog in power down, open all switches, all clocks off, leakage power only) - 1: IP enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<ConfigSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "LPCOMP Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
