#[doc = "Register `PWR_LVD_STATUS` reader"]
pub type R = crate::R<PwrLvdStatusSpec>;
#[doc = "Field `HVLVD1_OK` reader - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
pub type Hvlvd1OkR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - HVLVD1 output. 0: below voltage threshold 1: above voltage threshold"]
    #[inline(always)]
    pub fn hvlvd1_ok(&self) -> Hvlvd1OkR {
        Hvlvd1OkR::new((self.bits & 1) != 0)
    }
}
#[doc = "Low Voltage Detector (LVD) Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_lvd_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrLvdStatusSpec;
impl crate::RegisterSpec for PwrLvdStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_lvd_status::R`](R) reader structure"]
impl crate::Readable for PwrLvdStatusSpec {}
#[doc = "`reset()` method sets PWR_LVD_STATUS to value 0"]
impl crate::Resettable for PwrLvdStatusSpec {
    const RESET_VALUE: u32 = 0;
}
