#[doc = "Register `TR_CMD` reader"]
pub type R = crate::R<TrCmdSpec>;
#[doc = "Register `TR_CMD` writer"]
pub type W = crate::W<TrCmdSpec>;
#[doc = "Field `TR_SEL` reader - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
pub type TrSelR = crate::FieldReader;
#[doc = "Field `TR_SEL` writer - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
pub type TrSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GROUP_SEL` reader - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
pub type GroupSelR = crate::FieldReader;
#[doc = "Field `GROUP_SEL` writer - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
pub type GroupSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TR_EDGE` reader - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
pub type TrEdgeR = crate::BitReader;
#[doc = "Field `TR_EDGE` writer - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
pub type TrEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_SEL` reader - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
pub type OutSelR = crate::BitReader;
#[doc = "Field `OUT_SEL` writer - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
pub type OutSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVATE` reader - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields. SW MUST NOT set ACTIVATE bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set ACTIVATE to '1' with a new register write."]
pub type ActivateR = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields. SW MUST NOT set ACTIVATE bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set ACTIVATE to '1' with a new register write."]
pub type ActivateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    pub fn tr_sel(&self) -> TrSelR {
        TrSelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn group_sel(&self) -> GroupSelR {
        GroupSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    pub fn tr_edge(&self) -> TrEdgeR {
        TrEdgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    pub fn out_sel(&self) -> OutSelR {
        OutSelR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields. SW MUST NOT set ACTIVATE bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set ACTIVATE to '1' with a new register write."]
    #[inline(always)]
    pub fn activate(&self) -> ActivateR {
        ActivateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the activated trigger when ACTIVATE is '1'. If the specified trigger is not present, the trigger activation has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn tr_sel(&mut self) -> TrSelW<TrCmdSpec> {
        TrSelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Specifies the trigger group: '0'-'15': trigger multiplexer groups. '16'-'31': trigger 1-to-1 groups."]
    #[inline(always)]
    #[must_use]
    pub fn group_sel(&mut self) -> GroupSelW<TrCmdSpec> {
        GroupSelW::new(self, 8)
    }
    #[doc = "Bit 29 - Specifies if the activated trigger is treated as a level sensitive or edge sensitive trigger. '0': level sensitive. The trigger reflects TR_CMD.ACTIVATE. '1': edge sensitive trigger. The trigger is activated for two clk_peri cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tr_edge(&mut self) -> TrEdgeW<TrCmdSpec> {
        TrEdgeW::new(self, 29)
    }
    #[doc = "Bit 30 - Specifies whether trigger activation is for a specific input or output trigger of the trigger multiplexer. Activation of a specific input trigger, will result in activation of all output triggers that have the specific input trigger selected through their TR_OUT_CTL.TR_SEL field. Activation of a specific output trigger, will result in activation of the specified TR_SEL output trigger only. '0': TR_SEL selection and trigger activation is for an input trigger to the trigger multiplexer. '1': TR_SEL selection and trigger activation is for an output trigger from the trigger multiplexer. Note: this field is not used for trigger 1-to-1 groups."]
    #[inline(always)]
    #[must_use]
    pub fn out_sel(&mut self) -> OutSelW<TrCmdSpec> {
        OutSelW::new(self, 30)
    }
    #[doc = "Bit 31 - SW sets this field to '1' to activate (set to '1') a trigger as identified by TR_SEL, TR_EDGE and OUT_SEL. HW sets this field to '0' for edge sensitive triggers AFTER the selected trigger is activated for two clk_peri cycles. Note: when ACTIVATE is '1', SW should not modify the other register fields. SW MUST NOT set ACTIVATE bit to '1' while updating the other register bits simultaneously. At first the SW MUST update the other register bits as needed, and then set ACTIVATE to '1' with a new register write."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ActivateW<TrCmdSpec> {
        ActivateW::new(self, 31)
    }
}
#[doc = "Trigger command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCmdSpec;
impl crate::RegisterSpec for TrCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_cmd::R`](R) reader structure"]
impl crate::Readable for TrCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_cmd::W`](W) writer structure"]
impl crate::Writable for TrCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TrCmdSpec {
    const RESET_VALUE: u32 = 0;
}
