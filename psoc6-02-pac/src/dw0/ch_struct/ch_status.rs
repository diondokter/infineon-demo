#[doc = "Register `CH_STATUS` reader"]
pub type R = crate::R<ChStatusSpec>;
#[doc = "Field `INTR_CAUSE` reader - Specifies the source of the interrupt cause: '0': No interrupt generated '1': Interrupt based on transfer complettion configuration based on INTR_TYPE '2': Source transfer bus error '3': Destination transfer bus error '4': Source address misalignment '5': Destination address misalignment '6': Current descriptor pointer is null '7': Active channel is disabled '8': Descriptor bus error '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
pub type IntrCauseR = crate::FieldReader;
#[doc = "Field `PENDING` reader - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
pub type PendingR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Specifies the source of the interrupt cause: '0': No interrupt generated '1': Interrupt based on transfer complettion configuration based on INTR_TYPE '2': Source transfer bus error '3': Destination transfer bus error '4': Source address misalignment '5': Destination address misalignment '6': Current descriptor pointer is null '7': Active channel is disabled '8': Descriptor bus error '9'-'15': Not used. For error related interrupt causes (INTR_CAUSE is '2', '3', ..., '8'), the channel is disabled (HW sets CH_CTL.ENABLED to '0')."]
    #[inline(always)]
    pub fn intr_cause(&self) -> IntrCauseR {
        IntrCauseR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies pending DW channels; i.e. enabled channels whose trigger got activated. This field includes all channels that are in the pending state (not scheduled) or active state (scheduled and performing data transfer(s))."]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel status\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChStatusSpec;
impl crate::RegisterSpec for ChStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_status::R`](R) reader structure"]
impl crate::Readable for ChStatusSpec {}
#[doc = "`reset()` method sets CH_STATUS to value 0"]
impl crate::Resettable for ChStatusSpec {
    const RESET_VALUE: u32 = 0;
}
