#[doc = "Register `HOST_CNTRL_VERS_R` reader"]
pub type R = crate::R<HostCntrlVersRSpec>;
#[doc = "Field `SPEC_VERSION_NUM` reader - N/A"]
pub type SpecVersionNumR = crate::FieldReader;
#[doc = "Field `VENDOR_VERSION_NUM` reader - N/A"]
pub type VendorVersionNumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn spec_version_num(&self) -> SpecVersionNumR {
        SpecVersionNumR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - N/A"]
    #[inline(always)]
    pub fn vendor_version_num(&self) -> VendorVersionNumR {
        VendorVersionNumR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nYou can [`read`](crate::Reg::read) this register and get [`host_cntrl_vers_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCntrlVersRSpec;
impl crate::RegisterSpec for HostCntrlVersRSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`host_cntrl_vers_r::R`](R) reader structure"]
impl crate::Readable for HostCntrlVersRSpec {}
#[doc = "`reset()` method sets HOST_CNTRL_VERS_R to value 0x05"]
impl crate::Resettable for HostCntrlVersRSpec {
    const RESET_VALUE: u16 = 0x05;
}
