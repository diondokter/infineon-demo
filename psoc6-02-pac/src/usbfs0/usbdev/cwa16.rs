#[doc = "Register `CWA16` reader"]
pub type R = crate::R<Cwa16Spec>;
#[doc = "Register `CWA16` writer"]
pub type W = crate::W<Cwa16Spec>;
#[doc = "Field `CWA16` reader - Write Address for Common Area"]
pub type Cwa16R = crate::FieldReader<u16>;
#[doc = "Field `CWA16` writer - Write Address for Common Area"]
pub type Cwa16W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa16(&self) -> Cwa16R {
        Cwa16R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa16(&mut self) -> Cwa16W<Cwa16Spec> {
        Cwa16W::new(self, 0)
    }
}
#[doc = "Common Area Write Address\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cwa16Spec;
impl crate::RegisterSpec for Cwa16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa16::R`](R) reader structure"]
impl crate::Readable for Cwa16Spec {}
#[doc = "`write(|w| ..)` method takes [`cwa16::W`](W) writer structure"]
impl crate::Writable for Cwa16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWA16 to value 0"]
impl crate::Resettable for Cwa16Spec {
    const RESET_VALUE: u32 = 0;
}
