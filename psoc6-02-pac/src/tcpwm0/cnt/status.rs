#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `DOWN` reader - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
pub type DownR = crate::BitReader;
#[doc = "Field `GENERIC` reader - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
pub type GenericR = crate::FieldReader;
#[doc = "Field `RUNNING` reader - When '0', the counter is NOT running. When '1', the counter is running."]
pub type RunningR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When '0', counter is counting up. When '1', counter is counting down. In QUAD mode, this field indicates the direction of the latest counter change: '0' when last incremented and '1' when last decremented."]
    #[inline(always)]
    pub fn down(&self) -> DownR {
        DownR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Generic 8-bit counter field. In PWM_DT mode, this counter is used for dead time insertion. In all other modes, this counter is used for pre-scaling the selected counter clock. PWM_DT mode can NOT use prescaled clock functionality."]
    #[inline(always)]
    pub fn generic(&self) -> GenericR {
        GenericR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - When '0', the counter is NOT running. When '1', the counter is running."]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Counter status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
