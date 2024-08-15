#[doc = "Register `UART_TX_CTRL` reader"]
pub type R = crate::R<UartTxCtrlSpec>;
#[doc = "Register `UART_TX_CTRL` writer"]
pub type W = crate::W<UartTxCtrlSpec>;
#[doc = "Field `STOP_BITS` reader - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type StopBitsR = crate::FieldReader;
#[doc = "Field `STOP_BITS` writer - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
pub type StopBitsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PARITY` reader - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ENABLED` reader - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
pub type ParityEnabledR = crate::BitReader;
#[doc = "Field `PARITY_ENABLED` writer - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
pub type ParityEnabledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY_ON_NACK` reader - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
pub type RetryOnNackR = crate::BitReader;
#[doc = "Field `RETRY_ON_NACK` writer - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
pub type RetryOnNackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    pub fn stop_bits(&self) -> StopBitsR {
        StopBitsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
    #[inline(always)]
    pub fn parity_enabled(&self) -> ParityEnabledR {
        ParityEnabledR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    pub fn retry_on_nack(&self) -> RetryOnNackR {
        RetryOnNackR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop bits. STOP_BITS + 1 is the duration of the stop period in terms of halve bit periods. Valid range is \\[1, 7\\]; i.e. a stop period should last at least one bit period."]
    #[inline(always)]
    #[must_use]
    pub fn stop_bits(&mut self) -> StopBitsW<UartTxCtrlSpec> {
        StopBitsW::new(self, 0)
    }
    #[doc = "Bit 4 - Parity bit. When '0', the transmitter generates an even parity. When '1', the transmitter generates an odd parity. Only applicable in standard UART and SmartCard submodes."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<UartTxCtrlSpec> {
        ParityW::new(self, 4)
    }
    #[doc = "Bit 5 - Parity generation enabled ('1') or not ('0'). Only applicable in standard UART submodes. In SmartCard submode, parity generation is always enabled through hardware. In IrDA submode, parity generation is always disabled through hardware"]
    #[inline(always)]
    #[must_use]
    pub fn parity_enabled(&mut self) -> ParityEnabledW<UartTxCtrlSpec> {
        ParityEnabledW::new(self, 5)
    }
    #[doc = "Bit 8 - When '1', a data frame is retransmitted when a negative acknowledgement is received. Only applicable to the SmartCard submode."]
    #[inline(always)]
    #[must_use]
    pub fn retry_on_nack(&mut self) -> RetryOnNackW<UartTxCtrlSpec> {
        RetryOnNackW::new(self, 8)
    }
}
#[doc = "UART transmitter control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartTxCtrlSpec;
impl crate::RegisterSpec for UartTxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_tx_ctrl::R`](R) reader structure"]
impl crate::Readable for UartTxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_tx_ctrl::W`](W) writer structure"]
impl crate::Writable for UartTxCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_TX_CTRL to value 0x02"]
impl crate::Resettable for UartTxCtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
