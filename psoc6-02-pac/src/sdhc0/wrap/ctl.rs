#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `ENABLE` reader - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - IP Enable: 0: IP disabled, RAM in DeepSleep, SDHC_CORE regs are inaccessible (any attempts to access will result in AHB Error responses), IP is NOT held in reset but the clocks are gated 1: IP enabled, normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtlSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Top level wrapper control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
