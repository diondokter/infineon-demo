#[doc = "Register `GP_IN_R` reader"]
pub type R = crate::R<GpInRSpec>;
#[doc = "Field `GP_IN` reader - It reflects the value of gp_in ports. NOT USED - ALWAYS READS 0"]
pub type GpInR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - It reflects the value of gp_in ports. NOT USED - ALWAYS READS 0"]
    #[inline(always)]
    pub fn gp_in(&self) -> GpInR {
        GpInR::new((self.bits & 1) != 0)
    }
}
#[doc = "General Purpose Input register\n\nYou can [`read`](crate::Reg::read) this register and get [`gp_in_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpInRSpec;
impl crate::RegisterSpec for GpInRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gp_in_r::R`](R) reader structure"]
impl crate::Readable for GpInRSpec {}
#[doc = "`reset()` method sets GP_IN_R to value 0"]
impl crate::Resettable for GpInRSpec {
    const RESET_VALUE: u32 = 0;
}
