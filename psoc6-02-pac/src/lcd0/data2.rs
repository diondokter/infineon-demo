#[doc = "Register `DATA2[%s]` reader"]
pub type R = crate::R<Data2Spec>;
#[doc = "Register `DATA2[%s]` writer"]
pub type W = crate::W<Data2Spec>;
#[doc = "Field `DATA` reader - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 9-12 (COM9 is lsb)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Data2Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data2Spec;
impl crate::RegisterSpec for Data2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data2::R`](R) reader structure"]
impl crate::Readable for Data2Spec {}
#[doc = "`write(|w| ..)` method takes [`data2::W`](W) writer structure"]
impl crate::Writable for Data2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA2[%s]
to value 0"]
impl crate::Resettable for Data2Spec {
    const RESET_VALUE: u32 = 0;
}
