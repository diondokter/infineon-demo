#[doc = "Register `LOCK_STATUS` reader"]
pub type R = crate::R<LockStatusSpec>;
#[doc = "Field `P` reader - This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
pub type PR = crate::BitReader;
#[doc = "Field `NS` reader - This field specifies the secure/non-secure access control: '0': secure. '1': non-secure."]
pub type NsR = crate::BitReader;
#[doc = "Field `PC` reader - This field specifies the protection context that successfully acquired the lock."]
pub type PcR = crate::FieldReader;
#[doc = "Field `MS` reader - This field specifies the bus master identifier that successfully acquired the lock."]
pub type MsR = crate::FieldReader;
#[doc = "Field `ACQUIRED` reader - Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1'). If zero, P, NS, PC, and MS are not valid."]
pub type AcquiredR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This field specifies the user/privileged access control: '0': user mode. '1': privileged mode."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field specifies the secure/non-secure access control: '0': secure. '1': non-secure."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context that successfully acquired the lock."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier that successfully acquired the lock."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Specifies if the lock is acquired. This field is set to '1', if a ACQUIRE read transfer successfully acquires the lock (the ACQUIRE read transfer returns ACQUIRE.SUCCESS as '1'). If zero, P, NS, PC, and MS are not valid."]
    #[inline(always)]
    pub fn acquired(&self) -> AcquiredR {
        AcquiredR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "IPC lock status\n\nYou can [`read`](crate::Reg::read) this register and get [`lock_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockStatusSpec;
impl crate::RegisterSpec for LockStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock_status::R`](R) reader structure"]
impl crate::Readable for LockStatusSpec {}
#[doc = "`reset()` method sets LOCK_STATUS to value 0"]
impl crate::Resettable for LockStatusSpec {
    const RESET_VALUE: u32 = 0;
}
