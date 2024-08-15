#[doc = "Register `ICTAT_TRIM0` reader"]
pub type R = crate::R<IctatTrim0Spec>;
#[doc = "Register `ICTAT_TRIM0` writer"]
pub type W = crate::W<IctatTrim0Spec>;
#[doc = "Field `ICTAT_TRIM` reader - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type IctatTrimR = crate::FieldReader;
#[doc = "Field `ICTAT_TRIM` writer - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
pub type IctatTrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    pub fn ictat_trim(&self) -> IctatTrimR {
        IctatTrimR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ICTAT trim 0x00 : Minimum ICTAT current (~150nA at room) 0x0F : Maximum ICTAT current (~350nA at room)"]
    #[inline(always)]
    #[must_use]
    pub fn ictat_trim(&mut self) -> IctatTrimW<IctatTrim0Spec> {
        IctatTrimW::new(self, 0)
    }
}
#[doc = "ICTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`ictat_trim0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ictat_trim0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IctatTrim0Spec;
impl crate::RegisterSpec for IctatTrim0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ictat_trim0::R`](R) reader structure"]
impl crate::Readable for IctatTrim0Spec {}
#[doc = "`write(|w| ..)` method takes [`ictat_trim0::W`](W) writer structure"]
impl crate::Writable for IctatTrim0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICTAT_TRIM0 to value 0"]
impl crate::Resettable for IctatTrim0Spec {
    const RESET_VALUE: u32 = 0;
}
