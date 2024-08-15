#[doc = "Register `INSTR_FF_CTL` reader"]
pub type R = crate::R<InstrFfCtlSpec>;
#[doc = "Register `INSTR_FF_CTL` writer"]
pub type W = crate::W<InstrFfCtlSpec>;
#[doc = "Field `EVENT_LEVEL` reader - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
pub type EventLevelR = crate::FieldReader;
#[doc = "Field `EVENT_LEVEL` writer - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
pub type EventLevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CLEAR` reader - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
pub type ClearR = crate::BitReader;
#[doc = "Field `CLEAR` writer - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK` reader - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
pub type BlockR = crate::BitReader;
#[doc = "Field `BLOCK` writer - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
pub type BlockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
    #[inline(always)]
    pub fn event_level(&self) -> EventLevelR {
        EventLevelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 16 - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Event level. When the number of entries in the instruction FIFO is less than the amount of this field, an event is generated: - 'event' = INSTR_FF_STATUS.USED &lt; EVENT_LEVEL."]
    #[inline(always)]
    #[must_use]
    pub fn event_level(&mut self) -> EventLevelW<InstrFfCtlSpec> {
        EventLevelW::new(self, 0)
    }
    #[doc = "Bit 16 - When '1', the instruction FIFO is cleared/invalidated. Invalidation will last for as long as this field is '1'. If a quick clear/invalidation is required, the field should be set to '1' and be followed by a set to '0'. If a clear/invalidation is required for an extended time period, the field should be set to '1' during the complete time period. HW sets this field to '1' on when a INSTR_OPC_ERROR, INSTR_CC_ERROR or BUS_ERROR interrupt cause is activated."]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<InstrFfCtlSpec> {
        ClearW::new(self, 16)
    }
    #[doc = "Bit 17 - This field specifies the behavior when an instruction is written to a full FIFO (INSTR_FIFO_WR MMIO register): '0': The write is ignored/dropped and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1'. '1': The write is blocked, resulting in AHB-Lite wait states and the INTR.INSTR_FF_OVERFLOW interrupt cause is set to '1' (this cause may be masked out). The instruction is written to the FIFO as soon as a FIFO entry becomes available. The maximum time is roughly the time of the execution of the slowest/longest instruction. Note that this setting may 'lock up' /stall the CPU. When the CPU is 'locked up'/stalled it can not respond to any system interrupts. As a result, the interrupt latency is increased. Note that this may not be an issue if the associated CPU is only performing cryptography functionality, e.g. the CM0+ during boot time."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BlockW<InstrFfCtlSpec> {
        BlockW::new(self, 17)
    }
}
#[doc = "Instruction FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`instr_ff_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr_ff_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrFfCtlSpec;
impl crate::RegisterSpec for InstrFfCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_ff_ctl::R`](R) reader structure"]
impl crate::Readable for InstrFfCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`instr_ff_ctl::W`](W) writer structure"]
impl crate::Writable for InstrFfCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTR_FF_CTL to value 0x0002_0000"]
impl crate::Resettable for InstrFfCtlSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
