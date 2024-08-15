#[doc = "Register `FM_SRAM_ECC_CTL0` reader"]
pub type R = crate::R<FmSramEccCtl0Spec>;
#[doc = "Register `FM_SRAM_ECC_CTL0` writer"]
pub type W = crate::W<FmSramEccCtl0Spec>;
#[doc = "Field `ECC_INJ_DATA` reader - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type EccInjDataR = crate::FieldReader<u32>;
#[doc = "Field `ECC_INJ_DATA` writer - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type EccInjDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_data(&self) -> EccInjDataR {
        EccInjDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit data for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_data(&mut self) -> EccInjDataW<FmSramEccCtl0Spec> {
        EccInjDataW::new(self, 0)
    }
}
#[doc = "eCT Flash SRAM ECC control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmSramEccCtl0Spec;
impl crate::RegisterSpec for FmSramEccCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl0::R`](R) reader structure"]
impl crate::Readable for FmSramEccCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`fm_sram_ecc_ctl0::W`](W) writer structure"]
impl crate::Writable for FmSramEccCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL0 to value 0"]
impl crate::Resettable for FmSramEccCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
