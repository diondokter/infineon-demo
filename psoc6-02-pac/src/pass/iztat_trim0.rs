#[doc = "Register `IZTAT_TRIM0` reader"]
pub type R = crate::R<IztatTrim0Spec>;
#[doc = "Register `IZTAT_TRIM0` writer"]
pub type W = crate::W<IztatTrim0Spec>;
#[doc = "Field `IZTAT_ABS_TRIM` reader - N/A"]
pub type IztatAbsTrimR = crate::FieldReader;
#[doc = "Field `IZTAT_ABS_TRIM` writer - N/A"]
pub type IztatAbsTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    pub fn iztat_abs_trim(&self) -> IztatAbsTrimR {
        IztatAbsTrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn iztat_abs_trim(&mut self) -> IztatAbsTrimW<IztatTrim0Spec> {
        IztatAbsTrimW::new(self, 0)
    }
}
#[doc = "IZTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iztat_trim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iztat_trim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IztatTrim0Spec;
impl crate::RegisterSpec for IztatTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iztat_trim0::R`](R) reader structure"]
impl crate::Readable for IztatTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`iztat_trim0::W`](W) writer structure"]
impl crate::Writable for IztatTrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IZTAT_TRIM0 to value 0"]
impl crate::Resettable for IztatTrim0Spec {
    const RESET_VALUE: u32 = 0;
}
