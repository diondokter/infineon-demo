#[doc = "Register `IDENTITY` reader"]
pub type R = crate::R<IdentitySpec>;
#[doc = "Field `P` reader - This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
pub type PR = crate::BitReader;
#[doc = "Field `NS` reader - This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
pub type NsR = crate::BitReader;
#[doc = "Field `PC` reader - This field specifies the protection context of the transfer that reads the register."]
pub type PcR = crate::FieldReader;
#[doc = "Field `MS` reader - This field specifies the bus master identifier of the transfer that reads the register."]
pub type MsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This field specifies the privileged setting ('0': user mode; '1': privileged mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field specifies the security setting ('0': secure mode; '1': non-secure mode) of the transfer that reads the register."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This field specifies the protection context of the transfer that reads the register."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - This field specifies the bus master identifier of the transfer that reads the register."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "Identity\n\nYou can [`read`](crate::Reg::read) this register and get [`identity::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdentitySpec;
impl crate::RegisterSpec for IdentitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`identity::R`](R) reader structure"]
impl crate::Readable for IdentitySpec {}
#[doc = "`reset()` method sets IDENTITY to value 0"]
impl crate::Resettable for IdentitySpec {
    const RESET_VALUE: u32 = 0;
}
