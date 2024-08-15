#[doc = "Register `HOST_CTL2` reader"]
pub type R = crate::R<HostCtl2Spec>;
#[doc = "Register `HOST_CTL2` writer"]
pub type W = crate::W<HostCtl2Spec>;
#[doc = "Field `RETRY` reader - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RetryR = crate::BitReader;
#[doc = "Field `RETRY` writer - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RetryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL` reader - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
pub type CancelR = crate::BitReader;
#[doc = "Field `CANCEL` writer - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFSTEP` reader - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
pub type SofstepR = crate::BitReader;
#[doc = "Field `SOFSTEP` writer - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
pub type SofstepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIVE` reader - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
pub type AliveR = crate::BitReader;
#[doc = "Field `ALIVE` writer - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
pub type AliveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_4` reader - N/A"]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `RSVD_4` writer - N/A"]
pub type Rsvd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_5` reader - N/A"]
pub type Rsvd5R = crate::BitReader;
#[doc = "Field `RSVD_5` writer - N/A"]
pub type Rsvd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTEST` reader - N/A"]
pub type TtestR = crate::FieldReader;
#[doc = "Field `TTEST` writer - N/A"]
pub type TtestW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn retry(&self) -> RetryR {
        RetryR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    pub fn cancel(&self) -> CancelR {
        CancelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    pub fn sofstep(&self) -> SofstepR {
        SofstepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    pub fn alive(&self) -> AliveR {
        AliveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    pub fn rsvd_4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5(&self) -> Rsvd5R {
        Rsvd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    pub fn ttest(&self) -> TtestR {
        TtestR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to '1', the target token is retried if a NAK or error* occurs. Retry processing is performed after the time that is specified in the Host Retry Timer Setup Register (HOST_RTIMER). * : HOST_ERR.RERR='1', HOST_ERR.TOUT='1', HOST_ERR.CRC='1', HOST_ERR.TGERR='1', HOST_ERR.STUFF='1' '0' : Doesn't retry token sending. '1' : Retries token sending Note: - This bit isn't initialized even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RetryW<HostCtl2Spec> {
        RetryW::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is set to '1', if the target token is written to the Host Token Endpoint Register (HOST_TOKEN) in the EOF area (specified in the Host EOF Setup Register), its sending is canceled. When this bit is set to '0', token sending is not canceled even if the target token is written to the register. The cancellation of token sending is detected by reading the TCAN bit of the Interrupt USB Host Register (INTR_USBHOST). '0' : Continues a token. '1' : Cancels a token."]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CancelW<HostCtl2Spec> {
        CancelW::new(self, 1)
    }
    #[doc = "Bit 2 - If this bit is set to '1', the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1' each time SOF is sent. If this bit is set to '0', the set value of the Host SOF Interrupt Frame Compare Register (HOST_FCOMP) is compared with the low-order eight bits of the SOF frame number. If they match, the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is set to '1'. '0' : An interrupt occurred due to the HOST_HFCOMP setting. '1' : An interrupt occurred. Notes: - If a SOF token (TKNEN='001') is sent by the setting of the Host Token Endpoint Register (HOST_TOKEN), the SOF interrupt flag (INTR_USBHOST.SOFIRQ) is not set to '1' regardless of the setting of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn sofstep(&mut self) -> SofstepW<HostCtl2Spec> {
        SofstepW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to specify the keep-alive function in the low-speed mode. If this bit it set to '1' while the CLKSEL bit of the Host Control 1 Register (HOST_CTL1) is '0', SE0 is output instead of SOF. This bit is only effective when the CLKSEL bit is '0'. If the CLKSEL bit is '1' (Full-Speed mode), SOF is output regardless of the setting of the ALIVE bit. '0' : SOF output. '1' : SE0 output (Keep alive)"]
    #[inline(always)]
    #[must_use]
    pub fn alive(&mut self) -> AliveW<HostCtl2Spec> {
        AliveW::new(self, 3)
    }
    #[doc = "Bit 4 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_4(&mut self) -> Rsvd4W<HostCtl2Spec> {
        Rsvd4W::new(self, 4)
    }
    #[doc = "Bit 5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd_5(&mut self) -> Rsvd5W<HostCtl2Spec> {
        Rsvd5W::new(self, 5)
    }
    #[doc = "Bits 6:7 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ttest(&mut self) -> TtestW<HostCtl2Spec> {
        TtestW::new(self, 6)
    }
}
#[doc = "Host Control 2 Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostCtl2Spec;
impl crate::RegisterSpec for HostCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_ctl2::R`](R) reader structure"]
impl crate::Readable for HostCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`host_ctl2::W`](W) writer structure"]
impl crate::Writable for HostCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_CTL2 to value 0x01"]
impl crate::Resettable for HostCtl2Spec {
    const RESET_VALUE: u32 = 0x01;
}
