#[doc = "Register `UART_CTRL` reader"]
pub type R = crate::R<UartCtrlSpec>;
#[doc = "Register `UART_CTRL` writer"]
pub type W = crate::W<UartCtrlSpec>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Standard UART submode."]
    UartStd = 0,
    #[doc = "1: SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    UartSmartcard = 1,
    #[doc = "2: Infrared Data Association (IrDA) submode. Return to Zero modulation scheme."]
    UartIrda = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - N/A"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::UartStd),
            1 => Some(Mode::UartSmartcard),
            2 => Some(Mode::UartIrda),
            _ => None,
        }
    }
    #[doc = "Standard UART submode."]
    #[inline(always)]
    pub fn is_uart_std(&self) -> bool {
        *self == Mode::UartStd
    }
    #[doc = "SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    #[inline(always)]
    pub fn is_uart_smartcard(&self) -> bool {
        *self == Mode::UartSmartcard
    }
    #[doc = "Infrared Data Association (IrDA) submode. Return to Zero modulation scheme."]
    #[inline(always)]
    pub fn is_uart_irda(&self) -> bool {
        *self == Mode::UartIrda
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard UART submode."]
    #[inline(always)]
    pub fn uart_std(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UartStd)
    }
    #[doc = "SmartCard (ISO7816) submode. Support for negative acknowledgement (NACK) on the receiver side and retransmission on the transmitter side."]
    #[inline(always)]
    pub fn uart_smartcard(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UartSmartcard)
    }
    #[doc = "Infrared Data Association (IrDA) submode. Return to Zero modulation scheme."]
    #[inline(always)]
    pub fn uart_irda(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UartIrda)
    }
}
impl R {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). When '0', the transmitter TX line 'uart_tx_out' is connected to the TX pin and the receiver RX line 'uart_rx_in' is connected to the RX pin. When '1', the transmitter TX line 'uart_tx_out' is connected to the receiver RX line 'uart_rx_in'. A similar connections scheme is followed for 'uart_rts_out' and 'uart_cts_in'. This allows a SCB UART transmitter to communicate with its receiver counterpart."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<UartCtrlSpec> {
        LoopbackW::new(self, 16)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<UartCtrlSpec> {
        ModeW::new(self, 24)
    }
}
#[doc = "UART control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartCtrlSpec;
impl crate::RegisterSpec for UartCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_ctrl::R`](R) reader structure"]
impl crate::Readable for UartCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_ctrl::W`](W) writer structure"]
impl crate::Writable for UartCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_CTRL to value 0x0300_0000"]
impl crate::Resettable for UartCtrlSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
