#[doc = "Register `ADC_CTL` reader"]
pub type R = crate::R<AdcCtlSpec>;
#[doc = "Register `ADC_CTL` writer"]
pub type W = crate::W<AdcCtlSpec>;
#[doc = "Field `ADC_TIME` reader - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
pub type AdcTimeR = crate::FieldReader;
#[doc = "Field `ADC_TIME` writer - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
pub type AdcTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdcMode {
    #[doc = "0: No ADC measurement"]
    Off = 0,
    #[doc = "1: Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    VrefCnt = 1,
    #[doc = "2: Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    VrefBy2Cnt = 2,
    #[doc = "3: Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    VinCnt = 3,
}
impl From<AdcMode> for u8 {
    #[inline(always)]
    fn from(variant: AdcMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AdcMode {
    type Ux = u8;
}
impl crate::IsEnum for AdcMode {}
#[doc = "Field `ADC_MODE` reader - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub type AdcModeR = crate::FieldReader<AdcMode>;
impl AdcModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AdcMode {
        match self.bits {
            0 => AdcMode::Off,
            1 => AdcMode::VrefCnt,
            2 => AdcMode::VrefBy2Cnt,
            3 => AdcMode::VinCnt,
            _ => unreachable!(),
        }
    }
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AdcMode::Off
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn is_vref_cnt(&self) -> bool {
        *self == AdcMode::VrefCnt
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn is_vref_by2_cnt(&self) -> bool {
        *self == AdcMode::VrefBy2Cnt
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn is_vin_cnt(&self) -> bool {
        *self == AdcMode::VinCnt
    }
}
#[doc = "Field `ADC_MODE` writer - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
pub type AdcModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, AdcMode, crate::Safe>;
impl<'a, REG> AdcModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No ADC measurement"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(AdcMode::Off)
    }
    #[doc = "Count time A to bring Cref1 + Cref2 up from Vssa to Vrefhi with IDACB"]
    #[inline(always)]
    pub fn vref_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(AdcMode::VrefCnt)
    }
    #[doc = "Count time B to bring Cref1 + Cref2 back up to Vrefhi with IDACB (after bringing them down for time A/2 cycles with IDACB sinking)"]
    #[inline(always)]
    pub fn vref_by2_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(AdcMode::VrefBy2Cnt)
    }
    #[doc = "Determine HSCMP polarity and count time C to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(self) -> &'a mut crate::W<REG> {
        self.variant(AdcMode::VinCnt)
    }
}
impl R {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
    #[inline(always)]
    pub fn adc_time(&self) -> AdcTimeR {
        AdcTimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    pub fn adc_mode(&self) -> AdcModeR {
        AdcModeR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADC timing -1 in csd_sense clock cycles (actual time is ADC_TIME+1 cycles), either used to discharge Cref1&amp;2, or as the aperture to capture the input voltage on Cref1&amp;2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_time(&mut self) -> AdcTimeW<AdcCtlSpec> {
        AdcTimeW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Enable ADC measurement. When enabled the ADC sequencer will be started when the main sequencer goes to the SAMPLE_NORM state"]
    #[inline(always)]
    #[must_use]
    pub fn adc_mode(&mut self) -> AdcModeW<AdcCtlSpec> {
        AdcModeW::new(self, 16)
    }
}
#[doc = "ADC Control\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcCtlSpec;
impl crate::RegisterSpec for AdcCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ctl::R`](R) reader structure"]
impl crate::Readable for AdcCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`adc_ctl::W`](W) writer structure"]
impl crate::Writable for AdcCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CTL to value 0"]
impl crate::Resettable for AdcCtlSpec {
    const RESET_VALUE: u32 = 0;
}
