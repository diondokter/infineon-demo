#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `P` reader - Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
pub type PR = crate::BitReader;
#[doc = "Field `NS` reader - Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
pub type NsR = crate::BitReader;
#[doc = "Field `B` reader - Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
pub type BR = crate::BitReader;
#[doc = "Field `PC` reader - Active channel protection context."]
pub type PcR = crate::FieldReader;
#[doc = "Field `PRIO` reader - Active channel priority."]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PREEMPTABLE` reader - Active channel preemptable."]
pub type PreemptableR = crate::BitReader;
#[doc = "Field `CH_IDX` reader - Active channel index."]
pub type ChIdxR = crate::FieldReader<u16>;
#[doc = "Field `STATE` reader - State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': CRC functionality (only used for CRC transfer descriptor type). '5': Update of active control information (e.g. source and destination addresses) and wait for trigger de-activation. '6': Error."]
pub type StateR = crate::FieldReader;
#[doc = "Field `ACTIVE` reader - Active channel present: '0': No. '1': Yes."]
pub type ActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Active channel, user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active channel, secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active channel, non-bufferable/bufferable access control: '0': non-bufferable '1': bufferable."]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Active channel protection context."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Active channel priority."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Active channel preemptable."]
    #[inline(always)]
    pub fn preemptable(&self) -> PreemptableR {
        PreemptableR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Active channel index."]
    #[inline(always)]
    pub fn ch_idx(&self) -> ChIdxR {
        ChIdxR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 28:30 - State of the DW controller. '0': Default/inactive state. '1': Loading descriptor. '2': Loading data element from source location. '3': Storing data element to destination location. '4': CRC functionality (only used for CRC transfer descriptor type). '5': Update of active control information (e.g. source and destination addresses) and wait for trigger de-activation. '6': Error."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Active channel present: '0': No. '1': Yes."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
