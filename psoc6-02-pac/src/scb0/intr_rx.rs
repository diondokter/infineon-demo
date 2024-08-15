#[doc = "Register `INTR_RX` reader"]
pub type R = crate::R<IntrRxSpec>;
#[doc = "Register `INTR_RX` writer"]
pub type W = crate::W<IntrRxSpec>;
#[doc = "Field `TRIGGER` reader - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TriggerR = crate::BitReader;
#[doc = "Field `TRIGGER` writer - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
pub type TriggerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOT_EMPTY` reader - RX FIFO is not empty. Only used in FIFO mode."]
pub type NotEmptyR = crate::BitReader;
#[doc = "Field `NOT_EMPTY` writer - RX FIFO is not empty. Only used in FIFO mode."]
pub type NotEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL` reader - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
pub type FullR = crate::BitReader;
#[doc = "Field `FULL` writer - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
pub type FullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCKED` reader - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BlockedR = crate::BitReader;
#[doc = "Field `BLOCKED` writer - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
pub type BlockedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ERROR` reader - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FrameErrorR = crate::BitReader;
#[doc = "Field `FRAME_ERROR` writer - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
pub type FrameErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERROR` reader - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type ParityErrorR = crate::BitReader;
#[doc = "Field `PARITY_ERROR` writer - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
pub type ParityErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BAUD_DETECT` reader - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BaudDetectR = crate::BitReader;
#[doc = "Field `BAUD_DETECT` writer - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BaudDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREAK_DETECT` reader - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BreakDetectR = crate::BitReader;
#[doc = "Field `BREAK_DETECT` writer - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
pub type BreakDetectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    pub fn trigger(&self) -> TriggerR {
        TriggerR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    pub fn not_empty(&self) -> NotEmptyR {
        NotEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    pub fn full(&self) -> FullR {
        FullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    pub fn blocked(&self) -> BlockedR {
        BlockedR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    pub fn frame_error(&self) -> FrameErrorR {
        FrameErrorR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    pub fn parity_error(&self) -> ParityErrorR {
        ParityErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn baud_detect(&self) -> BaudDetectR {
        BaudDetectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn break_detect(&self) -> BreakDetectR {
        BreakDetectR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - More entries in the RX FIFO than the value specified by RX_FIFO_CTRL.TRIGGER_LEVEL. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn trigger(&mut self) -> TriggerW<IntrRxSpec> {
        TriggerW::new(self, 0)
    }
    #[doc = "Bit 2 - RX FIFO is not empty. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn not_empty(&mut self) -> NotEmptyW<IntrRxSpec> {
        NotEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - RX FIFO is full. Note that received data frames are lost when the RX FIFO is full. Dependent on CTRL.BYTE_MODE: (FF_DATA_NR = EZ_DATA_NR/2) BYTE_MODE is '0': # entries == FF_DATA_NR/2. BYTE_MODE is '1': # entries == FF_DATA_NR. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FullW<IntrRxSpec> {
        FullW::new(self, 3)
    }
    #[doc = "Bit 5 - Attempt to write to a full RX FIFO. Note: in I2C mode, the OVERFLOW is set when a data frame is received and the RX FIFO is full, independent of whether it is ACK'd or NACK'd. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn overflow(&mut self) -> OverflowW<IntrRxSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Attempt to read from an empty RX FIFO. Only used in FIFO mode."]
    #[inline(always)]
    #[must_use]
    pub fn underflow(&mut self) -> UnderflowW<IntrRxSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - AHB-Lite read transfer can not get access to the EZ memory (EZ_DATA accesses), due to an externally clocked EZ access. This may happen when STATUS.EC_BUSY is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn blocked(&mut self) -> BlockedW<IntrRxSpec> {
        BlockedW::new(self, 7)
    }
    #[doc = "Bit 8 - Frame error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. This can be either a start or stop bit(s) error: Start bit error: after the detection of the beginning of a start bit period (RX line changes from '1' to '0'), the middle of the start bit period is sampled erroneously (RX line is '1'). Note: a start bit error is detected BEFORE a data frame is received. Stop bit error: the RX line is sampled as '0', but a '1' was expected. Note: a stop bit error may result in failure to receive successive data frame(s). Note: a stop bit error is detected AFTER a data frame is received. A stop bit error is detected after a data frame is received, and the UART_RX_CTL.DROP_ON_FRAME_ERROR field specifies whether the received frame is dropped or send to the RX FIFO. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '1', the received data frame is dropped. If UART_RX_CTL.DROP_ON_FRAME_ERROR is '0', the received data frame is send to the RX FIFO. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO; i.e. the RX FIFO does not have error flags to tag erroneous data frames."]
    #[inline(always)]
    #[must_use]
    pub fn frame_error(&mut self) -> FrameErrorW<IntrRxSpec> {
        FrameErrorW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity error in received data frame. Set to '1', when event is detected. Write with '1' to clear bit. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '1', the received frame is dropped. If UART_RX_CTL.DROP_ON_PARITY_ERROR is '0', the received frame is send to the RX FIFO. In SmartCard submode, negatively acknowledged data frames generate a parity error. Note that Firmware can only identify the erroneous data frame in the RX FIFO if it is fast enough to read the data frame before the hardware writes a next data frame into the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn parity_error(&mut self) -> ParityErrorW<IntrRxSpec> {
        ParityErrorW::new(self, 9)
    }
    #[doc = "Bit 10 - LIN baudrate detection is completed. The receiver software uses the UART_RX_STATUS.BR_COUNTER value to set the right IP clock (from the programmable clock IP) to guarantee successful receipt of the first LIN data frame (Protected Identifier Field) after the synchronization byte. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn baud_detect(&mut self) -> BaudDetectW<IntrRxSpec> {
        BaudDetectW::new(self, 10)
    }
    #[doc = "Bit 11 - Break detection is successful: the line is '0' for UART_RX_CTRL.BREAK_WIDTH + 1 bit period. Can occur at any time to address unanticipated break fields; i.e. 'break-in-data' is supported. This feature is supported for the UART standard and LIN submodes. For the UART standard submodes, ongoing receipt of data frames is NOT affected; i.e. Firmware is expected to take the proper action. For the LIN submode, possible ongoing receipt of a data frame is stopped and the (partially) received data frame is dropped and baud rate detection is started. Set to '1', when event is detected. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn break_detect(&mut self) -> BreakDetectW<IntrRxSpec> {
        BreakDetectW::new(self, 11)
    }
}
#[doc = "Receiver interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_rx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrRxSpec;
impl crate::RegisterSpec for IntrRxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_rx::R`](R) reader structure"]
impl crate::Readable for IntrRxSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_rx::W`](W) writer structure"]
impl crate::Writable for IntrRxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_RX to value 0"]
impl crate::Resettable for IntrRxSpec {
    const RESET_VALUE: u32 = 0;
}
