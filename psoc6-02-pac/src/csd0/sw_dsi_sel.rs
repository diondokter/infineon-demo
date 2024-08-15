#[doc = "Register `SW_DSI_SEL` reader"]
pub type R = crate::R<SwDsiSelSpec>;
#[doc = "Register `SW_DSI_SEL` writer"]
pub type W = crate::W<SwDsiSelSpec>;
#[doc = "Field `DSI_CSH_TANK` reader - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 &amp; HSCMP 5: phi2 &amp; HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 &amp; HSCMP) 13: !(phi2 &amp; HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
pub type DsiCshTankR = crate::FieldReader;
#[doc = "Field `DSI_CSH_TANK` writer - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 &amp; HSCMP 5: phi2 &amp; HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 &amp; HSCMP) 13: !(phi2 &amp; HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
pub type DsiCshTankW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DSI_CMOD` reader - Select waveform for dsi_cmod output signal"]
pub type DsiCmodR = crate::FieldReader;
#[doc = "Field `DSI_CMOD` writer - Select waveform for dsi_cmod output signal"]
pub type DsiCmodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 &amp; HSCMP 5: phi2 &amp; HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 &amp; HSCMP) 13: !(phi2 &amp; HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    pub fn dsi_csh_tank(&self) -> DsiCshTankR {
        DsiCshTankR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    pub fn dsi_cmod(&self) -> DsiCmodR {
        DsiCmodR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select waveform for dsi_csh_tank output signal 0: static open 1: static closed 2: phi1 3: phi2 4: phi1 &amp; HSCMP 5: phi2 &amp; HSCMP 6: HSCMP // ignores phi1/2 7: !sense // = phi1 but ignores OVERLAP_PHI1 8: phi1_delay // phi1 delayed with shield delay 9: phi2_delay // phi2 delayed with shield delay 10: !phi1 11: !phi2 12: !(phi1 &amp; HSCMP) 13: !(phi2 &amp; HSCMP) 14: !HSCMP // ignores phi1/2 15: sense // = phi2 but ignores OVERLAP_PHI2"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_csh_tank(&mut self) -> DsiCshTankW<SwDsiSelSpec> {
        DsiCshTankW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Select waveform for dsi_cmod output signal"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_cmod(&mut self) -> DsiCmodW<SwDsiSelSpec> {
        DsiCmodW::new(self, 4)
    }
}
#[doc = "DSI output switch control Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_dsi_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_dsi_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwDsiSelSpec;
impl crate::RegisterSpec for SwDsiSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_dsi_sel::R`](R) reader structure"]
impl crate::Readable for SwDsiSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_dsi_sel::W`](W) writer structure"]
impl crate::Writable for SwDsiSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_DSI_SEL to value 0"]
impl crate::Resettable for SwDsiSelSpec {
    const RESET_VALUE: u32 = 0;
}
