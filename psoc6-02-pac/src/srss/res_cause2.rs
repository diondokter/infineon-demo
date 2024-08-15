#[doc = "Register `RES_CAUSE2` reader"]
pub type R = crate::R<ResCause2Spec>;
#[doc = "Register `RES_CAUSE2` writer"]
pub type W = crate::W<ResCause2Spec>;
#[doc = "Field `RESET_CSV_HF_LOSS` reader - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type ResetCsvHfLossR = crate::FieldReader<u16>;
#[doc = "Field `RESET_CSV_HF_LOSS` writer - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type ResetCsvHfLossW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESET_CSV_HF_FREQ` reader - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type ResetCsvHfFreqR = crate::FieldReader<u16>;
#[doc = "Field `RESET_CSV_HF_FREQ` writer - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
pub type ResetCsvHfFreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_loss(&self) -> ResetCsvHfLossR {
        ResetCsvHfLossR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    pub fn reset_csv_hf_freq(&self) -> ResetCsvHfFreqR {
        ResetCsvHfFreqR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock supervision logic requested a reset due to loss of a high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_hf_loss(&mut self) -> ResetCsvHfLossW<ResCause2Spec> {
        ResetCsvHfLossW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Clock supervision logic requested a reset due to frequency error of high-frequency clock. Each bit index K corresponds to a HFCLK&lt;K>. Unimplemented clock bits return zero."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_hf_freq(&mut self) -> ResetCsvHfFreqW<ResCause2Spec> {
        ResetCsvHfFreqW::new(self, 16)
    }
}
#[doc = "Reset Cause Observation Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`res_cause2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res_cause2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResCause2Spec;
impl crate::RegisterSpec for ResCause2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res_cause2::R`](R) reader structure"]
impl crate::Readable for ResCause2Spec {}
#[doc = "`write(|w| ..)` method takes [`res_cause2::W`](W) writer structure"]
impl crate::Writable for ResCause2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES_CAUSE2 to value 0"]
impl crate::Resettable for ResCause2Spec {
    const RESET_VALUE: u32 = 0;
}
