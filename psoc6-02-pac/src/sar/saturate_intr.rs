#[doc = "Register `SATURATE_INTR` reader"]
pub type R = crate::R<SaturateIntrSpec>;
#[doc = "Register `SATURATE_INTR` writer"]
pub type W = crate::W<SaturateIntrSpec>;
#[doc = "Field `SATURATE_INTR` reader - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SaturateIntrR = crate::FieldReader<u16>;
#[doc = "Field `SATURATE_INTR` writer - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
pub type SaturateIntrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn saturate_intr(&self) -> SaturateIntrR {
        SaturateIntrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Saturate Interrupt: hardware sets this interrupt for each channel if a conversion result (before averaging) of that channel is either 0x000 or 0xFFF, this is an indication that the ADC likely saturated. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn saturate_intr(&mut self) -> SaturateIntrW<SaturateIntrSpec> {
        SaturateIntrW::new(self, 0)
    }
}
#[doc = "Saturate interrupt request register.\n\nYou can [`read`](crate::Reg::read) this register and get [`saturate_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saturate_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaturateIntrSpec;
impl crate::RegisterSpec for SaturateIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saturate_intr::R`](R) reader structure"]
impl crate::Readable for SaturateIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`saturate_intr::W`](W) writer structure"]
impl crate::Writable for SaturateIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SATURATE_INTR to value 0"]
impl crate::Resettable for SaturateIntrSpec {
    const RESET_VALUE: u32 = 0;
}
