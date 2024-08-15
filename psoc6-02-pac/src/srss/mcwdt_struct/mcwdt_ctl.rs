#[doc = "Register `MCWDT_CTL` reader"]
pub type R = crate::R<McwdtCtlSpec>;
#[doc = "Register `MCWDT_CTL` writer"]
pub type W = crate::W<McwdtCtlSpec>;
#[doc = "Field `WDT_ENABLE0` reader - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable0R = crate::BitReader;
#[doc = "Field `WDT_ENABLE0` writer - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED0` reader - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
pub type WdtEnabled0R = crate::BitReader;
#[doc = "Field `WDT_RESET0` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset0R = crate::BitReader;
#[doc = "Field `WDT_RESET0` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLE1` reader - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable1R = crate::BitReader;
#[doc = "Field `WDT_ENABLE1` writer - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED1` reader - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
pub type WdtEnabled1R = crate::BitReader;
#[doc = "Field `WDT_RESET1` reader - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset1R = crate::BitReader;
#[doc = "Field `WDT_RESET1` writer - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLE2` reader - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable2R = crate::BitReader;
#[doc = "Field `WDT_ENABLE2` writer - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type WdtEnable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_ENABLED2` reader - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
pub type WdtEnabled2R = crate::BitReader;
#[doc = "Field `WDT_RESET2` reader - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset2R = crate::BitReader;
#[doc = "Field `WDT_RESET2` writer - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub type WdtReset2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&self) -> WdtEnable0R {
        WdtEnable0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled0(&self) -> WdtEnabled0R {
        WdtEnabled0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&self) -> WdtReset0R {
        WdtReset0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&self) -> WdtEnable1R {
        WdtEnable1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled1(&self) -> WdtEnabled1R {
        WdtEnabled1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&self) -> WdtReset1R {
        WdtReset1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&self) -> WdtEnable2R {
        WdtEnable2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled2(&self) -> WdtEnabled2R {
        WdtEnabled2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&self) -> WdtReset2R {
        WdtReset2R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable0(&mut self) -> WdtEnable0W<McwdtCtlSpec> {
        WdtEnable0W::new(self, 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset0(&mut self) -> WdtReset0W<McwdtCtlSpec> {
        WdtReset0W::new(self, 3)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable1(&mut self) -> WdtEnable1W<McwdtCtlSpec> {
        WdtEnable1W::new(self, 8)
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset1(&mut self) -> WdtReset1W<McwdtCtlSpec> {
        WdtReset1W::new(self, 11)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_enable2(&mut self) -> WdtEnable2W<McwdtCtlSpec> {
        WdtEnable2W::new(self, 16)
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_reset2(&mut self) -> WdtReset2W<McwdtCtlSpec> {
        WdtReset2W::new(self, 19)
    }
}
#[doc = "Multi-Counter Watchdog Counter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtCtlSpec;
impl crate::RegisterSpec for McwdtCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_ctl::R`](R) reader structure"]
impl crate::Readable for McwdtCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_ctl::W`](W) writer structure"]
impl crate::Writable for McwdtCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_CTL to value 0"]
impl crate::Resettable for McwdtCtlSpec {
    const RESET_VALUE: u32 = 0;
}
