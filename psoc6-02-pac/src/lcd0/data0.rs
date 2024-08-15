#[doc = "Register `DATA0[%s]` reader"]
pub type R = crate::R<Data0Spec>;
#[doc = "Register `DATA0[%s]` writer"]
pub type W = crate::W<Data0Spec>;
#[doc = "Field `DATA` reader - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 1-4 (COM1 is lsb)."]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 1-4 (COM1 is lsb)."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 1-4 (COM1 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\]
represent the pin data for pin \\[i\\]
for COMS 1-4 (COM1 is lsb)."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<Data0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data0Spec;
impl crate::RegisterSpec for Data0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0::R`](R) reader structure"]
impl crate::Readable for Data0Spec {}
#[doc = "`write(|w| ..)` method takes [`data0::W`](W) writer structure"]
impl crate::Writable for Data0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0[%s]
to value 0"]
impl crate::Resettable for Data0Spec {
    const RESET_VALUE: u32 = 0;
}
