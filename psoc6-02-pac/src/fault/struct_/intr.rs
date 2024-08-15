#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `FAULT` reader - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to this field to clear the interrupt cause to '0'. SW clear STATUS.VALID to '0' to enable capture of the next fault. Note that when there is an enabled pending fault source, the pending fault source is captured immediately and INTR.FAULT is immediately activated (set to '1')."]
    #[inline(always)]
    #[must_use]
    pub fn fault(&mut self) -> FaultW<IntrSpec> {
        FaultW::new(self, 0)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
