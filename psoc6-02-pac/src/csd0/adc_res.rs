#[doc = "Register `ADC_RES` reader"]
pub type R = crate::R<AdcResSpec>;
#[doc = "Field `VIN_CNT` reader - Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
pub type VinCntR = crate::FieldReader<u16>;
#[doc = "Field `HSCMP_POL` reader - Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
pub type HscmpPolR = crate::BitReader;
#[doc = "Field `ADC_OVERFLOW` reader - This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
pub type AdcOverflowR = crate::BitReader;
#[doc = "Field `ADC_ABORT` reader - This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
pub type AdcAbortR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Count to source/sink Cref1 + Cref2 from Vin to Vrefhi."]
    #[inline(always)]
    pub fn vin_cnt(&self) -> VinCntR {
        VinCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Polarity used for IDACB for this last ADC result, 0= source, 1= sink"]
    #[inline(always)]
    pub fn hscmp_pol(&self) -> HscmpPolR {
        HscmpPolR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 30 - This flag is set when the ADC counter overflows. This is an indication to the firmware that the IDACB current level is too low."]
    #[inline(always)]
    pub fn adc_overflow(&self) -> AdcOverflowR {
        AdcOverflowR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This flag is set when the ADC sequencer was aborted before tripping HSCMP."]
    #[inline(always)]
    pub fn adc_abort(&self) -> AdcAbortR {
        AdcAbortR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "ADC measurement\n\nYou can [`read`](crate::Reg::read) this register and get [`adc_res::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcResSpec;
impl crate::RegisterSpec for AdcResSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_res::R`](R) reader structure"]
impl crate::Readable for AdcResSpec {}
#[doc = "`reset()` method sets ADC_RES to value 0"]
impl crate::Resettable for AdcResSpec {
    const RESET_VALUE: u32 = 0;
}
