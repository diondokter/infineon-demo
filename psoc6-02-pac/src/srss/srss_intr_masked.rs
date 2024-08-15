#[doc = "Register `SRSS_INTR_MASKED` reader"]
pub type R = crate::R<SrssIntrMaskedSpec>;
#[doc = "Field `WDT_MATCH` reader - Logical and of corresponding request and mask bits."]
pub type WdtMatchR = crate::BitReader;
#[doc = "Field `HVLVD1` reader - Logical and of corresponding request and mask bits."]
pub type Hvlvd1R = crate::BitReader;
#[doc = "Field `CLK_CAL` reader - Logical and of corresponding request and mask bits."]
pub type ClkCalR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn wdt_match(&self) -> WdtMatchR {
        WdtMatchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn hvlvd1(&self) -> Hvlvd1R {
        Hvlvd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Logical and of corresponding request and mask bits."]
    #[inline(always)]
    pub fn clk_cal(&self) -> ClkCalR {
        ClkCalR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "SRSS Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srss_intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrssIntrMaskedSpec;
impl crate::RegisterSpec for SrssIntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srss_intr_masked::R`](R) reader structure"]
impl crate::Readable for SrssIntrMaskedSpec {}
#[doc = "`reset()` method sets SRSS_INTR_MASKED to value 0"]
impl crate::Resettable for SrssIntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
