#[doc = "Register `LPM_STAT` reader"]
pub type R = crate::R<LpmStatSpec>;
#[doc = "Field `LPM_BESL` reader - Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
pub type LpmBeslR = crate::FieldReader;
#[doc = "Field `LPM_REMOTEWAKE` reader - 0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
pub type LpmRemotewakeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Best Effort Service Latency This value should match either the Baseline (DeepSleep) or Deep (Hibernate) BESL in the BOS descriptor."]
    #[inline(always)]
    pub fn lpm_besl(&self) -> LpmBeslR {
        LpmBeslR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 0: Device is prohibited from initiating a remote wake 1: Device is allow to wake the host"]
    #[inline(always)]
    pub fn lpm_remotewake(&self) -> LpmRemotewakeR {
        LpmRemotewakeR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "LPM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmStatSpec;
impl crate::RegisterSpec for LpmStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm_stat::R`](R) reader structure"]
impl crate::Readable for LpmStatSpec {}
#[doc = "`reset()` method sets LPM_STAT to value 0"]
impl crate::Resettable for LpmStatSpec {
    const RESET_VALUE: u32 = 0;
}
