#[doc = "Register `INTR_CAUSE_LO` reader"]
pub type R = crate::R<IntrCauseLoSpec>;
#[doc = "Field `SOF_INTR` reader - USB SOF Interrupt"]
pub type SofIntrR = crate::BitReader;
#[doc = "Field `BUS_RESET_INTR` reader - BUS RESET Interrupt"]
pub type BusResetIntrR = crate::BitReader;
#[doc = "Field `EP0_INTR` reader - EP0 Interrupt"]
pub type Ep0IntrR = crate::BitReader;
#[doc = "Field `LPM_INTR` reader - LPM Interrupt"]
pub type LpmIntrR = crate::BitReader;
#[doc = "Field `RESUME_INTR` reader - Resume Interrupt"]
pub type ResumeIntrR = crate::BitReader;
#[doc = "Field `ARB_EP_INTR` reader - Arbiter Endpoint Interrupt"]
pub type ArbEpIntrR = crate::BitReader;
#[doc = "Field `EP1_INTR` reader - EP1 Interrupt"]
pub type Ep1IntrR = crate::BitReader;
#[doc = "Field `EP2_INTR` reader - EP2 Interrupt"]
pub type Ep2IntrR = crate::BitReader;
#[doc = "Field `EP3_INTR` reader - EP3 Interrupt"]
pub type Ep3IntrR = crate::BitReader;
#[doc = "Field `EP4_INTR` reader - EP4 Interrupt"]
pub type Ep4IntrR = crate::BitReader;
#[doc = "Field `EP5_INTR` reader - EP5 Interrupt"]
pub type Ep5IntrR = crate::BitReader;
#[doc = "Field `EP6_INTR` reader - EP6 Interrupt"]
pub type Ep6IntrR = crate::BitReader;
#[doc = "Field `EP7_INTR` reader - EP7 Interrupt"]
pub type Ep7IntrR = crate::BitReader;
#[doc = "Field `EP8_INTR` reader - EP8 Interrupt"]
pub type Ep8IntrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - USB SOF Interrupt"]
    #[inline(always)]
    pub fn sof_intr(&self) -> SofIntrR {
        SofIntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BUS RESET Interrupt"]
    #[inline(always)]
    pub fn bus_reset_intr(&self) -> BusResetIntrR {
        BusResetIntrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EP0 Interrupt"]
    #[inline(always)]
    pub fn ep0_intr(&self) -> Ep0IntrR {
        Ep0IntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPM Interrupt"]
    #[inline(always)]
    pub fn lpm_intr(&self) -> LpmIntrR {
        LpmIntrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume Interrupt"]
    #[inline(always)]
    pub fn resume_intr(&self) -> ResumeIntrR {
        ResumeIntrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbiter Endpoint Interrupt"]
    #[inline(always)]
    pub fn arb_ep_intr(&self) -> ArbEpIntrR {
        ArbEpIntrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - EP1 Interrupt"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> Ep1IntrR {
        Ep1IntrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - EP2 Interrupt"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> Ep2IntrR {
        Ep2IntrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - EP3 Interrupt"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> Ep3IntrR {
        Ep3IntrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EP4 Interrupt"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> Ep4IntrR {
        Ep4IntrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - EP5 Interrupt"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> Ep5IntrR {
        Ep5IntrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - EP6 Interrupt"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> Ep6IntrR {
        Ep6IntrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - EP7 Interrupt"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> Ep7IntrR {
        Ep7IntrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EP8 Interrupt"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> Ep8IntrR {
        Ep8IntrR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Low priority interrupt Cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause_lo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCauseLoSpec;
impl crate::RegisterSpec for IntrCauseLoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause_lo::R`](R) reader structure"]
impl crate::Readable for IntrCauseLoSpec {}
#[doc = "`reset()` method sets INTR_CAUSE_LO to value 0"]
impl crate::Resettable for IntrCauseLoSpec {
    const RESET_VALUE: u32 = 0;
}
