#[doc = "Register `CWA_MSB` reader"]
pub type R = crate::R<CwaMsbSpec>;
#[doc = "Register `CWA_MSB` writer"]
pub type W = crate::W<CwaMsbSpec>;
#[doc = "Field `CWA_MSB` reader - Write Address for Common Area"]
pub type CwaMsbR = crate::BitReader;
#[doc = "Field `CWA_MSB` writer - Write Address for Common Area"]
pub type CwaMsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    pub fn cwa_msb(&self) -> CwaMsbR {
        CwaMsbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Address for Common Area"]
    #[inline(always)]
    #[must_use]
    pub fn cwa_msb(&mut self) -> CwaMsbW<CwaMsbSpec> {
        CwaMsbW::new(self, 0)
    }
}
#[doc = "Endpoint Read Address value *1\n\nYou can [`read`](crate::Reg::read) this register and get [`cwa_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwa_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwaMsbSpec;
impl crate::RegisterSpec for CwaMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwa_msb::R`](R) reader structure"]
impl crate::Readable for CwaMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`cwa_msb::W`](W) writer structure"]
impl crate::Writable for CwaMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWA_MSB to value 0"]
impl crate::Resettable for CwaMsbSpec {
    const RESET_VALUE: u32 = 0;
}
