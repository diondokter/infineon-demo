#[doc = "Register `CM0_PC3_HANDLER` reader"]
pub type R = crate::R<Cm0Pc3HandlerSpec>;
#[doc = "Register `CM0_PC3_HANDLER` writer"]
pub type W = crate::W<Cm0Pc3HandlerSpec>;
#[doc = "Field `ADDR` reader - Address of the protection context 3 handler."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address of the protection context 3 handler."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the protection context 3 handler."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address of the protection context 3 handler."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<Cm0Pc3HandlerSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "CM0+ protection context 3 handler\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc3_handler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc3_handler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0Pc3HandlerSpec;
impl crate::RegisterSpec for Cm0Pc3HandlerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_pc3_handler::R`](R) reader structure"]
impl crate::Readable for Cm0Pc3HandlerSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_pc3_handler::W`](W) writer structure"]
impl crate::Writable for Cm0Pc3HandlerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_PC3_HANDLER to value 0"]
impl crate::Resettable for Cm0Pc3HandlerSpec {
    const RESET_VALUE: u32 = 0;
}
