#[doc = "Register `CM0_SYSTEM_INT_CTL[%s]` reader"]
pub type R = crate::R<Cm0SystemIntCtlSpec>;
#[doc = "Register `CM0_SYSTEM_INT_CTL[%s]` writer"]
pub type W = crate::W<Cm0SystemIntCtlSpec>;
#[doc = "Field `CPU_INT_IDX` reader - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
pub type CpuIntIdxR = crate::FieldReader;
#[doc = "Field `CPU_INT_IDX` writer - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
pub type CpuIntIdxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPU_INT_VALID` reader - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
pub type CpuIntValidR = crate::BitReader;
#[doc = "Field `CPU_INT_VALID` writer - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
pub type CpuIntValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CpuIntIdxR {
        CpuIntIdxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CpuIntValidR {
        CpuIntValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU interrupt index (legal range \\[0, 7\\]). This field specifies to which CPU interrupt the system interrupt is mapped. E.g., if CPU_INT_IDX is '6', the system interrupt is mapped to CPU interrupt '6'. Note: it is possible to map multiple system interrupts to the same CPU interrupt. It is advised to assign different priorities to the CPU interrupts and to assign system interrupts to CPU interrupts accordingly."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_idx(&mut self) -> CpuIntIdxW<Cm0SystemIntCtlSpec> {
        CpuIntIdxW::new(self, 0)
    }
    #[doc = "Bit 31 - Interrupt enable: '0': Disabled. The system interrupt will NOT be mapped to any CPU interrupt. '1': Enabled. The system interrupt is mapped on CPU interrupt CPU_INT_IDX. Note: the CPUs have dedicated XXX_SYSTEM_INT_CTL registers. In other words, the CPUs can use different CPU interrupts for the same system interrupt. However, typically only one of the CPUs will have the ENABLED field of a specific system interrupt set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_valid(&mut self) -> CpuIntValidW<Cm0SystemIntCtlSpec> {
        CpuIntValidW::new(self, 31)
    }
}
#[doc = "CM0+ system interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_system_int_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_system_int_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0SystemIntCtlSpec;
impl crate::RegisterSpec for Cm0SystemIntCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_system_int_ctl::R`](R) reader structure"]
impl crate::Readable for Cm0SystemIntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_system_int_ctl::W`](W) writer structure"]
impl crate::Writable for Cm0SystemIntCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_SYSTEM_INT_CTL[%s]
to value 0"]
impl crate::Resettable for Cm0SystemIntCtlSpec {
    const RESET_VALUE: u32 = 0;
}
