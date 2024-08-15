#[doc = "Register `CLK_PILO_CONFIG` reader"]
pub type R = crate::R<ClkPiloConfigSpec>;
#[doc = "Register `CLK_PILO_CONFIG` writer"]
pub type W = crate::W<ClkPiloConfigSpec>;
#[doc = "Field `PILO_FFREQ` reader - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PiloFfreqR = crate::FieldReader<u16>;
#[doc = "Field `PILO_FFREQ` writer - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
pub type PiloFfreqW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PILO_CLK_EN` reader - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PiloClkEnR = crate::BitReader;
#[doc = "Field `PILO_CLK_EN` writer - Enable the PILO clock output. See PILO_EN field for required sequencing."]
pub type PiloClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PILO_RESET_N` reader - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PiloResetNR = crate::BitReader;
#[doc = "Field `PILO_RESET_N` writer - Reset the PILO. See PILO_EN field for required sequencing."]
pub type PiloResetNW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PILO_EN` reader - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PiloEnR = crate::BitReader;
#[doc = "Field `PILO_EN` writer - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
pub type PiloEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    pub fn pilo_ffreq(&self) -> PiloFfreqR {
        PiloFfreqR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_clk_en(&self) -> PiloClkEnR {
        PiloClkEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    pub fn pilo_reset_n(&self) -> PiloResetNR {
        PiloResetNR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    pub fn pilo_en(&self) -> PiloEnR {
        PiloEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine frequency trim allowing +/-250ppm accuracy with periodic calibration. The nominal step size of the LSB is 8Hz."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_ffreq(&mut self) -> PiloFfreqW<ClkPiloConfigSpec> {
        PiloFfreqW::new(self, 0)
    }
    #[doc = "Bit 29 - Enable the PILO clock output. See PILO_EN field for required sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_clk_en(&mut self) -> PiloClkEnW<ClkPiloConfigSpec> {
        PiloClkEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset the PILO. See PILO_EN field for required sequencing."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_reset_n(&mut self) -> PiloResetNW<ClkPiloConfigSpec> {
        PiloResetNW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable PILO. When enabling PILO, set PILO_EN=1, wait 1ms, then PILO_RESET_N=1 and PILO_CLK_EN=1. When disabling PILO, clear PILO_EN=0, PILO_RESET_N=0, and PLO_CLK_EN=0 in the same write cycle."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_en(&mut self) -> PiloEnW<ClkPiloConfigSpec> {
        PiloEnW::new(self, 31)
    }
}
#[doc = "Precision ILO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pilo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pilo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkPiloConfigSpec;
impl crate::RegisterSpec for ClkPiloConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pilo_config::R`](R) reader structure"]
impl crate::Readable for ClkPiloConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_pilo_config::W`](W) writer structure"]
impl crate::Writable for ClkPiloConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_PILO_CONFIG to value 0x80"]
impl crate::Resettable for ClkPiloConfigSpec {
    const RESET_VALUE: u32 = 0x80;
}
