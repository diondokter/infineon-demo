#[doc = "Register `FM_SRAM_ECC_CTL1` reader"]
pub type R = crate::R<FmSramEccCtl1Spec>;
#[doc = "Register `FM_SRAM_ECC_CTL1` writer"]
pub type W = crate::W<FmSramEccCtl1Spec>;
#[doc = "Field `ECC_INJ_PARITY` reader - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type EccInjParityR = crate::FieldReader;
#[doc = "Field `ECC_INJ_PARITY` writer - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
pub type EccInjParityW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    pub fn ecc_inj_parity(&self) -> EccInjParityR {
        EccInjParityR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit parity for ECC error injection test of eCT Flash SRAM ECC logic."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_parity(&mut self) -> EccInjParityW<FmSramEccCtl1Spec> {
        EccInjParityW::new(self, 0)
    }
}
#[doc = "eCT Flash SRAM ECC control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmSramEccCtl1Spec;
impl crate::RegisterSpec for FmSramEccCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl1::R`](R) reader structure"]
impl crate::Readable for FmSramEccCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fm_sram_ecc_ctl1::W`](W) writer structure"]
impl crate::Writable for FmSramEccCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL1 to value 0"]
impl crate::Resettable for FmSramEccCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
