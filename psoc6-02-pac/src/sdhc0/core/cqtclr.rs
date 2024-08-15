#[doc = "Register `CQTCLR` reader"]
pub type R = crate::R<CqtclrSpec>;
#[doc = "Register `CQTCLR` writer"]
pub type W = crate::W<CqtclrSpec>;
#[doc = "Field `TCLR` reader - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
pub type TclrR = crate::FieldReader<u32>;
#[doc = "Field `TCLR` writer - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
pub type TclrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
    #[inline(always)]
    pub fn tclr(&self) -> TclrR {
        TclrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Writing 1 to bit n of this register orders CQE to clear a task that the software has previously issued. This bit can only be written when CQE is in Halt state as indicated in CQCFG register Halt bit. When software writes 1 to a bit in this register, CQE updates the value to 1, and starts clearing the data structures related to the task. CQE clears the bit fields (sets a value of 0) in CQTCLR and in CQTDBR once the clear operation is complete. Software must poll on the CQTCLR until it is cleared to verify that a clear operation was done."]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TclrW<CqtclrSpec> {
        TclrW::new(self, 0)
    }
}
#[doc = "Command Queuing DoorBell register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqtclrSpec;
impl crate::RegisterSpec for CqtclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtclr::R`](R) reader structure"]
impl crate::Readable for CqtclrSpec {}
#[doc = "`write(|w| ..)` method takes [`cqtclr::W`](W) writer structure"]
impl crate::Writable for CqtclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQTCLR to value 0"]
impl crate::Resettable for CqtclrSpec {
    const RESET_VALUE: u32 = 0;
}
