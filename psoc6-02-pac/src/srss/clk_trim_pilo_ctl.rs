#[doc = "Register `CLK_TRIM_PILO_CTL` reader"]
pub type R = crate::R<ClkTrimPiloCtlSpec>;
#[doc = "Register `CLK_TRIM_PILO_CTL` writer"]
pub type W = crate::W<ClkTrimPiloCtlSpec>;
#[doc = "Field `PILO_CFREQ` reader - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PiloCfreqR = crate::FieldReader;
#[doc = "Field `PILO_CFREQ` writer - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
pub type PiloCfreqW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PILO_OSC_TRIM` reader - Trim for current in oscillator block."]
pub type PiloOscTrimR = crate::FieldReader;
#[doc = "Field `PILO_OSC_TRIM` writer - Trim for current in oscillator block."]
pub type PiloOscTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PILO_COMP_TRIM` reader - Trim for comparator bias current."]
pub type PiloCompTrimR = crate::FieldReader;
#[doc = "Field `PILO_COMP_TRIM` writer - Trim for comparator bias current."]
pub type PiloCompTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_NBIAS_TRIM` reader - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PiloNbiasTrimR = crate::FieldReader;
#[doc = "Field `PILO_NBIAS_TRIM` writer - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
pub type PiloNbiasTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_RES_TRIM` reader - Trim for beta-multiplier branch current"]
pub type PiloResTrimR = crate::FieldReader;
#[doc = "Field `PILO_RES_TRIM` writer - Trim for beta-multiplier branch current"]
pub type PiloResTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PILO_ISLOPE_TRIM` reader - Trim for beta-multiplier current slope"]
pub type PiloIslopeTrimR = crate::FieldReader;
#[doc = "Field `PILO_ISLOPE_TRIM` writer - Trim for beta-multiplier current slope"]
pub type PiloIslopeTrimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PILO_VTDIFF_TRIM` reader - Trim for VT-DIFF output (internal power supply)"]
pub type PiloVtdiffTrimR = crate::FieldReader;
#[doc = "Field `PILO_VTDIFF_TRIM` writer - Trim for VT-DIFF output (internal power supply)"]
pub type PiloVtdiffTrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    pub fn pilo_cfreq(&self) -> PiloCfreqR {
        PiloCfreqR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    pub fn pilo_osc_trim(&self) -> PiloOscTrimR {
        PiloOscTrimR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    pub fn pilo_comp_trim(&self) -> PiloCompTrimR {
        PiloCompTrimR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    pub fn pilo_nbias_trim(&self) -> PiloNbiasTrimR {
        PiloNbiasTrimR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    pub fn pilo_res_trim(&self) -> PiloResTrimR {
        PiloResTrimR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    pub fn pilo_islope_trim(&self) -> PiloIslopeTrimR {
        PiloIslopeTrimR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    pub fn pilo_vtdiff_trim(&self) -> PiloVtdiffTrimR {
        PiloVtdiffTrimR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Coarse frequency trim to meet 32.768kHz +/-2 percent across PVT without calibration. The nominal step size of the LSB is 1kHz."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_cfreq(&mut self) -> PiloCfreqW<ClkTrimPiloCtlSpec> {
        PiloCfreqW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Trim for current in oscillator block."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_osc_trim(&mut self) -> PiloOscTrimW<ClkTrimPiloCtlSpec> {
        PiloOscTrimW::new(self, 12)
    }
    #[doc = "Bits 16:17 - Trim for comparator bias current."]
    #[inline(always)]
    #[must_use]
    pub fn pilo_comp_trim(&mut self) -> PiloCompTrimW<ClkTrimPiloCtlSpec> {
        PiloCompTrimW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Trim for biasn by trimming sub-Vth NMOS width in beta-multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_nbias_trim(&mut self) -> PiloNbiasTrimW<ClkTrimPiloCtlSpec> {
        PiloNbiasTrimW::new(self, 18)
    }
    #[doc = "Bits 20:24 - Trim for beta-multiplier branch current"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_res_trim(&mut self) -> PiloResTrimW<ClkTrimPiloCtlSpec> {
        PiloResTrimW::new(self, 20)
    }
    #[doc = "Bits 26:27 - Trim for beta-multiplier current slope"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_islope_trim(&mut self) -> PiloIslopeTrimW<ClkTrimPiloCtlSpec> {
        PiloIslopeTrimW::new(self, 26)
    }
    #[doc = "Bits 28:30 - Trim for VT-DIFF output (internal power supply)"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_vtdiff_trim(&mut self) -> PiloVtdiffTrimW<ClkTrimPiloCtlSpec> {
        PiloVtdiffTrimW::new(self, 28)
    }
}
#[doc = "PILO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimPiloCtlSpec;
impl crate::RegisterSpec for ClkTrimPiloCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl::R`](R) reader structure"]
impl crate::Readable for ClkTrimPiloCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl::W`](W) writer structure"]
impl crate::Writable for ClkTrimPiloCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL to value 0x0108_500f"]
impl crate::Resettable for ClkTrimPiloCtlSpec {
    const RESET_VALUE: u32 = 0x0108_500f;
}
