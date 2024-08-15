#[doc = "Register `ANA_CTL1` reader"]
pub type R = crate::R<AnaCtl1Spec>;
#[doc = "Register `ANA_CTL1` writer"]
pub type W = crate::W<AnaCtl1Spec>;
#[doc = "Field `NDAC_MAX` reader - Ndac Max Value.Trimming of negative pump output Voltage."]
pub type NdacMaxR = crate::FieldReader;
#[doc = "Field `NDAC_MAX` writer - Ndac Max Value.Trimming of negative pump output Voltage."]
pub type NdacMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NDAC_STEP` reader - Ndac step increment"]
pub type NdacStepR = crate::FieldReader;
#[doc = "Field `NDAC_STEP` writer - Ndac step increment"]
pub type NdacStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_MAX` reader - Pdac Max Value.Trimming of positive pump output Voltage:"]
pub type PdacMaxR = crate::FieldReader;
#[doc = "Field `PDAC_MAX` writer - Pdac Max Value.Trimming of positive pump output Voltage:"]
pub type PdacMaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_STEP` reader - Pdac step increment"]
pub type PdacStepR = crate::FieldReader;
#[doc = "Field `PDAC_STEP` writer - Pdac step increment"]
pub type PdacStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NPDAC_STEP_TIME` reader - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
pub type NpdacStepTimeR = crate::FieldReader;
#[doc = "Field `NPDAC_STEP_TIME` writer - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
pub type NpdacStepTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NPDAC_ZERO_TIME` reader - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
pub type NpdacZeroTimeR = crate::FieldReader;
#[doc = "Field `NPDAC_ZERO_TIME` writer - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
pub type NpdacZeroTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    pub fn ndac_max(&self) -> NdacMaxR {
        NdacMaxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    pub fn ndac_step(&self) -> NdacStepR {
        NdacStepR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    pub fn pdac_max(&self) -> PdacMaxR {
        PdacMaxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    pub fn pdac_step(&self) -> PdacStepR {
        PdacStepR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    pub fn npdac_step_time(&self) -> NpdacStepTimeR {
        NpdacStepTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    pub fn npdac_zero_time(&self) -> NpdacZeroTimeR {
        NpdacZeroTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Ndac Max Value.Trimming of negative pump output Voltage."]
    #[inline(always)]
    #[must_use]
    pub fn ndac_max(&mut self) -> NdacMaxW<AnaCtl1Spec> {
        NdacMaxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Ndac step increment"]
    #[inline(always)]
    #[must_use]
    pub fn ndac_step(&mut self) -> NdacStepW<AnaCtl1Spec> {
        NdacStepW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Pdac Max Value.Trimming of positive pump output Voltage:"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_max(&mut self) -> PdacMaxW<AnaCtl1Spec> {
        PdacMaxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Pdac step increment"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_step(&mut self) -> PdacStepW<AnaCtl1Spec> {
        PdacStepW::new(self, 12)
    }
    #[doc = "Bits 16:23 - Ndac/Pdac step duration: (1uS .. 255uS) * 8 When = 0 N/PDAC_MAX control the pumps"]
    #[inline(always)]
    #[must_use]
    pub fn npdac_step_time(&mut self) -> NpdacStepTimeW<AnaCtl1Spec> {
        NpdacStepTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Ndac/Pdac LO duration: (1uS .. 255uS) * 8 When 0, N/PDAC don't return to 0"]
    #[inline(always)]
    #[must_use]
    pub fn npdac_zero_time(&mut self) -> NpdacZeroTimeW<AnaCtl1Spec> {
        NpdacZeroTimeW::new(self, 24)
    }
}
#[doc = "Analog control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaCtl1Spec;
impl crate::RegisterSpec for AnaCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_ctl1::R`](R) reader structure"]
impl crate::Readable for AnaCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_ctl1::W`](W) writer structure"]
impl crate::Writable for AnaCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_CTL1 to value 0x0d32_fafa"]
impl crate::Resettable for AnaCtl1Spec {
    const RESET_VALUE: u32 = 0x0d32_fafa;
}
