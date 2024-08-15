#[doc = "Register `CLK_IMO_CONFIG` reader"]
pub type R = crate::R<ClkImoConfigSpec>;
#[doc = "Register `CLK_IMO_CONFIG` writer"]
pub type W = crate::W<ClkImoConfigSpec>;
#[doc = "Field `ENABLE` reader - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Master enable for IMO oscillator. This bit must be high at all times for all functions to work properly. Hardware will automatically disable the IMO during HIBERNATE and XRES. It will automatically disable during DEEPSLEEP if DPSLP_ENABLE==0."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkImoConfigSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "IMO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_imo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_imo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkImoConfigSpec;
impl crate::RegisterSpec for ClkImoConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_imo_config::R`](R) reader structure"]
impl crate::Readable for ClkImoConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_imo_config::W`](W) writer structure"]
impl crate::Writable for ClkImoConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_IMO_CONFIG to value 0x8000_0000"]
impl crate::Resettable for ClkImoConfigSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
