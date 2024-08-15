#[doc = "Register `INSTR_FF_STATUS` reader"]
pub type R = crate::R<InstrFfStatusSpec>;
#[doc = "Field `USED` reader - Number of instructions in the instruction FIFO. The value of this field ranges from 0 to 8."]
pub type UsedR = crate::FieldReader;
#[doc = "Field `EVENT` reader - Instruction FIFO event."]
pub type EventR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of instructions in the instruction FIFO. The value of this field ranges from 0 to 8."]
    #[inline(always)]
    pub fn used(&self) -> UsedR {
        UsedR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Instruction FIFO event."]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Instruction FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`instr_ff_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrFfStatusSpec;
impl crate::RegisterSpec for InstrFfStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`instr_ff_status::R`](R) reader structure"]
impl crate::Readable for InstrFfStatusSpec {}
#[doc = "`reset()` method sets INSTR_FF_STATUS to value 0"]
impl crate::Resettable for InstrFfStatusSpec {
    const RESET_VALUE: u32 = 0;
}
