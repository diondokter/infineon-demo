#[doc = "Register `CQIS` reader"]
pub type R = crate::R<CqisSpec>;
#[doc = "Register `CQIS` writer"]
pub type W = crate::W<CqisSpec>;
#[doc = "Field `HAC` reader - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HacR = crate::BitReader;
#[doc = "Field `HAC` writer - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
pub type HacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC` reader - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
pub type TccR = crate::BitReader;
#[doc = "Field `TCC` writer - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED` reader - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
pub type RedR = crate::BitReader;
#[doc = "Field `RED` writer - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
pub type RedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL` reader - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TclR = crate::BitReader;
#[doc = "Field `TCL` writer - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
pub type TclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCE` reader - N/A"]
pub type GceR = crate::BitReader;
#[doc = "Field `GCE` writer - N/A"]
pub type GceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCE` reader - N/A"]
pub type IcceR = crate::BitReader;
#[doc = "Field `ICCE` writer - N/A"]
pub type IcceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    pub fn hac(&self) -> HacR {
        HacR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn gce(&self) -> GceR {
        GceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn icce(&self) -> IcceR {
        IcceR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt This status bit is asserted (only if CQISE.HAC_STE=1) when halt bit in the CQCTL register transitions from 0 to 1 indicating that the host controller has completed its current ongoing task and has entered halt state. A value of 1 clears this status bit. Values: - 0x1 (SET): HAC Interrupt is set - 0x0 (NOTSET): HAC Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn hac(&mut self) -> HacW<CqisSpec> {
        HacW::new(self, 0)
    }
    #[doc = "Bit 1 - Task complete interrupt This status bit is asserted (if CQISE.TCC_STE=1) when at least one of the following conditions are met: - A task is completed and the INT bit is set in its Task Descriptor - Interrupt caused by Interrupt Coalescing logic due to timeout - Interrupt Coalescing logic reached the configured threshold A value of 1 clears this status bit Values: - 0x1 (SET): TCC Interrupt is set - 0x0 (NOTSET): TCC Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn tcc(&mut self) -> TccW<CqisSpec> {
        TccW::new(self, 1)
    }
    #[doc = "Bit 2 - Response error detected interrupt This status bit is asserted (if CQISE.RED_STE=1) when a response is received with an error bit set in the device status field. Configure the CQRMEM register to identify device status bit fields that may trigger an interrupt and that are masked. A value of 1 clears this status bit. Values: - 0x1 (SET): RED Interrupt is set - 0x0 (NOTSET): RED Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RedW<CqisSpec> {
        RedW::new(self, 2)
    }
    #[doc = "Bit 3 - Task cleared interrupt This status bit is asserted (if CQISE.TCL_STE=1) when a task clear operation is completed by CQE. The completed task clear operation is either an individual task clear (by writing CQTCLR) or clearing of all tasks (by writing CQCTL). A value of 1 clears this status bit. Values: - 0x1 (SET): TCL Interrupt is set - 0x0 (NOTSET): TCL Interrupt is not set"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TclW<CqisSpec> {
        TclW::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn gce(&mut self) -> GceW<CqisSpec> {
        GceW::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn icce(&mut self) -> IcceW<CqisSpec> {
        IcceW::new(self, 5)
    }
}
#[doc = "Command Queuing Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqisSpec;
impl crate::RegisterSpec for CqisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqis::R`](R) reader structure"]
impl crate::Readable for CqisSpec {}
#[doc = "`write(|w| ..)` method takes [`cqis::W`](W) writer structure"]
impl crate::Writable for CqisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQIS to value 0"]
impl crate::Resettable for CqisSpec {
    const RESET_VALUE: u32 = 0;
}
