#[doc = "Register `FM_SRAM_ECC_CTL3` reader"]
pub type R = crate::R<FmSramEccCtl3Spec>;
#[doc = "Register `FM_SRAM_ECC_CTL3` writer"]
pub type W = crate::W<FmSramEccCtl3Spec>;
#[doc = "Field `ECC_ENABLE` reader - ECC generation/check enable for eCT Flash SRAM memory."]
pub type EccEnableR = crate::BitReader;
#[doc = "Field `ECC_ENABLE` writer - ECC generation/check enable for eCT Flash SRAM memory."]
pub type EccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_INJ_EN` reader - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
pub type EccInjEnR = crate::BitReader;
#[doc = "Field `ECC_INJ_EN` writer - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
pub type EccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_TEST_FAIL` reader - Status of ECC test. 1 : ECC test failed because eCT Flash macro is busy and using the SRAM. 0: ECC was performed."]
pub type EccTestFailR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    pub fn ecc_enable(&self) -> EccEnableR {
        EccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    pub fn ecc_inj_en(&self) -> EccInjEnR {
        EccInjEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Status of ECC test. 1 : ECC test failed because eCT Flash macro is busy and using the SRAM. 0: ECC was performed."]
    #[inline(always)]
    pub fn ecc_test_fail(&self) -> EccTestFailR {
        EccTestFailR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC generation/check enable for eCT Flash SRAM memory."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_enable(&mut self) -> EccEnableW<FmSramEccCtl3Spec> {
        EccEnableW::new(self, 0)
    }
    #[doc = "Bit 4 - eCT Flash SRAM ECC error injection test enable. Follow the steps below for ECC logic test: 1. Write corrupted or uncorrupted 39-bit data to FM_SRAM_ECC_CTL0/1 registers. 2. Set the ECC_INJ_EN bit to '1'. 3. Confirm that the bit ECC_TEST_FAIL is '0'. If this is not the case, start over at item 1 because the eCT Flash was not idle. 4. Check the corrected data in FM_SRAM_ECC_CTL2. 5. Confirm that fault was reported to fault structure, and check syndrome (only applicable if corrupted data was written in step 1). 6. If not finished, start over at 1 with different data."]
    #[inline(always)]
    #[must_use]
    pub fn ecc_inj_en(&mut self) -> EccInjEnW<FmSramEccCtl3Spec> {
        EccInjEnW::new(self, 4)
    }
}
#[doc = "eCT Flash SRAM ECC control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_sram_ecc_ctl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_sram_ecc_ctl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmSramEccCtl3Spec;
impl crate::RegisterSpec for FmSramEccCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_sram_ecc_ctl3::R`](R) reader structure"]
impl crate::Readable for FmSramEccCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`fm_sram_ecc_ctl3::W`](W) writer structure"]
impl crate::Writable for FmSramEccCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_SRAM_ECC_CTL3 to value 0x01"]
impl crate::Resettable for FmSramEccCtl3Spec {
    const RESET_VALUE: u32 = 0x01;
}
