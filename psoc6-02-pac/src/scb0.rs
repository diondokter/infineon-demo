#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    cmd_resp_ctrl: CmdRespCtrl,
    cmd_resp_status: CmdRespStatus,
    _reserved4: [u8; 0x10],
    spi_ctrl: SpiCtrl,
    spi_status: SpiStatus,
    _reserved6: [u8; 0x18],
    uart_ctrl: UartCtrl,
    uart_tx_ctrl: UartTxCtrl,
    uart_rx_ctrl: UartRxCtrl,
    uart_rx_status: UartRxStatus,
    uart_flow_ctrl: UartFlowCtrl,
    _reserved11: [u8; 0x0c],
    i2c_ctrl: I2cCtrl,
    i2c_status: I2cStatus,
    i2c_m_cmd: I2cMCmd,
    i2c_s_cmd: I2cSCmd,
    i2c_cfg: I2cCfg,
    _reserved16: [u8; 0x018c],
    tx_ctrl: TxCtrl,
    tx_fifo_ctrl: TxFifoCtrl,
    tx_fifo_status: TxFifoStatus,
    _reserved19: [u8; 0x34],
    tx_fifo_wr: TxFifoWr,
    _reserved20: [u8; 0xbc],
    rx_ctrl: RxCtrl,
    rx_fifo_ctrl: RxFifoCtrl,
    rx_fifo_status: RxFifoStatus,
    _reserved23: [u8; 0x04],
    rx_match: RxMatch,
    _reserved24: [u8; 0x2c],
    rx_fifo_rd: RxFifoRd,
    rx_fifo_rd_silent: RxFifoRdSilent,
    _reserved26: [u8; 0x0ab8],
    intr_cause: IntrCause,
    _reserved27: [u8; 0x7c],
    intr_i2c_ec: IntrI2cEc,
    _reserved28: [u8; 0x04],
    intr_i2c_ec_mask: IntrI2cEcMask,
    intr_i2c_ec_masked: IntrI2cEcMasked,
    _reserved30: [u8; 0x30],
    intr_spi_ec: IntrSpiEc,
    _reserved31: [u8; 0x04],
    intr_spi_ec_mask: IntrSpiEcMask,
    intr_spi_ec_masked: IntrSpiEcMasked,
    _reserved33: [u8; 0x30],
    intr_m: IntrM,
    intr_m_set: IntrMSet,
    intr_m_mask: IntrMMask,
    intr_m_masked: IntrMMasked,
    _reserved37: [u8; 0x30],
    intr_s: IntrS,
    intr_s_set: IntrSSet,
    intr_s_mask: IntrSMask,
    intr_s_masked: IntrSMasked,
    _reserved41: [u8; 0x30],
    intr_tx: IntrTx,
    intr_tx_set: IntrTxSet,
    intr_tx_mask: IntrTxMask,
    intr_tx_masked: IntrTxMasked,
    _reserved45: [u8; 0x30],
    intr_rx: IntrRx,
    intr_rx_set: IntrRxSet,
    intr_rx_mask: IntrRxMask,
    intr_rx_masked: IntrRxMasked,
}
impl RegisterBlock {
    #[doc = "0x00 - Generic control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Generic status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Command/response control"]
    #[inline(always)]
    pub const fn cmd_resp_ctrl(&self) -> &CmdRespCtrl {
        &self.cmd_resp_ctrl
    }
    #[doc = "0x0c - Command/response status"]
    #[inline(always)]
    pub const fn cmd_resp_status(&self) -> &CmdRespStatus {
        &self.cmd_resp_status
    }
    #[doc = "0x20 - SPI control"]
    #[inline(always)]
    pub const fn spi_ctrl(&self) -> &SpiCtrl {
        &self.spi_ctrl
    }
    #[doc = "0x24 - SPI status"]
    #[inline(always)]
    pub const fn spi_status(&self) -> &SpiStatus {
        &self.spi_status
    }
    #[doc = "0x40 - UART control"]
    #[inline(always)]
    pub const fn uart_ctrl(&self) -> &UartCtrl {
        &self.uart_ctrl
    }
    #[doc = "0x44 - UART transmitter control"]
    #[inline(always)]
    pub const fn uart_tx_ctrl(&self) -> &UartTxCtrl {
        &self.uart_tx_ctrl
    }
    #[doc = "0x48 - UART receiver control"]
    #[inline(always)]
    pub const fn uart_rx_ctrl(&self) -> &UartRxCtrl {
        &self.uart_rx_ctrl
    }
    #[doc = "0x4c - UART receiver status"]
    #[inline(always)]
    pub const fn uart_rx_status(&self) -> &UartRxStatus {
        &self.uart_rx_status
    }
    #[doc = "0x50 - UART flow control"]
    #[inline(always)]
    pub const fn uart_flow_ctrl(&self) -> &UartFlowCtrl {
        &self.uart_flow_ctrl
    }
    #[doc = "0x60 - I2C control"]
    #[inline(always)]
    pub const fn i2c_ctrl(&self) -> &I2cCtrl {
        &self.i2c_ctrl
    }
    #[doc = "0x64 - I2C status"]
    #[inline(always)]
    pub const fn i2c_status(&self) -> &I2cStatus {
        &self.i2c_status
    }
    #[doc = "0x68 - I2C master command"]
    #[inline(always)]
    pub const fn i2c_m_cmd(&self) -> &I2cMCmd {
        &self.i2c_m_cmd
    }
    #[doc = "0x6c - I2C slave command"]
    #[inline(always)]
    pub const fn i2c_s_cmd(&self) -> &I2cSCmd {
        &self.i2c_s_cmd
    }
    #[doc = "0x70 - I2C configuration"]
    #[inline(always)]
    pub const fn i2c_cfg(&self) -> &I2cCfg {
        &self.i2c_cfg
    }
    #[doc = "0x200 - Transmitter control"]
    #[inline(always)]
    pub const fn tx_ctrl(&self) -> &TxCtrl {
        &self.tx_ctrl
    }
    #[doc = "0x204 - Transmitter FIFO control"]
    #[inline(always)]
    pub const fn tx_fifo_ctrl(&self) -> &TxFifoCtrl {
        &self.tx_fifo_ctrl
    }
    #[doc = "0x208 - Transmitter FIFO status"]
    #[inline(always)]
    pub const fn tx_fifo_status(&self) -> &TxFifoStatus {
        &self.tx_fifo_status
    }
    #[doc = "0x240 - Transmitter FIFO write"]
    #[inline(always)]
    pub const fn tx_fifo_wr(&self) -> &TxFifoWr {
        &self.tx_fifo_wr
    }
    #[doc = "0x300 - Receiver control"]
    #[inline(always)]
    pub const fn rx_ctrl(&self) -> &RxCtrl {
        &self.rx_ctrl
    }
    #[doc = "0x304 - Receiver FIFO control"]
    #[inline(always)]
    pub const fn rx_fifo_ctrl(&self) -> &RxFifoCtrl {
        &self.rx_fifo_ctrl
    }
    #[doc = "0x308 - Receiver FIFO status"]
    #[inline(always)]
    pub const fn rx_fifo_status(&self) -> &RxFifoStatus {
        &self.rx_fifo_status
    }
    #[doc = "0x310 - Slave address and mask"]
    #[inline(always)]
    pub const fn rx_match(&self) -> &RxMatch {
        &self.rx_match
    }
    #[doc = "0x340 - Receiver FIFO read"]
    #[inline(always)]
    pub const fn rx_fifo_rd(&self) -> &RxFifoRd {
        &self.rx_fifo_rd
    }
    #[doc = "0x344 - Receiver FIFO read silent"]
    #[inline(always)]
    pub const fn rx_fifo_rd_silent(&self) -> &RxFifoRdSilent {
        &self.rx_fifo_rd_silent
    }
    #[doc = "0xe00 - Active clocked interrupt signal"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &IntrCause {
        &self.intr_cause
    }
    #[doc = "0xe80 - Externally clocked I2C interrupt request"]
    #[inline(always)]
    pub const fn intr_i2c_ec(&self) -> &IntrI2cEc {
        &self.intr_i2c_ec
    }
    #[doc = "0xe88 - Externally clocked I2C interrupt mask"]
    #[inline(always)]
    pub const fn intr_i2c_ec_mask(&self) -> &IntrI2cEcMask {
        &self.intr_i2c_ec_mask
    }
    #[doc = "0xe8c - Externally clocked I2C interrupt masked"]
    #[inline(always)]
    pub const fn intr_i2c_ec_masked(&self) -> &IntrI2cEcMasked {
        &self.intr_i2c_ec_masked
    }
    #[doc = "0xec0 - Externally clocked SPI interrupt request"]
    #[inline(always)]
    pub const fn intr_spi_ec(&self) -> &IntrSpiEc {
        &self.intr_spi_ec
    }
    #[doc = "0xec8 - Externally clocked SPI interrupt mask"]
    #[inline(always)]
    pub const fn intr_spi_ec_mask(&self) -> &IntrSpiEcMask {
        &self.intr_spi_ec_mask
    }
    #[doc = "0xecc - Externally clocked SPI interrupt masked"]
    #[inline(always)]
    pub const fn intr_spi_ec_masked(&self) -> &IntrSpiEcMasked {
        &self.intr_spi_ec_masked
    }
    #[doc = "0xf00 - Master interrupt request"]
    #[inline(always)]
    pub const fn intr_m(&self) -> &IntrM {
        &self.intr_m
    }
    #[doc = "0xf04 - Master interrupt set request"]
    #[inline(always)]
    pub const fn intr_m_set(&self) -> &IntrMSet {
        &self.intr_m_set
    }
    #[doc = "0xf08 - Master interrupt mask"]
    #[inline(always)]
    pub const fn intr_m_mask(&self) -> &IntrMMask {
        &self.intr_m_mask
    }
    #[doc = "0xf0c - Master interrupt masked request"]
    #[inline(always)]
    pub const fn intr_m_masked(&self) -> &IntrMMasked {
        &self.intr_m_masked
    }
    #[doc = "0xf40 - Slave interrupt request"]
    #[inline(always)]
    pub const fn intr_s(&self) -> &IntrS {
        &self.intr_s
    }
    #[doc = "0xf44 - Slave interrupt set request"]
    #[inline(always)]
    pub const fn intr_s_set(&self) -> &IntrSSet {
        &self.intr_s_set
    }
    #[doc = "0xf48 - Slave interrupt mask"]
    #[inline(always)]
    pub const fn intr_s_mask(&self) -> &IntrSMask {
        &self.intr_s_mask
    }
    #[doc = "0xf4c - Slave interrupt masked request"]
    #[inline(always)]
    pub const fn intr_s_masked(&self) -> &IntrSMasked {
        &self.intr_s_masked
    }
    #[doc = "0xf80 - Transmitter interrupt request"]
    #[inline(always)]
    pub const fn intr_tx(&self) -> &IntrTx {
        &self.intr_tx
    }
    #[doc = "0xf84 - Transmitter interrupt set request"]
    #[inline(always)]
    pub const fn intr_tx_set(&self) -> &IntrTxSet {
        &self.intr_tx_set
    }
    #[doc = "0xf88 - Transmitter interrupt mask"]
    #[inline(always)]
    pub const fn intr_tx_mask(&self) -> &IntrTxMask {
        &self.intr_tx_mask
    }
    #[doc = "0xf8c - Transmitter interrupt masked request"]
    #[inline(always)]
    pub const fn intr_tx_masked(&self) -> &IntrTxMasked {
        &self.intr_tx_masked
    }
    #[doc = "0xfc0 - Receiver interrupt request"]
    #[inline(always)]
    pub const fn intr_rx(&self) -> &IntrRx {
        &self.intr_rx
    }
    #[doc = "0xfc4 - Receiver interrupt set request"]
    #[inline(always)]
    pub const fn intr_rx_set(&self) -> &IntrRxSet {
        &self.intr_rx_set
    }
    #[doc = "0xfc8 - Receiver interrupt mask"]
    #[inline(always)]
    pub const fn intr_rx_mask(&self) -> &IntrRxMask {
        &self.intr_rx_mask
    }
    #[doc = "0xfcc - Receiver interrupt masked request"]
    #[inline(always)]
    pub const fn intr_rx_masked(&self) -> &IntrRxMasked {
        &self.intr_rx_masked
    }
}
#[doc = "CTRL (rw) register accessor: Generic control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Generic control"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: Generic status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Generic status"]
pub mod status;
#[doc = "CMD_RESP_CTRL (rw) register accessor: Command/response control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_resp_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_resp_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_resp_ctrl`]
module"]
#[doc(alias = "CMD_RESP_CTRL")]
pub type CmdRespCtrl = crate::Reg<cmd_resp_ctrl::CmdRespCtrlSpec>;
#[doc = "Command/response control"]
pub mod cmd_resp_ctrl;
#[doc = "CMD_RESP_STATUS (r) register accessor: Command/response status\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_resp_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_resp_status`]
module"]
#[doc(alias = "CMD_RESP_STATUS")]
pub type CmdRespStatus = crate::Reg<cmd_resp_status::CmdRespStatusSpec>;
#[doc = "Command/response status"]
pub mod cmd_resp_status;
#[doc = "SPI_CTRL (rw) register accessor: SPI control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_ctrl`]
module"]
#[doc(alias = "SPI_CTRL")]
pub type SpiCtrl = crate::Reg<spi_ctrl::SpiCtrlSpec>;
#[doc = "SPI control"]
pub mod spi_ctrl;
#[doc = "SPI_STATUS (r) register accessor: SPI status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi_status`]
module"]
#[doc(alias = "SPI_STATUS")]
pub type SpiStatus = crate::Reg<spi_status::SpiStatusSpec>;
#[doc = "SPI status"]
pub mod spi_status;
#[doc = "UART_CTRL (rw) register accessor: UART control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_ctrl`]
module"]
#[doc(alias = "UART_CTRL")]
pub type UartCtrl = crate::Reg<uart_ctrl::UartCtrlSpec>;
#[doc = "UART control"]
pub mod uart_ctrl;
#[doc = "UART_TX_CTRL (rw) register accessor: UART transmitter control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_tx_ctrl`]
module"]
#[doc(alias = "UART_TX_CTRL")]
pub type UartTxCtrl = crate::Reg<uart_tx_ctrl::UartTxCtrlSpec>;
#[doc = "UART transmitter control"]
pub mod uart_tx_ctrl;
#[doc = "UART_RX_CTRL (rw) register accessor: UART receiver control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_ctrl`]
module"]
#[doc(alias = "UART_RX_CTRL")]
pub type UartRxCtrl = crate::Reg<uart_rx_ctrl::UartRxCtrlSpec>;
#[doc = "UART receiver control"]
pub mod uart_rx_ctrl;
#[doc = "UART_RX_STATUS (r) register accessor: UART receiver status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_rx_status`]
module"]
#[doc(alias = "UART_RX_STATUS")]
pub type UartRxStatus = crate::Reg<uart_rx_status::UartRxStatusSpec>;
#[doc = "UART receiver status"]
pub mod uart_rx_status;
#[doc = "UART_FLOW_CTRL (rw) register accessor: UART flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_flow_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_flow_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart_flow_ctrl`]
module"]
#[doc(alias = "UART_FLOW_CTRL")]
pub type UartFlowCtrl = crate::Reg<uart_flow_ctrl::UartFlowCtrlSpec>;
#[doc = "UART flow control"]
pub mod uart_flow_ctrl;
#[doc = "I2C_CTRL (rw) register accessor: I2C control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_ctrl`]
module"]
#[doc(alias = "I2C_CTRL")]
pub type I2cCtrl = crate::Reg<i2c_ctrl::I2cCtrlSpec>;
#[doc = "I2C control"]
pub mod i2c_ctrl;
#[doc = "I2C_STATUS (r) register accessor: I2C status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_status`]
module"]
#[doc(alias = "I2C_STATUS")]
pub type I2cStatus = crate::Reg<i2c_status::I2cStatusSpec>;
#[doc = "I2C status"]
pub mod i2c_status;
#[doc = "I2C_M_CMD (rw) register accessor: I2C master command\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_m_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_m_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_m_cmd`]
module"]
#[doc(alias = "I2C_M_CMD")]
pub type I2cMCmd = crate::Reg<i2c_m_cmd::I2cMCmdSpec>;
#[doc = "I2C master command"]
pub mod i2c_m_cmd;
#[doc = "I2C_S_CMD (rw) register accessor: I2C slave command\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_s_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_s_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_s_cmd`]
module"]
#[doc(alias = "I2C_S_CMD")]
pub type I2cSCmd = crate::Reg<i2c_s_cmd::I2cSCmdSpec>;
#[doc = "I2C slave command"]
pub mod i2c_s_cmd;
#[doc = "I2C_CFG (rw) register accessor: I2C configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_cfg`]
module"]
#[doc(alias = "I2C_CFG")]
pub type I2cCfg = crate::Reg<i2c_cfg::I2cCfgSpec>;
#[doc = "I2C configuration"]
pub mod i2c_cfg;
#[doc = "TX_CTRL (rw) register accessor: Transmitter control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ctrl`]
module"]
#[doc(alias = "TX_CTRL")]
pub type TxCtrl = crate::Reg<tx_ctrl::TxCtrlSpec>;
#[doc = "Transmitter control"]
pub mod tx_ctrl;
#[doc = "TX_FIFO_CTRL (rw) register accessor: Transmitter FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_ctrl`]
module"]
#[doc(alias = "TX_FIFO_CTRL")]
pub type TxFifoCtrl = crate::Reg<tx_fifo_ctrl::TxFifoCtrlSpec>;
#[doc = "Transmitter FIFO control"]
pub mod tx_fifo_ctrl;
#[doc = "TX_FIFO_STATUS (r) register accessor: Transmitter FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_status`]
module"]
#[doc(alias = "TX_FIFO_STATUS")]
pub type TxFifoStatus = crate::Reg<tx_fifo_status::TxFifoStatusSpec>;
#[doc = "Transmitter FIFO status"]
pub mod tx_fifo_status;
#[doc = "TX_FIFO_WR (w) register accessor: Transmitter FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_fifo_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_fifo_wr`]
module"]
#[doc(alias = "TX_FIFO_WR")]
pub type TxFifoWr = crate::Reg<tx_fifo_wr::TxFifoWrSpec>;
#[doc = "Transmitter FIFO write"]
pub mod tx_fifo_wr;
#[doc = "RX_CTRL (rw) register accessor: Receiver control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ctrl`]
module"]
#[doc(alias = "RX_CTRL")]
pub type RxCtrl = crate::Reg<rx_ctrl::RxCtrlSpec>;
#[doc = "Receiver control"]
pub mod rx_ctrl;
#[doc = "RX_FIFO_CTRL (rw) register accessor: Receiver FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_fifo_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_ctrl`]
module"]
#[doc(alias = "RX_FIFO_CTRL")]
pub type RxFifoCtrl = crate::Reg<rx_fifo_ctrl::RxFifoCtrlSpec>;
#[doc = "Receiver FIFO control"]
pub mod rx_fifo_ctrl;
#[doc = "RX_FIFO_STATUS (r) register accessor: Receiver FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_status`]
module"]
#[doc(alias = "RX_FIFO_STATUS")]
pub type RxFifoStatus = crate::Reg<rx_fifo_status::RxFifoStatusSpec>;
#[doc = "Receiver FIFO status"]
pub mod rx_fifo_status;
#[doc = "RX_MATCH (rw) register accessor: Slave address and mask\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_match::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_match::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_match`]
module"]
#[doc(alias = "RX_MATCH")]
pub type RxMatch = crate::Reg<rx_match::RxMatchSpec>;
#[doc = "Slave address and mask"]
pub mod rx_match;
#[doc = "RX_FIFO_RD (r) register accessor: Receiver FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd`]
module"]
#[doc(alias = "RX_FIFO_RD")]
pub type RxFifoRd = crate::Reg<rx_fifo_rd::RxFifoRdSpec>;
#[doc = "Receiver FIFO read"]
pub mod rx_fifo_rd;
#[doc = "RX_FIFO_RD_SILENT (r) register accessor: Receiver FIFO read silent\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_fifo_rd_silent::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_fifo_rd_silent`]
module"]
#[doc(alias = "RX_FIFO_RD_SILENT")]
pub type RxFifoRdSilent = crate::Reg<rx_fifo_rd_silent::RxFifoRdSilentSpec>;
#[doc = "Receiver FIFO read silent"]
pub mod rx_fifo_rd_silent;
#[doc = "INTR_CAUSE (r) register accessor: Active clocked interrupt signal\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
#[doc(alias = "INTR_CAUSE")]
pub type IntrCause = crate::Reg<intr_cause::IntrCauseSpec>;
#[doc = "Active clocked interrupt signal"]
pub mod intr_cause;
#[doc = "INTR_I2C_EC (rw) register accessor: Externally clocked I2C interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_i2c_ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec`]
module"]
#[doc(alias = "INTR_I2C_EC")]
pub type IntrI2cEc = crate::Reg<intr_i2c_ec::IntrI2cEcSpec>;
#[doc = "Externally clocked I2C interrupt request"]
pub mod intr_i2c_ec;
#[doc = "INTR_I2C_EC_MASK (rw) register accessor: Externally clocked I2C interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_i2c_ec_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec_mask`]
module"]
#[doc(alias = "INTR_I2C_EC_MASK")]
pub type IntrI2cEcMask = crate::Reg<intr_i2c_ec_mask::IntrI2cEcMaskSpec>;
#[doc = "Externally clocked I2C interrupt mask"]
pub mod intr_i2c_ec_mask;
#[doc = "INTR_I2C_EC_MASKED (r) register accessor: Externally clocked I2C interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_i2c_ec_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_i2c_ec_masked`]
module"]
#[doc(alias = "INTR_I2C_EC_MASKED")]
pub type IntrI2cEcMasked = crate::Reg<intr_i2c_ec_masked::IntrI2cEcMaskedSpec>;
#[doc = "Externally clocked I2C interrupt masked"]
pub mod intr_i2c_ec_masked;
#[doc = "INTR_SPI_EC (rw) register accessor: Externally clocked SPI interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_spi_ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_spi_ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec`]
module"]
#[doc(alias = "INTR_SPI_EC")]
pub type IntrSpiEc = crate::Reg<intr_spi_ec::IntrSpiEcSpec>;
#[doc = "Externally clocked SPI interrupt request"]
pub mod intr_spi_ec;
#[doc = "INTR_SPI_EC_MASK (rw) register accessor: Externally clocked SPI interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_spi_ec_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_spi_ec_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec_mask`]
module"]
#[doc(alias = "INTR_SPI_EC_MASK")]
pub type IntrSpiEcMask = crate::Reg<intr_spi_ec_mask::IntrSpiEcMaskSpec>;
#[doc = "Externally clocked SPI interrupt mask"]
pub mod intr_spi_ec_mask;
#[doc = "INTR_SPI_EC_MASKED (r) register accessor: Externally clocked SPI interrupt masked\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_spi_ec_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_spi_ec_masked`]
module"]
#[doc(alias = "INTR_SPI_EC_MASKED")]
pub type IntrSpiEcMasked = crate::Reg<intr_spi_ec_masked::IntrSpiEcMaskedSpec>;
#[doc = "Externally clocked SPI interrupt masked"]
pub mod intr_spi_ec_masked;
#[doc = "INTR_M (rw) register accessor: Master interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m`]
module"]
#[doc(alias = "INTR_M")]
pub type IntrM = crate::Reg<intr_m::IntrMSpec>;
#[doc = "Master interrupt request"]
pub mod intr_m;
#[doc = "INTR_M_SET (rw) register accessor: Master interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_set`]
module"]
#[doc(alias = "INTR_M_SET")]
pub type IntrMSet = crate::Reg<intr_m_set::IntrMSetSpec>;
#[doc = "Master interrupt set request"]
pub mod intr_m_set;
#[doc = "INTR_M_MASK (rw) register accessor: Master interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_m_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_mask`]
module"]
#[doc(alias = "INTR_M_MASK")]
pub type IntrMMask = crate::Reg<intr_m_mask::IntrMMaskSpec>;
#[doc = "Master interrupt mask"]
pub mod intr_m_mask;
#[doc = "INTR_M_MASKED (r) register accessor: Master interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_m_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_m_masked`]
module"]
#[doc(alias = "INTR_M_MASKED")]
pub type IntrMMasked = crate::Reg<intr_m_masked::IntrMMaskedSpec>;
#[doc = "Master interrupt masked request"]
pub mod intr_m_masked;
#[doc = "INTR_S (rw) register accessor: Slave interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s`]
module"]
#[doc(alias = "INTR_S")]
pub type IntrS = crate::Reg<intr_s::IntrSSpec>;
#[doc = "Slave interrupt request"]
pub mod intr_s;
#[doc = "INTR_S_SET (rw) register accessor: Slave interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_set`]
module"]
#[doc(alias = "INTR_S_SET")]
pub type IntrSSet = crate::Reg<intr_s_set::IntrSSetSpec>;
#[doc = "Slave interrupt set request"]
pub mod intr_s_set;
#[doc = "INTR_S_MASK (rw) register accessor: Slave interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_s_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_mask`]
module"]
#[doc(alias = "INTR_S_MASK")]
pub type IntrSMask = crate::Reg<intr_s_mask::IntrSMaskSpec>;
#[doc = "Slave interrupt mask"]
pub mod intr_s_mask;
#[doc = "INTR_S_MASKED (r) register accessor: Slave interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_s_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_s_masked`]
module"]
#[doc(alias = "INTR_S_MASKED")]
pub type IntrSMasked = crate::Reg<intr_s_masked::IntrSMaskedSpec>;
#[doc = "Slave interrupt masked request"]
pub mod intr_s_masked;
#[doc = "INTR_TX (rw) register accessor: Transmitter interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_tx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx`]
module"]
#[doc(alias = "INTR_TX")]
pub type IntrTx = crate::Reg<intr_tx::IntrTxSpec>;
#[doc = "Transmitter interrupt request"]
pub mod intr_tx;
#[doc = "INTR_TX_SET (rw) register accessor: Transmitter interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_tx_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_set`]
module"]
#[doc(alias = "INTR_TX_SET")]
pub type IntrTxSet = crate::Reg<intr_tx_set::IntrTxSetSpec>;
#[doc = "Transmitter interrupt set request"]
pub mod intr_tx_set;
#[doc = "INTR_TX_MASK (rw) register accessor: Transmitter interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_tx_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_mask`]
module"]
#[doc(alias = "INTR_TX_MASK")]
pub type IntrTxMask = crate::Reg<intr_tx_mask::IntrTxMaskSpec>;
#[doc = "Transmitter interrupt mask"]
pub mod intr_tx_mask;
#[doc = "INTR_TX_MASKED (r) register accessor: Transmitter interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_tx_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_tx_masked`]
module"]
#[doc(alias = "INTR_TX_MASKED")]
pub type IntrTxMasked = crate::Reg<intr_tx_masked::IntrTxMaskedSpec>;
#[doc = "Transmitter interrupt masked request"]
pub mod intr_tx_masked;
#[doc = "INTR_RX (rw) register accessor: Receiver interrupt request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_rx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx`]
module"]
#[doc(alias = "INTR_RX")]
pub type IntrRx = crate::Reg<intr_rx::IntrRxSpec>;
#[doc = "Receiver interrupt request"]
pub mod intr_rx;
#[doc = "INTR_RX_SET (rw) register accessor: Receiver interrupt set request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_rx_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_set`]
module"]
#[doc(alias = "INTR_RX_SET")]
pub type IntrRxSet = crate::Reg<intr_rx_set::IntrRxSetSpec>;
#[doc = "Receiver interrupt set request"]
pub mod intr_rx_set;
#[doc = "INTR_RX_MASK (rw) register accessor: Receiver interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_rx_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_mask`]
module"]
#[doc(alias = "INTR_RX_MASK")]
pub type IntrRxMask = crate::Reg<intr_rx_mask::IntrRxMaskSpec>;
#[doc = "Receiver interrupt mask"]
pub mod intr_rx_mask;
#[doc = "INTR_RX_MASKED (r) register accessor: Receiver interrupt masked request\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_rx_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_rx_masked`]
module"]
#[doc(alias = "INTR_RX_MASKED")]
pub type IntrRxMasked = crate::Reg<intr_rx_masked::IntrRxMaskedSpec>;
#[doc = "Receiver interrupt masked request"]
pub mod intr_rx_masked;
