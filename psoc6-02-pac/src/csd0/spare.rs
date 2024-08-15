#[doc = "Register `SPARE` reader"]
pub type R = crate::R<SpareSpec>;
#[doc = "Register `SPARE` writer"]
pub type W = crate::W<SpareSpec>;
#[doc = "Field `SPARE` reader - Spare MMIO"]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Spare MMIO"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Spare MMIO"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<SpareSpec> {
        SpareW::new(self, 0)
    }
}
#[doc = "Spare MMIO\n\nYou can [`read`](crate::Reg::read) this register and get [`spare::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spare::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpareSpec;
impl crate::RegisterSpec for SpareSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spare::R`](R) reader structure"]
impl crate::Readable for SpareSpec {}
#[doc = "`write(|w| ..)` method takes [`spare::W`](W) writer structure"]
impl crate::Writable for SpareSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPARE to value 0"]
impl crate::Resettable for SpareSpec {
    const RESET_VALUE: u32 = 0;
}
