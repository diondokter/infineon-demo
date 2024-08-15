#[doc = "Register `VU_STATUS` reader"]
pub type R = crate::R<VuStatusSpec>;
#[doc = "Field `CARRY` reader - STATUS CARRY field."]
pub type CarryR = crate::BitReader;
#[doc = "Field `EVEN` reader - STATUS EVEN field."]
pub type EvenR = crate::BitReader;
#[doc = "Field `ZERO` reader - STATUS ZERO field."]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ONE` reader - STATUS ONE field."]
pub type OneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - STATUS CARRY field."]
    #[inline(always)]
    pub fn carry(&self) -> CarryR {
        CarryR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STATUS EVEN field."]
    #[inline(always)]
    pub fn even(&self) -> EvenR {
        EvenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STATUS ZERO field."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STATUS ONE field."]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Vector unit status\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VuStatusSpec;
impl crate::RegisterSpec for VuStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vu_status::R`](R) reader structure"]
impl crate::Readable for VuStatusSpec {}
#[doc = "`reset()` method sets VU_STATUS to value 0"]
impl crate::Resettable for VuStatusSpec {
    const RESET_VALUE: u32 = 0;
}
