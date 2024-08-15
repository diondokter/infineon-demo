#[doc = "Register `TR_CTL2` reader"]
pub type R = crate::R<TrCtl2Spec>;
#[doc = "Register `TR_CTL2` writer"]
pub type W = crate::W<TrCtl2Spec>;
#[doc = "Field `SIZE` reader - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit size of generated random number in TR_RESULT. Legal range is in \\[0, 32\\]."]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<TrCtl2Spec> {
        SizeW::new(self, 0)
    }
}
#[doc = "True random control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtl2Spec;
impl crate::RegisterSpec for TrCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl2::R`](R) reader structure"]
impl crate::Readable for TrCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl2::W`](W) writer structure"]
impl crate::Writable for TrCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTL2 to value 0"]
impl crate::Resettable for TrCtl2Spec {
    const RESET_VALUE: u32 = 0;
}
