#[doc = "Register `INTR_USBHOST_CAUSE_LO` reader"]
pub type R = crate::R<IntrUsbhostCauseLoSpec>;
#[doc = "Field `SOFIRQ_INT` reader - SOFIRQ interrupt"]
pub type SofirqIntR = crate::BitReader;
#[doc = "Field `DIRQ_INT` reader - DIRQ interrupt"]
pub type DirqIntR = crate::BitReader;
#[doc = "Field `CNNIRQ_INT` reader - CNNIRQ interrupt"]
pub type CnnirqIntR = crate::BitReader;
#[doc = "Field `CMPIRQ_INT` reader - CMPIRQ interrupt"]
pub type CmpirqIntR = crate::BitReader;
#[doc = "Field `URIRQ_INT` reader - URIRQ interrupt"]
pub type UrirqIntR = crate::BitReader;
#[doc = "Field `RWKIRQ_INT` reader - RWKIRQ interrupt"]
pub type RwkirqIntR = crate::BitReader;
#[doc = "Field `RSVD_6` reader - N/A"]
pub type Rsvd6R = crate::BitReader;
#[doc = "Field `TCAN_INT` reader - TCAN interrupt"]
pub type TcanIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SOFIRQ interrupt"]
    #[inline(always)]
    pub fn sofirq_int(&self) -> SofirqIntR {
        SofirqIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIRQ interrupt"]
    #[inline(always)]
    pub fn dirq_int(&self) -> DirqIntR {
        DirqIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNNIRQ interrupt"]
    #[inline(always)]
    pub fn cnnirq_int(&self) -> CnnirqIntR {
        CnnirqIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMPIRQ interrupt"]
    #[inline(always)]
    pub fn cmpirq_int(&self) -> CmpirqIntR {
        CmpirqIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - URIRQ interrupt"]
    #[inline(always)]
    pub fn urirq_int(&self) -> UrirqIntR {
        UrirqIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RWKIRQ interrupt"]
    #[inline(always)]
    pub fn rwkirq_int(&self) -> RwkirqIntR {
        RwkirqIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn rsvd_6(&self) -> Rsvd6R {
        Rsvd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCAN interrupt"]
    #[inline(always)]
    pub fn tcan_int(&self) -> TcanIntR {
        TcanIntR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt USB Host Cause Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_usbhost_cause_lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrUsbhostCauseLoSpec;
impl crate::RegisterSpec for IntrUsbhostCauseLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_usbhost_cause_lo::R`](R) reader structure"]
impl crate::Readable for IntrUsbhostCauseLoSpec {}
#[doc = "`reset()` method sets INTR_USBHOST_CAUSE_LO to value 0"]
impl crate::Resettable for IntrUsbhostCauseLoSpec {
    const RESET_VALUE: u32 = 0;
}
