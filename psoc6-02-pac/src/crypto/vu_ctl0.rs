#[doc = "Register `VU_CTL0` reader"]
pub type R = crate::R<VuCtl0Spec>;
#[doc = "Register `VU_CTL0` writer"]
pub type W = crate::W<VuCtl0Spec>;
#[doc = "Field `ALWAYS_EXECUTE` reader - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
pub type AlwaysExecuteR = crate::BitReader;
#[doc = "Field `ALWAYS_EXECUTE` writer - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
pub type AlwaysExecuteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
    #[inline(always)]
    pub fn always_execute(&self) -> AlwaysExecuteR {
        AlwaysExecuteR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if a conditional instruction is executed or not, when its condition code evaluates to false/'0'. '0': The instruction is NOT executed. As a result, the instruction may be handled faster than when it is executed. '1': The instruction is executed, but the execution result (including status field information) is not reflected in the IP. The instruction is handled just as fast as when it is executed. Note: a conditional instruction with a condition code that evaluates to false/'0' does not affect the architectural state: VU_STATUS fields, memory or register-file data. Note: Always execution is useful to prevent/complicate differential timing and differential power attacks."]
    #[inline(always)]
    #[must_use]
    pub fn always_execute(&mut self) -> AlwaysExecuteW<VuCtl0Spec> {
        AlwaysExecuteW::new(self, 0)
    }
}
#[doc = "Vector unit control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`vu_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vu_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VuCtl0Spec;
impl crate::RegisterSpec for VuCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vu_ctl0::R`](R) reader structure"]
impl crate::Readable for VuCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`vu_ctl0::W`](W) writer structure"]
impl crate::Writable for VuCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VU_CTL0 to value 0"]
impl crate::Resettable for VuCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
