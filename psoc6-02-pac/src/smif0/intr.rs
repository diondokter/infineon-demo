#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `TR_TX_REQ` reader - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
pub type TrTxReqR = crate::BitReader;
#[doc = "Field `TR_TX_REQ` writer - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
pub type TrTxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR_RX_REQ` reader - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
pub type TrRxReqR = crate::BitReader;
#[doc = "Field `TR_RX_REQ` writer - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
pub type TrRxReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIP_ALIGNMENT_ERROR` reader - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
pub type XipAlignmentErrorR = crate::BitReader;
#[doc = "Field `XIP_ALIGNMENT_ERROR` writer - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
pub type XipAlignmentErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` reader - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
pub type TxCmdFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_CMD_FIFO_OVERFLOW` writer - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
pub type TxCmdFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` reader - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
pub type TxDataFifoOverflowR = crate::BitReader;
#[doc = "Field `TX_DATA_FIFO_OVERFLOW` writer - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
pub type TxDataFifoOverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` reader - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RxDataFifoUnderflowR = crate::BitReader;
#[doc = "Field `RX_DATA_FIFO_UNDERFLOW` writer - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
pub type RxDataFifoUnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    pub fn tr_tx_req(&self) -> TrTxReqR {
        TrTxReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    pub fn tr_rx_req(&self) -> TrRxReqR {
        TrRxReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    pub fn xip_alignment_error(&self) -> XipAlignmentErrorR {
        XipAlignmentErrorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_cmd_fifo_overflow(&self) -> TxCmdFifoOverflowR {
        TxCmdFifoOverflowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    pub fn tx_data_fifo_overflow(&self) -> TxDataFifoOverflowR {
        TxDataFifoOverflowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    pub fn rx_data_fifo_underflow(&self) -> RxDataFifoUnderflowR {
        RxDataFifoUnderflowR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Activated in MMIO mode, when a TX data FIFO trigger 'tr_tx_req' is activated."]
    #[inline(always)]
    #[must_use]
    pub fn tr_tx_req(&mut self) -> TrTxReqW<IntrSpec> {
        TrTxReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Activated in MMIO mode, when a RX data FIFO trigger 'tr_rx_req' is activated."]
    #[inline(always)]
    #[must_use]
    pub fn tr_rx_req(&mut self) -> TrRxReqW<IntrSpec> {
        TrRxReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Activated in XIP mode, if: - The selected device's ADDR_CTL.DIV2 is '1' and the AHB-Lite bus transfer address is not a multiple of 2. - The selected device's ADDR_CTL.DIV2 is '1' and the XIP transfer request is NOT for a multiple of 2 Bytes. Note: In dual-quad SPI mode (ADDR_CTL.DIV is '1'), each memory device contributes a 4-bit nibble for read data or write data. This is only possible if the request address is a multiple of 2 and the number of requested Bytes is a multiple of 2."]
    #[inline(always)]
    #[must_use]
    pub fn xip_alignment_error(&mut self) -> XipAlignmentErrorW<IntrSpec> {
        XipAlignmentErrorW::new(self, 2)
    }
    #[doc = "Bit 3 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX command FIFO (TX_CMD_FIFO_WR) with not enough free entries available."]
    #[inline(always)]
    #[must_use]
    pub fn tx_cmd_fifo_overflow(&mut self) -> TxCmdFifoOverflowW<IntrSpec> {
        TxCmdFifoOverflowW::new(self, 3)
    }
    #[doc = "Bit 4 - Activated in MMIO mode, on an AHB-Lite write transfer to the TX data FIFO (TX_DATA_FIFO_WR1, TX_DATA_FIFO_WR2, TX_DATA_FIFO_WR4) with not enough free entries available."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_fifo_overflow(&mut self) -> TxDataFifoOverflowW<IntrSpec> {
        TxDataFifoOverflowW::new(self, 4)
    }
    #[doc = "Bit 5 - Activated in MMIO mode, on an AHB-Lite read transfer from the RX data FIFO (RX_DATA_FIFO_RD1, RX_DATA_FIFO_RD2, RX_DATA_FIFO_RD4) with not enough entries available. Only activated for NON test bus controller transfers."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_fifo_underflow(&mut self) -> RxDataFifoUnderflowW<IntrSpec> {
        RxDataFifoUnderflowW::new(self, 5)
    }
}
#[doc = "Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
