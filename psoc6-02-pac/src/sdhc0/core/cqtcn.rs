#[doc = "Register `CQTCN` reader"]
pub type R = crate::R<CqtcnSpec>;
#[doc = "Register `CQTCN` writer"]
pub type W = crate::W<CqtcnSpec>;
#[doc = "Field `TCN` reader - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TcnR = crate::FieldReader<u32>;
#[doc = "Field `TCN` writer - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
pub type TcnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    pub fn tcn(&self) -> TcnR {
        TcnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Task Completion Notification Each of the 32 bits are bit mapped to the 32 tasks. - Bit-N(1): Task-N has completed execution (with success or errors) - Bit-N(0): Task-N has not completed, could be pending or not submitted. On task completion, software may read this register to know tasks that have completed. After reading this register, software may clear the relevant bit fields by writing 1 to the corresponding bits."]
    #[inline(always)]
    #[must_use]
    pub fn tcn(&mut self) -> TcnW<CqtcnSpec> {
        TcnW::new(self, 0)
    }
}
#[doc = "Command Queuing TaskClear Notification register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtcn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtcn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqtcnSpec;
impl crate::RegisterSpec for CqtcnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtcn::R`](R) reader structure"]
impl crate::Readable for CqtcnSpec {}
#[doc = "`write(|w| ..)` method takes [`cqtcn::W`](W) writer structure"]
impl crate::Writable for CqtcnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQTCN to value 0"]
impl crate::Resettable for CqtcnSpec {
    const RESET_VALUE: u32 = 0;
}
