#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `OUT0` reader - Current output value of the comparator 0."]
pub type Out0R = crate::BitReader;
#[doc = "Field `OUT1` reader - Current output value of the comparator 1."]
pub type Out1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current output value of the comparator 0."]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Current output value of the comparator 1."]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "LPCOMP Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
