#[doc = "Register `HOST_FCOMP` reader"]
pub type R = crate::R<HostFcompSpec>;
#[doc = "Register `HOST_FCOMP` writer"]
pub type W = crate::W<HostFcompSpec>;
#[doc = "Field `FRAMECOMP` reader - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FramecompR = crate::FieldReader;
#[doc = "Field `FRAMECOMP` writer - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type FramecompW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn framecomp(&self) -> FramecompR {
        FramecompR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are used to specify the data to be compared with the low-order eight bits of a frame number when sending a SOF token. If the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '0', the frame number of SOF is compared with the value of this register when sending a SOF token. If they match, the SOFIRQ bit of the Interrupt USB Host Register (INTR_USBHOST) is set to '1'. The setting of this register is invalid when the SOFSTEP bit of Host Control 2 Register (HOST_CTL2) is '1'. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn framecomp(&mut self) -> FramecompW<HostFcompSpec> {
        FramecompW::new(self, 0)
    }
}
#[doc = "Host SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_fcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_fcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostFcompSpec;
impl crate::RegisterSpec for HostFcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_fcomp::R`](R) reader structure"]
impl crate::Readable for HostFcompSpec {}
#[doc = "`write(|w| ..)` method takes [`host_fcomp::W`](W) writer structure"]
impl crate::Writable for HostFcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_FCOMP to value 0"]
impl crate::Resettable for HostFcompSpec {
    const RESET_VALUE: u32 = 0;
}
