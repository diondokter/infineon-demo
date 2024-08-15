#[doc = "Register `WDT_CNT` reader"]
pub type R = crate::R<WdtCntSpec>;
#[doc = "Register `WDT_CNT` writer"]
pub type W = crate::W<WdtCntSpec>;
#[doc = "Field `COUNTER` reader - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
pub type CounterR = crate::FieldReader<u16>;
#[doc = "Field `COUNTER` writer - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current value of WDT Counter. The write feature of this register is for engineering use (DfV), have no synchronization, and can only be applied when the WDT is fully off. When writing, the value is updated immediately in the WDT counter, but it will read back as the old value until this register resynchronizes just after the negedge of ILO. Writes will be ignored if they occur when the WDT is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<WdtCntSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "Watchdog Counter Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtCntSpec;
impl crate::RegisterSpec for WdtCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt_cnt::R`](R) reader structure"]
impl crate::Readable for WdtCntSpec {}
#[doc = "`write(|w| ..)` method takes [`wdt_cnt::W`](W) writer structure"]
impl crate::Writable for WdtCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDT_CNT to value 0"]
impl crate::Resettable for WdtCntSpec {
    const RESET_VALUE: u32 = 0;
}
