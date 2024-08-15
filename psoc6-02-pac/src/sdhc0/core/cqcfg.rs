#[doc = "Register `CQCFG` reader"]
pub type R = crate::R<CqcfgSpec>;
#[doc = "Register `CQCFG` writer"]
pub type W = crate::W<CqcfgSpec>;
#[doc = "Field `CQ_EN` reader - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CqEnR = crate::BitReader;
#[doc = "Field `CQ_EN` writer - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
pub type CqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_GENERAL_EN` reader - N/A"]
pub type CrGeneralEnR = crate::BitReader;
#[doc = "Field `CR_GENERAL_EN` writer - N/A"]
pub type CrGeneralEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASK_DESC_SIZE` reader - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TaskDescSizeR = crate::BitReader;
#[doc = "Field `TASK_DESC_SIZE` writer - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
pub type TaskDescSizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMD_EN` reader - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DcmdEnR = crate::BitReader;
#[doc = "Field `DCMD_EN` writer - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
pub type DcmdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    pub fn cq_en(&self) -> CqEnR {
        CqEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn cr_general_en(&self) -> CrGeneralEnR {
        CrGeneralEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    pub fn task_desc_size(&self) -> TaskDescSizeR {
        TaskDescSizeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    pub fn dcmd_en(&self) -> DcmdEnR {
        DcmdEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable command queuing engine (CQE). When CQE is disable, the software controls the eMMC bus using the registers between the addresses 0x000 to 0x1FF. Before the software writes to this bit, the software verifies that the eMMC host controller is in idle state and there are no ongoing commands or data transfers. When software wants to exit command queuing mode, it clears all previous tasks (if any) before setting this bit to 0. Values: - 0x1 (CQE_ENABLE): Enable command queuing - 0x0 (CQE_DISABLE): Disable command queuing"]
    #[inline(always)]
    #[must_use]
    pub fn cq_en(&mut self) -> CqEnW<CqcfgSpec> {
        CqEnW::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cr_general_en(&mut self) -> CrGeneralEnW<CqcfgSpec> {
        CrGeneralEnW::new(self, 1)
    }
    #[doc = "Bit 8 - Bit Value Description This bit indicates the size of task descriptor used in host memory. This bit can only be configured when Command Queuing Enable bit is 0 (command queuing is disabled). Values: - 0x1 (TASK_DESC_128b): Task descriptor size is 128 bits - 0x0 (TASK_DESC_64b): Task descriptor size is 64 bits"]
    #[inline(always)]
    #[must_use]
    pub fn task_desc_size(&mut self) -> TaskDescSizeW<CqcfgSpec> {
        TaskDescSizeW::new(self, 8)
    }
    #[doc = "Bit 12 - This bit indicates to the hardware whether the Task Descriptor in slot #31 of the TDL is a data transfer descriptor or a direct-command descriptor. CQE uses this bit when a task is issued in slot #31, to determine how to decode the Task Descriptor. Values: - 0x1 (SLOT31_DCMD_ENABLE): Task descriptor in slot #31 is a DCMD Task Descriptor - 0x0 (SLOT31_DCMD_DISABLE): Task descriptor in slot #31 is a data Transfer Task Descriptor"]
    #[inline(always)]
    #[must_use]
    pub fn dcmd_en(&mut self) -> DcmdEnW<CqcfgSpec> {
        DcmdEnW::new(self, 12)
    }
}
#[doc = "Command Queuing Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqcfgSpec;
impl crate::RegisterSpec for CqcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqcfg::R`](R) reader structure"]
impl crate::Readable for CqcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cqcfg::W`](W) writer structure"]
impl crate::Writable for CqcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQCFG to value 0"]
impl crate::Resettable for CqcfgSpec {
    const RESET_VALUE: u32 = 0;
}
