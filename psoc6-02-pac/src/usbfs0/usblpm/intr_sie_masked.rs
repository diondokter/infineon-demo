#[doc = "Register `INTR_SIE_MASKED` reader"]
pub type R = crate::R<IntrSieMaskedSpec>;
#[doc = "Field `SOF_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type SofIntrMaskedR = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type BusResetIntrMaskedR = crate::BitReader;
#[doc = "Field `EP0_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type Ep0IntrMaskedR = crate::BitReader;
#[doc = "Field `LPM_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type LpmIntrMaskedR = crate::BitReader;
#[doc = "Field `RESUME_INTR_MASKED` reader - Logical and of corresponding request and mask bits."]
pub type ResumeIntrMaskedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn sof_intr_masked(&self) -> SofIntrMaskedR {
        SofIntrMaskedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn bus_reset_intr_masked(&self) -> BusResetIntrMaskedR {
        BusResetIntrMaskedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn ep0_intr_masked(&self) -> Ep0IntrMaskedR {
        Ep0IntrMaskedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn lpm_intr_masked(&self) -> LpmIntrMaskedR {
        LpmIntrMaskedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn resume_intr_masked(&self) -> ResumeIntrMaskedR {
        ResumeIntrMaskedR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "USB SOF, BUS RESET and EP0 Interrupt Masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_sie_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSieMaskedSpec;
impl crate::RegisterSpec for IntrSieMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_sie_masked::R`](R) reader structure"]
impl crate::Readable for IntrSieMaskedSpec {}
#[doc = "`reset()` method sets INTR_SIE_MASKED to value 0"]
impl crate::Resettable for IntrSieMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
