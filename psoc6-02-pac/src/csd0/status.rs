#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CSD_SENSE` reader - Signal used to drive the Cs switches."]
pub type CsdSenseR = crate::BitReader;
#[doc = "Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HscmpOut {
    #[doc = "0: Vin &lt; Vref"]
    CLtVref = 0,
    #[doc = "1: Vin > Vref"]
    CGtVref = 1,
}
impl From<HscmpOut> for bool {
    #[inline(always)]
    fn from(variant: HscmpOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCMP_OUT` reader - Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
pub type HscmpOutR = crate::BitReader<HscmpOut>;
impl HscmpOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HscmpOut {
        match self.bits {
            false => HscmpOut::CLtVref,
            true => HscmpOut::CGtVref,
        }
    }
    #[doc = "Vin &lt; Vref"]
    #[inline(always)]
    pub fn is_c_lt_vref(&self) -> bool {
        *self == HscmpOut::CLtVref
    }
    #[doc = "Vin > Vref"]
    #[inline(always)]
    pub fn is_c_gt_vref(&self) -> bool {
        *self == HscmpOut::CGtVref
    }
}
#[doc = "Field `CSDCMP_OUT` reader - Output of main sensing comparator (synchronized)"]
pub type CsdcmpOutR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Signal used to drive the Cs switches."]
    #[inline(always)]
    pub fn csd_sense(&self) -> CsdSenseR {
        CsdSenseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output of reference buffer comparator used to charge up Cmod and/or Csh_tank (synchronized)"]
    #[inline(always)]
    pub fn hscmp_out(&self) -> HscmpOutR {
        HscmpOutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output of main sensing comparator (synchronized)"]
    #[inline(always)]
    pub fn csdcmp_out(&self) -> CsdcmpOutR {
        CsdcmpOutR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
