#[doc = "Register `MS_CTL_READ_MIR[%s]` reader"]
pub type R = crate::R<MsCtlReadMirSpec>;
#[doc = "Field `PC` reader - Read-only mirror of MS_CTL.PC"]
pub type PcR = crate::FieldReader;
#[doc = "Field `PC_SAVED` reader - Read-only mirror of MS_CTL.PC_SAVED"]
pub type PcSavedR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Read-only mirror of MS_CTL.PC"]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Read-only mirror of MS_CTL.PC_SAVED"]
    #[inline(always)]
    pub fn pc_saved(&self) -> PcSavedR {
        PcSavedR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Master control read mirror\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_ctl_read_mir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsCtlReadMirSpec;
impl crate::RegisterSpec for MsCtlReadMirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_ctl_read_mir::R`](R) reader structure"]
impl crate::Readable for MsCtlReadMirSpec {}
#[doc = "`reset()` method sets MS_CTL_READ_MIR[%s]
to value 0"]
impl crate::Resettable for MsCtlReadMirSpec {
    const RESET_VALUE: u32 = 0;
}
