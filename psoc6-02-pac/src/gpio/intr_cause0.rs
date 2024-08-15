#[doc = "Register `INTR_CAUSE0` reader"]
pub type R = crate::R<IntrCause0Spec>;
#[doc = "Field `PORT_INT` reader - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PRT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
pub type PortIntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each IO port has an associated bit field in this register. The bit field reflects the IO port's interrupt line (bit field i reflects 'gpio_interrupts\\[i\\]' for IO port i). The register is used when the system uses a combined interrupt line 'gpio_interrupt'. The software ISR reads the register to determine which IO port(s) is responsible for the combined interrupt line. Once, the IO port(s) is determined, the IO port's GPIO_PRT_INTR register is read to determine the IO pin(s) in the IO port that caused the interrupt. '0': Port has no pending interrupt '1': Port has pending interrupt"]
    #[inline(always)]
    pub fn port_int(&self) -> PortIntR {
        PortIntR::new(self.bits)
    }
}
#[doc = "Interrupt port cause register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrCause0Spec;
impl crate::RegisterSpec for IntrCause0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_cause0::R`](R) reader structure"]
impl crate::Readable for IntrCause0Spec {}
#[doc = "`reset()` method sets INTR_CAUSE0 to value 0"]
impl crate::Resettable for IntrCause0Spec {
    const RESET_VALUE: u32 = 0;
}
