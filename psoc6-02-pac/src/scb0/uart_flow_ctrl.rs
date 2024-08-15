#[doc = "Register `UART_FLOW_CTRL` reader"]
pub type R = crate::R<UartFlowCtrlSpec>;
#[doc = "Register `UART_FLOW_CTRL` writer"]
pub type W = crate::W<UartFlowCtrlSpec>;
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
pub type TriggerLevelR = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
pub type TriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RTS_POLARITY` reader - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
pub type RtsPolarityR = crate::BitReader;
#[doc = "Field `RTS_POLARITY` writer - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
pub type RtsPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_POLARITY` reader - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
pub type CtsPolarityR = crate::BitReader;
#[doc = "Field `CTS_POLARITY` writer - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
pub type CtsPolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_ENABLED` reader - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
pub type CtsEnabledR = crate::BitReader;
#[doc = "Field `CTS_ENABLED` writer - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
pub type CtsEnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TriggerLevelR {
        TriggerLevelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RtsPolarityR {
        RtsPolarityR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CtsPolarityR {
        CtsPolarityR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    pub fn cts_enabled(&self) -> CtsEnabledR {
        CtsEnabledR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trigger level. When the receiver FIFO has less entries than the amount of this field, a Ready To Send (RTS) output signal 'uart_rts_out' is activated. By setting this field to '0', flow control is effectively SW disabled (may be useful for debug purposes)."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TriggerLevelW<UartFlowCtrlSpec> {
        TriggerLevelW::new(self, 0)
    }
    #[doc = "Bit 16 - Polarity of the RTS output signal 'uart_rts_out': '0': RTS is low/'0' active; 'uart_rts_out' is '0' when active and 'uart_rts_out' is '1' when inactive. '1': RTS is high/'1' active; 'uart_rts_out' is '1' when active and 'uart_rts_out' is '0' when inactive. During IP reset (Hibernate system power mode), 'uart_rts_out' is '1'. This represents an inactive state assuming a low/'0' active polarity."]
    #[inline(always)]
    #[must_use]
    pub fn rts_polarity(&mut self) -> RtsPolarityW<UartFlowCtrlSpec> {
        RtsPolarityW::new(self, 16)
    }
    #[doc = "Bit 24 - Polarity of the CTS input signal 'uart_cts_in': '0': CTS is low/'0' active; 'uart_cts_in' is '0' when active and 'uart_cts_in' is '1' when inactive. '1': CTS is high/'1' active; 'uart_cts_in' is '1' when active and 'uart_cts_in' is '0' when inactive."]
    #[inline(always)]
    #[must_use]
    pub fn cts_polarity(&mut self) -> CtsPolarityW<UartFlowCtrlSpec> {
        CtsPolarityW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable use of CTS input signal 'uart_cts_in' by the UART transmitter: '0': Disabled. The UART transmitter ignores 'uart_cts_in', and transmits when a data frame is available for transmission in the TX FIFO or the TX shift register. '1': Enabled. The UART transmitter uses 'uart_cts_in' to qualify the transmission of data. It transmits when 'uart_cts_in' is active and a data frame is available for transmission in the TX FIFO or the TX shift register. If UART_CTRL.LOOPBACK is '1', 'uart_cts_in' is connected to 'uart_rts_out' in the IP (both signals are subjected to signal polarity changes as indicated by RTS_POLARITY and CTS_POLARITY)."]
    #[inline(always)]
    #[must_use]
    pub fn cts_enabled(&mut self) -> CtsEnabledW<UartFlowCtrlSpec> {
        CtsEnabledW::new(self, 25)
    }
}
#[doc = "UART flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_flow_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_flow_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartFlowCtrlSpec;
impl crate::RegisterSpec for UartFlowCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_flow_ctrl::R`](R) reader structure"]
impl crate::Readable for UartFlowCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_flow_ctrl::W`](W) writer structure"]
impl crate::Writable for UartFlowCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_FLOW_CTRL to value 0"]
impl crate::Resettable for UartFlowCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
