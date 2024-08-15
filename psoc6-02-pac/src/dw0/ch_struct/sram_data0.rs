#[doc = "Register `SRAM_DATA0` reader"]
pub type R = crate::R<SramData0Spec>;
#[doc = "Register `SRAM_DATA0` writer"]
pub type W = crate::W<SramData0Spec>;
#[doc = "Field `DATA` reader - N/A"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - N/A"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<SramData0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SRAM data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramData0Spec;
impl crate::RegisterSpec for SramData0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_data0::R`](R) reader structure"]
impl crate::Readable for SramData0Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_data0::W`](W) writer structure"]
impl crate::Writable for SramData0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_DATA0 to value 0"]
impl crate::Resettable for SramData0Spec {
    const RESET_VALUE: u32 = 0;
}
