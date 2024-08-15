#[doc = "Register `INSTR_FF_WR` writer"]
pub type W = crate::W<InstrFfWrSpec>;
#[doc = "Field `DATA32` writer - Instruction or instruction operand data that is written to the instruction FIFO."]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Instruction or instruction operand data that is written to the instruction FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<InstrFfWrSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "Instruction FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`instr_ff_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InstrFfWrSpec;
impl crate::RegisterSpec for InstrFfWrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`instr_ff_wr::W`](W) writer structure"]
impl crate::Writable for InstrFfWrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INSTR_FF_WR to value 0"]
impl crate::Resettable for InstrFfWrSpec {
    const RESET_VALUE: u32 = 0;
}
