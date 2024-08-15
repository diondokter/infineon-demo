#[doc = "Register `IZTAT_TRIM1` reader"]
pub type R = crate::R<IztatTrim1Spec>;
#[doc = "Register `IZTAT_TRIM1` writer"]
pub type W = crate::W<IztatTrim1Spec>;
#[doc = "Field `IZTAT_TC_TRIM` reader - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IztatTcTrimR = crate::FieldReader;
#[doc = "Field `IZTAT_TC_TRIM` writer - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
pub type IztatTcTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    pub fn iztat_tc_trim(&self) -> IztatTcTrimR {
        IztatTcTrimR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IZTAT temperature correction trim (RMB) 0x00 : No IZTAT temperature correction 0xFF : Maximum IZTAT temperature correction As this is a Risk Mitigation Register, it should be loaded with 0x08."]
    #[inline(always)]
    #[must_use]
    pub fn iztat_tc_trim(&mut self) -> IztatTcTrimW<IztatTrim1Spec> {
        IztatTcTrimW::new(self, 0)
    }
}
#[doc = "IZTAT Trim bits\n\nYou can [`read`](crate::Reg::read) this register and get [`iztat_trim1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iztat_trim1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IztatTrim1Spec;
impl crate::RegisterSpec for IztatTrim1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iztat_trim1::R`](R) reader structure"]
impl crate::Readable for IztatTrim1Spec {}
#[doc = "`write(|w| ..)` method takes [`iztat_trim1::W`](W) writer structure"]
impl crate::Writable for IztatTrim1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IZTAT_TRIM1 to value 0"]
impl crate::Resettable for IztatTrim1Spec {
    const RESET_VALUE: u32 = 0;
}
