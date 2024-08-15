#[doc = "Register `CQVER` reader"]
pub type R = crate::R<CqverSpec>;
#[doc = "Field `EMMC_VER_SUFFIX` reader - This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
pub type EmmcVerSuffixR = crate::FieldReader;
#[doc = "Field `EMMC_VER_MINOR` reader - This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
pub type EmmcVerMinorR = crate::FieldReader;
#[doc = "Field `EMMC_VER_MAJOR` reader - This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
pub type EmmcVerMajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - This bit indicates the eMMC version suffix (2nd digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_suffix(&self) -> EmmcVerSuffixR {
        EmmcVerSuffixR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This bit indicates the eMMC minor version (1st digit right of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_minor(&self) -> EmmcVerMinorR {
        EmmcVerMinorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This bit indicates the eMMC major version (1st digit left of decimal point) in BCD format."]
    #[inline(always)]
    pub fn emmc_ver_major(&self) -> EmmcVerMajorR {
        EmmcVerMajorR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Command Queuing Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqver::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqverSpec;
impl crate::RegisterSpec for CqverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqver::R`](R) reader structure"]
impl crate::Readable for CqverSpec {}
#[doc = "`reset()` method sets CQVER to value 0x0510"]
impl crate::Resettable for CqverSpec {
    const RESET_VALUE: u32 = 0x0510;
}
