#[doc = "Register `CAL_SUP_SET` reader"]
pub type R = crate::R<CalSupSetSpec>;
#[doc = "Register `CAL_SUP_SET` writer"]
pub type W = crate::W<CalSupSetSpec>;
#[doc = "Field `DATA` reader - Read without side effect, write 1 to set"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read without side effect, write 1 to set"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read without side effect, write 1 to set"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read without side effect, write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CalSupSetSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Calibration support set and read\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_sup_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_sup_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSupSetSpec;
impl crate::RegisterSpec for CalSupSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_sup_set::R`](R) reader structure"]
impl crate::Readable for CalSupSetSpec {}
#[doc = "`write(|w| ..)` method takes [`cal_sup_set::W`](W) writer structure"]
impl crate::Writable for CalSupSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_SUP_SET to value 0"]
impl crate::Resettable for CalSupSetSpec {
    const RESET_VALUE: u32 = 0;
}
