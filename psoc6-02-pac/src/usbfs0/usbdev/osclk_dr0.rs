#[doc = "Register `OSCLK_DR0` reader"]
pub type R = crate::R<OsclkDr0Spec>;
#[doc = "Field `ADDER` reader - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
pub type AdderR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits return the lower 8 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder(&self) -> AdderR {
        AdderR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Oscillator lock data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsclkDr0Spec;
impl crate::RegisterSpec for OsclkDr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr0::R`](R) reader structure"]
impl crate::Readable for OsclkDr0Spec {}
#[doc = "`reset()` method sets OSCLK_DR0 to value 0"]
impl crate::Resettable for OsclkDr0Spec {
    const RESET_VALUE: u32 = 0;
}
