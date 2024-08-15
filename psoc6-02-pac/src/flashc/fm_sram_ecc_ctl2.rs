#[doc = "Register `FM_SRAM_ECC_CTL2` reader"]
pub type R = crate::R<FmSramEccCtl2Spec>;
#[doc = "Field `CORRECTED_DATA` reader - 32-bit corrected data output of the ECC syndrome logic."]
pub type CorrectedDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit corrected data output of the ECC syndrome logic."]
    #[inline(always)]
    pub fn corrected_data(&self) -> CorrectedDataR {
        CorrectedDataR::new(self.bits)
    }
}
#[doc = "eCT Flash SRAM ECC control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmSramEccCtl2Spec;
impl crate::RegisterSpec for FmSramEccCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl2::R`](R) reader structure"]
impl crate::Readable for FmSramEccCtl2Spec {}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL2 to value 0"]
impl crate::Resettable for FmSramEccCtl2Spec {
    const RESET_VALUE: u32 = 0;
}
