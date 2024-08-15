#[doc = "Register `CQTDBR` reader"]
pub type R = crate::R<CqtdbrSpec>;
#[doc = "Register `CQTDBR` writer"]
pub type W = crate::W<CqtdbrSpec>;
#[doc = "Field `DBR` reader - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: - A task execution is completed (with success or error). - The task is cleared using CQTCLR register. - All tasks are cleared using CQCTL register. - CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
pub type DbrR = crate::FieldReader<u32>;
#[doc = "Field `DBR` writer - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: - A task execution is completed (with success or error). - The task is cleared using CQTCLR register. - All tasks are cleared using CQCTL register. - CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
pub type DbrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: - A task execution is completed (with success or error). - The task is cleared using CQTCLR register. - All tasks are cleared using CQCTL register. - CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
    #[inline(always)]
    pub fn dbr(&self) -> DbrR {
        DbrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The software configures TDLBA and TDLBAU, and enable CQE in CQCFG before using this register. Writing 1 to bit n of this register triggers CQE to start processing the task encoded in slot n of the TDL. Writing 0 by the software does not have any impact on the hardware, and does not change the value of the register bit. CQE always processes tasks according to the order submitted to the list by CQTDBR write transactions. CQE processes Data Transfer tasks by reading the Task Descriptor and sending QUEUED_TASK_PARAMS (CMD44) and QUEUED_TASK_ADDRESS (CMD45) commands to the device. CQE processes DCMD tasks (in slot #31, when enabled) by reading the Task Descriptor, and generating the command encoded by its index and argument. The corresponding bit is cleared to 0 by CQE in one of the following events: - A task execution is completed (with success or error). - The task is cleared using CQTCLR register. - All tasks are cleared using CQCTL register. - CQE is disabled using CQCFG register. Software may initiate multiple tasks at the same time (batch submission) by writing 1 to multiple bits of this register in the same transaction. In the case of batch submission, CQE processes the tasks in order of the task index, starting with the lowest index. If one or more tasks in the batch are marked with QBR, the ordering of execution is based on said processing order."]
    #[inline(always)]
    #[must_use]
    pub fn dbr(&mut self) -> DbrW<CqtdbrSpec> {
        DbrW::new(self, 0)
    }
}
#[doc = "Command Queuing DoorBell register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqtdbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqtdbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqtdbrSpec;
impl crate::RegisterSpec for CqtdbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqtdbr::R`](R) reader structure"]
impl crate::Readable for CqtdbrSpec {}
#[doc = "`write(|w| ..)` method takes [`cqtdbr::W`](W) writer structure"]
impl crate::Writable for CqtdbrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQTDBR to value 0"]
impl crate::Resettable for CqtdbrSpec {
    const RESET_VALUE: u32 = 0;
}
