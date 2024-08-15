#[doc = "Register `OSCLK_DR1` reader"]
pub type R = crate::R<OsclkDr1Spec>;
#[doc = "Field `ADDER_MSB` reader - These bits return the upper 7 bits of the oscillator locking circuits adder output."]
pub type AdderMsbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - These bits return the upper 7 bits of the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder_msb(&self) -> AdderMsbR {
        AdderMsbR::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "Oscillator lock data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsclkDr1Spec;
impl crate::RegisterSpec for OsclkDr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr1::R`](R) reader structure"]
impl crate::Readable for OsclkDr1Spec {}
#[doc = "`reset()` method sets OSCLK_DR1 to value 0"]
impl crate::Resettable for OsclkDr1Spec {
    const RESET_VALUE: u32 = 0;
}
