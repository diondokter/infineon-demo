#[doc = "Register `INTR_TX` reader"]
pub type R = crate::R<IntrTxSpec>;
#[doc = "Register `INTR_TX` writer"]
pub type W = crate::W<IntrTxSpec>;
#[doc = "Field `TRIGGER` reader - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `TRIGGER` writer - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_FULL` reader - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
pub type NotFullR = crate::BitReader;
#[doc = "Field `NOT_FULL` writer - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
pub type NotFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMPTY` reader - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
pub type EmptyR = crate::BitReader;
#[doc = "Field `EMPTY` writer - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
pub type EmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `BLOCKED` writer - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_NACK` reader - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartNackR = crate::BitReader;
#[doc = "Field `UART_NACK` writer - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartNackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_DONE` reader - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartDoneR = crate::BitReader;
#[doc = "Field `UART_DONE` writer - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_ARB_LOST` reader - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartArbLostR = crate::BitReader;
#[doc = "Field `UART_ARB_LOST` writer - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type UartArbLostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_full(&self) -> NotFullR {
        NotFullR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_nack(&self) -> UartNackR {
        UartNackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_done(&self) -> UartDoneR {
        UartDoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn uart_arb_lost(&self) -> UartArbLostR {
        UartArbLostR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Less entries in the TX FIFO than the value specified by TX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<IntrTxSpec> {
        TriggerW::new(self, 0)
    }
    #[doc = "Bit 1 - TX FIFO is not full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries != FF_DATA_NR/2. BYTE_MODE is '1': # entries != FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn not_full(&mut self) -> NotFullW<IntrTxSpec> {
        NotFullW::new(self, 1)
    }
    #[doc = "Bit 4 - TX FIFO is empty; i.e. it has 0 entries. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EmptyW<IntrTxSpec> {
        EmptyW::new(self, 4)
    }
    #[doc = "Bit 5 - Attempt to write to a full TX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<IntrTxSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Attempt to read from an empty TX FIFO. This happens when the IP is ready to transfer data and EMPTY is '1'. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UnderflowW<IntrTxSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB-Lite write transfer can not get access to the EZ memory (EZ data access), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BlockedW<IntrTxSpec> {
        BlockedW::new(self, 7)
    }
    #[doc = "Bit 8 - UART transmitter received a negative acknowledgement in SmartCard mode. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn uart_nack(&mut self) -> UartNackW<IntrTxSpec> {
        UartNackW::new(self, 8)
    }
    #[doc = "Bit 9 - UART transmitter done event. This happens when the IP is done transferring all data in the TX FIFO, and the last stop field is transmitted (both TX FIFO and transmit shifter register are empty). Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn uart_done(&mut self) -> UartDoneW<IntrTxSpec> {
        UartDoneW::new(self, 9)
    }
    #[doc = "Bit 10 - UART lost arbitration: the value driven on the TX line is not the same as the value observed on the RX line. This condition event is useful when transmitter and receiver share a TX/RX line. This is the case in LIN or SmartCard modes. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn uart_arb_lost(&mut self) -> UartArbLostW<IntrTxSpec> {
        UartArbLostW::new(self, 10)
    }
}
#[doc = "Transmitter interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrTxSpec;
impl crate::RegisterSpec for IntrTxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_tx::R`](R) reader structure"]
impl crate::Readable for IntrTxSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_tx::W`](W) writer structure"]
impl crate::Writable for IntrTxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_TX to value 0"]
impl crate::Resettable for IntrTxSpec {
    const RESET_VALUE: u32 = 0;
}
