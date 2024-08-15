#[doc = "Register `HOST_FRAME` reader"]
pub type R = crate::R<HostFrameSpec>;
#[doc = "Register `HOST_FRAME` writer"]
pub type W = crate::W<HostFrameSpec>;
#[doc = "Field `FRAME` reader - These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FrameR = crate::FieldReader<u16>;
#[doc = "Field `FRAME` writer - These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
pub type FrameW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - These bits are used to specify a frame number of SOF. Notes: - This bit isn't reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'. - Specify a frame number in this register before setting SOF in the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). - This register cannot be written while the SOFBUSY bit of the Host Status Register (HOST_STATUS) is '1' and a SOF token is in process."]
    #[inline(always)]
    #[must_use]
    pub fn frame(&mut self) -> FrameW<HostFrameSpec> {
        FrameW::new(self, 0)
    }
}
#[doc = "Host Frame Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostFrameSpec;
impl crate::RegisterSpec for HostFrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_frame::R`](R) reader structure"]
impl crate::Readable for HostFrameSpec {}
#[doc = "`write(|w| ..)` method takes [`host_frame::W`](W) writer structure"]
impl crate::Writable for HostFrameSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_FRAME to value 0"]
impl crate::Resettable for HostFrameSpec {
    const RESET_VALUE: u32 = 0;
}
