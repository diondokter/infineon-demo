#[doc = "Register `CLK_ECO_CONFIG` reader"]
pub type R = crate::R<ClkEcoConfigSpec>;
#[doc = "Register `CLK_ECO_CONFIG` writer"]
pub type W = crate::W<ClkEcoConfigSpec>;
#[doc = "Field `AGC_EN` reader - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AgcEnR = crate::BitReader;
#[doc = "Field `AGC_EN` writer - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AgcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECO_EN` reader - Master enable for ECO oscillator."]
pub type EcoEnR = crate::BitReader;
#[doc = "Field `ECO_EN` writer - Master enable for ECO oscillator."]
pub type EcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn agc_en(&self) -> AgcEnR {
        AgcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn eco_en(&self) -> EcoEnR {
        EcoEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by ECO_TRIM0.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    #[must_use]
    pub fn agc_en(&mut self) -> AgcEnW<ClkEcoConfigSpec> {
        AgcEnW::new(self, 1)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn eco_en(&mut self) -> EcoEnW<ClkEcoConfigSpec> {
        EcoEnW::new(self, 31)
    }
}
#[doc = "ECO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_eco_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_eco_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkEcoConfigSpec;
impl crate::RegisterSpec for ClkEcoConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_eco_config::R`](R) reader structure"]
impl crate::Readable for ClkEcoConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_eco_config::W`](W) writer structure"]
impl crate::Writable for ClkEcoConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_ECO_CONFIG to value 0x02"]
impl crate::Resettable for ClkEcoConfigSpec {
    const RESET_VALUE: u32 = 0x02;
}
