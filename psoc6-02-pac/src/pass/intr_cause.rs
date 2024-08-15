#[doc = "Register `INTR_CAUSE` reader"]
pub type R = crate::R<IntrCauseSpec>;
#[doc = "Field `CTB0_INT` reader - CTB0 interrupt pending"]
pub type Ctb0IntR = crate::BitReader;
#[doc = "Field `CTB1_INT` reader - CTB1 interrupt pending"]
pub type Ctb1IntR = crate::BitReader;
#[doc = "Field `CTB2_INT` reader - CTB2 interrupt pending"]
pub type Ctb2IntR = crate::BitReader;
#[doc = "Field `CTB3_INT` reader - CTB3 interrupt pending"]
pub type Ctb3IntR = crate::BitReader;
#[doc = "Field `CTDAC0_INT` reader - CTDAC0 interrupt pending"]
pub type Ctdac0IntR = crate::BitReader;
#[doc = "Field `CTDAC1_INT` reader - CTDAC1 interrupt pending"]
pub type Ctdac1IntR = crate::BitReader;
#[doc = "Field `CTDAC2_INT` reader - CTDAC2 interrupt pending"]
pub type Ctdac2IntR = crate::BitReader;
#[doc = "Field `CTDAC3_INT` reader - CTDAC3 interrupt pending"]
pub type Ctdac3IntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CTB0 interrupt pending"]
    #[inline(always)]
    pub fn ctb0_int(&self) -> Ctb0IntR {
        Ctb0IntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTB1 interrupt pending"]
    #[inline(always)]
    pub fn ctb1_int(&self) -> Ctb1IntR {
        Ctb1IntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTB2 interrupt pending"]
    #[inline(always)]
    pub fn ctb2_int(&self) -> Ctb2IntR {
        Ctb2IntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTB3 interrupt pending"]
    #[inline(always)]
    pub fn ctb3_int(&self) -> Ctb3IntR {
        Ctb3IntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTDAC0 interrupt pending"]
    #[inline(always)]
    pub fn ctdac0_int(&self) -> Ctdac0IntR {
        Ctdac0IntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CTDAC1 interrupt pending"]
    #[inline(always)]
    pub fn ctdac1_int(&self) -> Ctdac1IntR {
        Ctdac1IntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CTDAC2 interrupt pending"]
    #[inline(always)]
    pub fn ctdac2_int(&self) -> Ctdac2IntR {
        Ctdac2IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTDAC3 interrupt pending"]
    #[inline(always)]
    pub fn ctdac3_int(&self) -> Ctdac3IntR {
        Ctdac3IntR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCauseSpec;
impl crate::RegisterSpec for IntrCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause::R`](R) reader structure"]
impl crate::Readable for IntrCauseSpec {}
#[doc = "`reset()` method sets INTR_CAUSE to value 0"]
impl crate::Resettable for IntrCauseSpec {
    const RESET_VALUE: u32 = 0;
}
