#[doc = "Register `MSHC_VER_TYPE_R` reader"]
pub type R = crate::R<MshcVerTypeRSpec>;
#[doc = "Field `MSHC_VER_TYPE` reader - Current release type This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application. For example, release type is 'ga' is represented in ASCII as 0x6761. Lower 16 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
pub type MshcVerTypeR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current release type This field indicates the Synopsys DesignWare Cores DWC_mshc/DWC_mshc_lite current release type that is read by an application. For example, release type is 'ga' is represented in ASCII as 0x6761. Lower 16 bits read from this register can be ignored by the application. An application reading this register in conjunction with the MSHC_VER_ID_R register, gathers details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_type(&self) -> MshcVerTypeR {
        MshcVerTypeR::new(self.bits)
    }
}
#[doc = "MSHC version type\n\nYou can [`read`](crate::Reg::read) this register and get [`mshc_ver_type_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MshcVerTypeRSpec;
impl crate::RegisterSpec for MshcVerTypeRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mshc_ver_type_r::R`](R) reader structure"]
impl crate::Readable for MshcVerTypeRSpec {}
#[doc = "`reset()` method sets MSHC_VER_TYPE_R to value 0x6761_2a2a"]
impl crate::Resettable for MshcVerTypeRSpec {
    const RESET_VALUE: u32 = 0x6761_2a2a;
}
