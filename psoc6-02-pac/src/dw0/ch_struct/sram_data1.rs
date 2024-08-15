#[doc = "Register `SRAM_DATA1` reader"]
pub type R = crate::R<SramData1Spec>;
#[doc = "Register `SRAM_DATA1` writer"]
pub type W = crate::W<SramData1Spec>;
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
    pub fn data(&mut self) -> DataW<SramData1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "SRAM data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramData1Spec;
impl crate::RegisterSpec for SramData1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_data1::R`](R) reader structure"]
impl crate::Readable for SramData1Spec {}
#[doc = "`write(|w| ..)` method takes [`sram_data1::W`](W) writer structure"]
impl crate::Writable for SramData1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRAM_DATA1 to value 0"]
impl crate::Resettable for SramData1Spec {
    const RESET_VALUE: u32 = 0;
}
