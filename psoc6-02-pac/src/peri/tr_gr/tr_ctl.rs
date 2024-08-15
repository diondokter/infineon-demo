#[doc = "Register `TR_CTL[%s]` reader"]
pub type R = crate::R<TrCtlSpec>;
#[doc = "Register `TR_CTL[%s]` writer"]
pub type W = crate::W<TrCtlSpec>;
#[doc = "Field `TR_SEL` reader - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type TrSelR = crate::FieldReader;
#[doc = "Field `TR_SEL` writer - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
pub type TrSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TR_INV` reader - Specifies if the output trigger is inverted."]
pub type TrInvR = crate::BitReader;
#[doc = "Field `TR_INV` writer - Specifies if the output trigger is inverted."]
pub type TrInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_EDGE` reader - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TrEdgeR = crate::BitReader;
#[doc = "Field `TR_EDGE` writer - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
pub type TrEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_FREEZE_EN` reader - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DbgFreezeEnR = crate::BitReader;
#[doc = "Field `DBG_FREEZE_EN` writer - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
pub type DbgFreezeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TrSelR {
        TrSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    pub fn tr_inv(&self) -> TrInvR {
        TrInvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TrEdgeR {
        TrEdgeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DbgFreezeEnR {
        DbgFreezeEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies input trigger. This field is typically set during the setup of a chip use case scenario. Changing this field while activated triggers are present on the input triggers may result in unpredictable behavior. Note that input trigger 0 (default value) is typically connected to a constant signal level of '0', and as a result will not cause HW activation of the output trigger."]
    #[inline(always)]
    #[must_use]
    pub fn tr_sel(&mut self) -> TrSelW<TrCtlSpec> {
        TrSelW::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies if the output trigger is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tr_inv(&mut self) -> TrInvW<TrCtlSpec> {
        TrInvW::new(self, 8)
    }
    #[doc = "Bit 9 - Specifies if the (inverted) output trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. '1': edge sensitive trigger. The (inverted) output trigger duration needs to be at least 2 cycles on the consumer clock. the(inverted) output trigger is synchronized to the consumer clock and a two cycle pulse is generated on the consumer clock."]
    #[inline(always)]
    #[must_use]
    pub fn tr_edge(&mut self) -> TrEdgeW<TrCtlSpec> {
        TrEdgeW::new(self, 9)
    }
    #[doc = "Bit 12 - Specifies if the output trigger is blocked in debug mode. When set high tr_dbg_freeze will block the output trigger generation."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DbgFreezeEnW<TrCtlSpec> {
        DbgFreezeEnW::new(self, 12)
    }
}
#[doc = "Trigger control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtlSpec;
impl crate::RegisterSpec for TrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl::R`](R) reader structure"]
impl crate::Readable for TrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl::W`](W) writer structure"]
impl crate::Writable for TrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTL[%s]
to value 0"]
impl crate::Resettable for TrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
