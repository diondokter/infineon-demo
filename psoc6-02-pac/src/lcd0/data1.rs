#[doc = "Register `DATA1[%s]` reader"]
pub type R = crate::R<Data1Spec>;
#[doc = "Register `DATA1[%s]` writer"]
pub type W = crate::W<Data1Spec>;
#[doc = "Field `DATA` reader - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 5-8 (COM5 is lsb)."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 5-8 (COM5 is lsb)."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 5-8 (COM5 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 5-8 (COM5 is lsb)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Data1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data1Spec;
impl crate::RegisterSpec for Data1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data1::R`](R) reader structure"]
impl crate::Readable for Data1Spec {}
#[doc = "`write(|w| ..)` method takes [`data1::W`](W) writer structure"]
impl crate::Writable for Data1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA1[%s]
to value 0"]
impl crate::Resettable for Data1Spec {
    const RESET_VALUE: u32 = 0;
}
