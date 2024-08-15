#[doc = "Register `ACQUIRE` reader"]
pub type R = crate::R<AcquireSpec>;
#[doc = "Field `P` reader - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
pub type PR = crate::BitReader;
#[doc = "Field `NS` reader - Secure/non-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
pub type NsR = crate::BitReader;
#[doc = "Field `PC` reader - This field specifies the protection context that successfully acquired the lock."]
pub type PcR = crate::FieldReader;
#[doc = "Field `MS` reader - This field specifies the bus master identifier that successfully acquired the lock."]
pub type MsR = crate::FieldReader;
#[doc = "Field `SUCCESS` reader - Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
pub type SuccessR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - User/privileged access control: '0': user mode. '1': privileged mode. This field is set with the user/privileged access control of the access that successfully acquired the lock."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure/non-secure access control: '0': secure. '1': non-secure. This field is set with the secure/non-secure access control of the access that successfully acquired the lock."]
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
    #[doc = "Bit 31 - Specifies if the lock is successfully acquired or not (reading the ACQUIRE register can have affect on SUCCESS and LOCK_STATUS.ACQUIRED): '0': Not successfully acquired; i.e. the lock was already acquired by another read transaction and not released. The P, NS, PC and MS fields reflect the access attributes of the transaction that previously successfully acuired the lock; the fields are NOT affected by the current access. '1': Successfully acquired. The P, NS, PC and MS fields reflect the access attributes of the current access. Note that this field is NOT SW writable. A lock is released by writing to the associated RELEASE register (irrespective of the write value)."]
    #[inline(always)]
    pub fn success(&self) -> SuccessR {
        SuccessR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "IPC acquire\n\nYou can [`read`](crate::Reg::read) this register and get [`acquire::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcquireSpec;
impl crate::RegisterSpec for AcquireSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acquire::R`](R) reader structure"]
impl crate::Readable for AcquireSpec {}
#[doc = "`reset()` method sets ACQUIRE to value 0"]
impl crate::Resettable for AcquireSpec {
    const RESET_VALUE: u32 = 0;
}
