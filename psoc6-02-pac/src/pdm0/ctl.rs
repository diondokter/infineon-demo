#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `PGA_R` reader - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PgaRR = crate::FieldReader;
#[doc = "Field `PGA_R` writer - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
pub type PgaRW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PGA_L` reader - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PgaLR = crate::FieldReader;
#[doc = "Field `PGA_L` writer - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
pub type PgaLW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SOFT_MUTE` reader - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SoftMuteR = crate::BitReader;
#[doc = "Field `SOFT_MUTE` writer - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
pub type SoftMuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STEP_SEL` reader - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type StepSelR = crate::BitReader;
#[doc = "Field `STEP_SEL` writer - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
pub type StepSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enables the PDM component: '0': Disabled. '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    pub fn pga_r(&self) -> PgaRR {
        PgaRR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    pub fn pga_l(&self) -> PgaLR {
        PgaLR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    pub fn soft_mute(&self) -> SoftMuteR {
        SoftMuteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    pub fn step_sel(&self) -> StepSelR {
        StepSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Right channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15' +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_R)"]
    #[inline(always)]
    #[must_use]
    pub fn pga_r(&mut self) -> PgaRW<CtlSpec> {
        PgaRW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Left channel PGA gain: +1.5dB/step, -12dB ~ +10.5dB '0': -12 dB '1': -10.5 dB ... '15': +10.5 dB (Note: These bits are connected to AR36U12.PDM_CORE_CFG.PGA_L)"]
    #[inline(always)]
    #[must_use]
    pub fn pga_l(&mut self) -> PgaLW<CtlSpec> {
        PgaLW::new(self, 8)
    }
    #[doc = "Bit 16 - Soft mute function to mute the volume smoothly '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.SOFT_MUTE)"]
    #[inline(always)]
    #[must_use]
    pub fn soft_mute(&mut self) -> SoftMuteW<CtlSpec> {
        SoftMuteW::new(self, 16)
    }
    #[doc = "Bit 17 - Set fine gain step for smooth PGA or Soft-Mute attenuation transition. '0': 0.13dB '1': 0.26dB (Note: This bit is connected to AR36U12.PDM_CORE2_CFG.SEL_STEP)"]
    #[inline(always)]
    #[must_use]
    pub fn step_sel(&mut self) -> StepSelW<CtlSpec> {
        StepSelW::new(self, 17)
    }
    #[doc = "Bit 31 - Enables the PDM component: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0x0002_0808"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0002_0808;
}
