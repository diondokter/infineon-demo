#[doc = "Register `OSCLK_DR16` reader"]
pub type R = crate::R<OsclkDr16Spec>;
#[doc = "Field `ADDER16` reader - These bits return the oscillator locking circuits adder output."]
pub type Adder16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - These bits return the oscillator locking circuits adder output."]
    #[inline(always)]
    pub fn adder16(&self) -> Adder16R {
        Adder16R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "Oscillator lock data register\n\nYou can [`read`](crate::Reg::read) this register and get [`osclk_dr16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsclkDr16Spec;
impl crate::RegisterSpec for OsclkDr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osclk_dr16::R`](R) reader structure"]
impl crate::Readable for OsclkDr16Spec {}
#[doc = "`reset()` method sets OSCLK_DR16 to value 0"]
impl crate::Resettable for OsclkDr16Spec {
    const RESET_VALUE: u32 = 0;
}
