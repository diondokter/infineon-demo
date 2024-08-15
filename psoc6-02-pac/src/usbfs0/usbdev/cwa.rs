#[doc = "Register `CWA` reader"]
pub type R = crate::R<CwaSpec>;
#[doc = "Register `CWA` writer"]
pub type W = crate::W<CwaSpec>;
#[doc = "Field `CWA` reader - Write Address for Common Area"]
pub type CwaR = crate::FieldReader;
#[doc = "Field `CWA` writer - Write Address for Common Area"]
pub type CwaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa(&self) -> CwaR {
        CwaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa(&mut self) -> CwaW<CwaSpec> {
        CwaW::new(self, 0)
    }
}
#[doc = "Common Area Write Address *1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwaSpec;
impl crate::RegisterSpec for CwaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa::R`](R) reader structure"]
impl crate::Readable for CwaSpec {}
#[doc = "`write(|w| ..)` method takes [`cwa::W`](W) writer structure"]
impl crate::Writable for CwaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWA to value 0"]
impl crate::Resettable for CwaSpec {
    const RESET_VALUE: u32 = 0;
}
