#[doc = "Register `HOST_RTIMER` reader"]
pub type R = crate::R<HostRtimerSpec>;
#[doc = "Register `HOST_RTIMER` writer"]
pub type W = crate::W<HostRtimerSpec>;
#[doc = "Field `RTIMER` reader - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RtimerR = crate::FieldReader<u32>;
#[doc = "Field `RTIMER` writer - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
pub type RtimerW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    pub fn rtimer(&self) -> RtimerR {
        RtimerR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - These bits are used to specify the retry time in this register. The retry timer is activated when token sending starts while the RETRY bit of Host Control 2 Register (HOST_CTL2) is '1'. The retry time is then decremented by one when a 1-bit transfer clock (12 MHz in the full-speed mode) is output. When the retry timer reaches 0, the target token is sent, and processing ends. If a token retry occurs in the EOF area, the retry timer is stopped until SOF sending is ended. After SOF sending has been completed, the retry timer restarts with the value that is set when the timer stopped."]
    #[inline(always)]
    #[must_use]
    pub fn rtimer(&mut self) -> RtimerW<HostRtimerSpec> {
        RtimerW::new(self, 0)
    }
}
#[doc = "Host Retry Timer Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_rtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_rtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostRtimerSpec;
impl crate::RegisterSpec for HostRtimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_rtimer::R`](R) reader structure"]
impl crate::Readable for HostRtimerSpec {}
#[doc = "`write(|w| ..)` method takes [`host_rtimer::W`](W) writer structure"]
impl crate::Writable for HostRtimerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_RTIMER to value 0"]
impl crate::Resettable for HostRtimerSpec {
    const RESET_VALUE: u32 = 0;
}
