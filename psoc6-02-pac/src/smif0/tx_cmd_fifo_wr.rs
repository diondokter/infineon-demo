#[doc = "Register `TX_CMD_FIFO_WR` writer"]
pub type W = crate::W<TxCmdFifoWrSpec>;
#[doc = "Field `DATA20` writer - Command data. The higher two bits DATA\\[19:18\\]
specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\]
specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\]
specifies which of the four devices are selected. DATA\\[11:8\\]
are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\]
specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\]
specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
pub type Data20W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl W {
    #[doc = "Bits 0:19 - Command data. The higher two bits DATA\\[19:18\\]
specify the specific command '0'/TX: A SPI transfer always start with a TX command FIFO entry of the 'TX' format. - DATA\\[17:16\\]
specifies the width of the data transfer: - '0': 1 bit/cycle (single data transfer). - '1': 2 bits/cycle (dual data transfer). - '2': 4 bits/cycle (quad data transfer). - '3': 8 bits/cycle (octal data transfer). - DATA\\[15\\]: specifies whether this is the last TX Byte; i.e. whether the 'spi_select_out\\[3:0\\]' IO output signals are de-activated after the transfer. - DATA\\[11:8\\]
specifies which of the four devices are selected. DATA\\[11:8\\]
are directly mapped to 'spi_select_out\\[3:0\\]'. Two devices can be selected at the same time in dual-quad mode. - '0': device deselected - '1': device selected - DATA\\[7:0\\]
specifies the transmitted Byte. '1'/TX_COUNT: The 'TX_COUNT' command relies on the TX data FIFO to provide the transmitted bytes. The 'TX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) from the TX data FIFO. '2'/RX_COUNT: The 'RX_COUNT' command relies on the RX data FIFO to accept the received bytes. The 'RX_COUNT' command is ALWAYS considered to be the last command of a SPI data transfers. - DATA\\[17:16\\]
specifies the width of the transfer. - DATA\\[15:0\\]
specifies the number of to be transmitted Bytes (minus 1) to the RX data FIFO. '3'/DUMMY_COUNT: The 'DUMMY_COUNT' command conveys dummy cycles. Dummy cycles are used to implement a Turn-Around time in which the SPI master changes from a transmitter driving the data lines to a receiver receiving on the same data lines. The 'DUMMY_COUNT' command is ALWAYS considered to be NOT the last command of a SPI data transfers; i.e. it needs to be followed by another command. - DATA\\[15:0\\]
specifies the number of dummy cycles (minus 1). In dummy cycles, the data lines are not driven."]
    #[inline(always)]
    #[must_use]
    pub fn data20(&mut self) -> Data20W<TxCmdFifoWrSpec> {
        Data20W::new(self, 0)
    }
}
#[doc = "Transmitter command FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cmd_fifo_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCmdFifoWrSpec;
impl crate::RegisterSpec for TxCmdFifoWrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tx_cmd_fifo_wr::W`](W) writer structure"]
impl crate::Writable for TxCmdFifoWrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_CMD_FIFO_WR to value 0"]
impl crate::Resettable for TxCmdFifoWrSpec {
    const RESET_VALUE: u32 = 0;
}
