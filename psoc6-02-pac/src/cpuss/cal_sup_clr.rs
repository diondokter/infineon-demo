#[doc = "Register `CAL_SUP_CLR` reader"]
pub type R = crate::R<CalSupClrSpec>;
#[doc = "Register `CAL_SUP_CLR` writer"]
pub type W = crate::W<CalSupClrSpec>;
#[doc = "Field `DATA` reader - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read side effect: when read all bits are cleared, write 1 to clear a specific bit Note: no exception for the debug host, it also causes the read side effect"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<CalSupClrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Calibration support clear and reset\n\nYou can [`read`](crate::Reg::read) this register and get [`cal_sup_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal_sup_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSupClrSpec;
impl crate::RegisterSpec for CalSupClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal_sup_clr::R`](R) reader structure"]
impl crate::Readable for CalSupClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cal_sup_clr::W`](W) writer structure"]
impl crate::Writable for CalSupClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL_SUP_CLR to value 0"]
impl crate::Resettable for CalSupClrSpec {
    const RESET_VALUE: u32 = 0;
}
