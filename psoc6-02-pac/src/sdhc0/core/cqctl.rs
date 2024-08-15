#[doc = "Register `CQCTL` reader"]
pub type R = crate::R<CqctlSpec>;
#[doc = "Register `CQCTL` writer"]
pub type W = crate::W<CqctlSpec>;
#[doc = "Field `HALT` reader - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
pub type HaltR = crate::BitReader;
#[doc = "Field `HALT` writer - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ALL_TASKS` reader - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type ClrAllTasksR = crate::BitReader;
#[doc = "Field `CLR_ALL_TASKS` writer - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
pub type ClrAllTasksW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    pub fn clr_all_tasks(&self) -> ClrAllTasksR {
        ClrAllTasksR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt request and resume Values: - 0x1 (HALT_CQE): Software writes 1 to this bit when it wants to acquire software control over the eMMC bus and to disable CQE from issuing command on the bus. For example, issuing a Discard Task command (CMDQ_TASK_MGMT). When the software writes 1, CQE completes the ongoing task (if any in progress). After the task is completed and the CQE is in idle state, CQE does not issue new commands and indicates to the software by setting this bit to 1. The software can poll on this bit until it is set to 1 and only then send commands on the eMMC bus. - 0x0 (RESUME_CQE): Software writes 0 to this bit to exit from the halt state and resume CQE activity."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<CqctlSpec> {
        HaltW::new(self, 0)
    }
    #[doc = "Bit 8 - Clear all tasks This bit can only be written when the controller is halted. This bit does not clear tasks in the device. The software has to use the CMDQ_TASK_MGMT command to clear device's queue. Values: - 0x1 (CLEAR_ALL_TASKS): Clears all the tasks in the controller - 0x0 (NO_EFFECT): Programming 0 has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn clr_all_tasks(&mut self) -> ClrAllTasksW<CqctlSpec> {
        ClrAllTasksW::new(self, 8)
    }
}
#[doc = "Command Queuing Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqctlSpec;
impl crate::RegisterSpec for CqctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqctl::R`](R) reader structure"]
impl crate::Readable for CqctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cqctl::W`](W) writer structure"]
impl crate::Writable for CqctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCTL to value 0"]
impl crate::Resettable for CqctlSpec {
    const RESET_VALUE: u32 = 0;
}
