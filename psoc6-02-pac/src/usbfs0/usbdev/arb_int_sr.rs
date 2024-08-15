#[doc = "Register `ARB_INT_SR` reader"]
pub type R = crate::R<ArbIntSrSpec>;
#[doc = "Field `EP1_INTR` reader - Interrupt status for EP1"]
pub type Ep1IntrR = crate::BitReader;
#[doc = "Field `EP2_INTR` reader - Interrupt status for EP2"]
pub type Ep2IntrR = crate::BitReader;
#[doc = "Field `EP3_INTR` reader - Interrupt status for EP3"]
pub type Ep3IntrR = crate::BitReader;
#[doc = "Field `EP4_INTR` reader - Interrupt status for EP4"]
pub type Ep4IntrR = crate::BitReader;
#[doc = "Field `EP5_INTR` reader - Interrupt status for EP5"]
pub type Ep5IntrR = crate::BitReader;
#[doc = "Field `EP6_INTR` reader - Interrupt status for EP6"]
pub type Ep6IntrR = crate::BitReader;
#[doc = "Field `EP7_INTR` reader - Interrupt status for EP7"]
pub type Ep7IntrR = crate::BitReader;
#[doc = "Field `EP8_INTR` reader - Interrupt status for EP8"]
pub type Ep8IntrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt status for EP1"]
    #[inline(always)]
    pub fn ep1_intr(&self) -> Ep1IntrR {
        Ep1IntrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt status for EP2"]
    #[inline(always)]
    pub fn ep2_intr(&self) -> Ep2IntrR {
        Ep2IntrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt status for EP3"]
    #[inline(always)]
    pub fn ep3_intr(&self) -> Ep3IntrR {
        Ep3IntrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt status for EP4"]
    #[inline(always)]
    pub fn ep4_intr(&self) -> Ep4IntrR {
        Ep4IntrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt status for EP5"]
    #[inline(always)]
    pub fn ep5_intr(&self) -> Ep5IntrR {
        Ep5IntrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt status for EP6"]
    #[inline(always)]
    pub fn ep6_intr(&self) -> Ep6IntrR {
        Ep6IntrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt status for EP7"]
    #[inline(always)]
    pub fn ep7_intr(&self) -> Ep7IntrR {
        Ep7IntrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt status for EP8"]
    #[inline(always)]
    pub fn ep8_intr(&self) -> Ep8IntrR {
        Ep8IntrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Arbiter Interrupt Status *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_int_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbIntSrSpec;
impl crate::RegisterSpec for ArbIntSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_int_sr::R`](R) reader structure"]
impl crate::Readable for ArbIntSrSpec {}
#[doc = "`reset()` method sets ARB_INT_SR to value 0"]
impl crate::Resettable for ArbIntSrSpec {
    const RESET_VALUE: u32 = 0;
}
