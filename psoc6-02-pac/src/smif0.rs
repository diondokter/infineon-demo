#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    status: Status,
    _reserved2: [u8; 0x3c],
    tx_cmd_fifo_status: TxCmdFifoStatus,
    _reserved3: [u8; 0x08],
    tx_cmd_fifo_wr: TxCmdFifoWr,
    _reserved4: [u8; 0x2c],
    tx_data_fifo_ctl: TxDataFifoCtl,
    tx_data_fifo_status: TxDataFifoStatus,
    _reserved6: [u8; 0x08],
    tx_data_fifo_wr1: TxDataFifoWr1,
    tx_data_fifo_wr2: TxDataFifoWr2,
    tx_data_fifo_wr4: TxDataFifoWr4,
    _reserved9: [u8; 0x24],
    rx_data_fifo_ctl: RxDataFifoCtl,
    rx_data_fifo_status: RxDataFifoStatus,
    _reserved11: [u8; 0x08],
    rx_data_fifo_rd1: RxDataFifoRd1,
    rx_data_fifo_rd2: RxDataFifoRd2,
    rx_data_fifo_rd4: RxDataFifoRd4,
    _reserved14: [u8; 0x04],
    rx_data_fifo_rd1_silent: RxDataFifoRd1Silent,
    _reserved15: [u8; 0x1c],
    slow_ca_ctl: SlowCaCtl,
    _reserved16: [u8; 0x04],
    slow_ca_cmd: SlowCaCmd,
    _reserved17: [u8; 0x74],
    fast_ca_ctl: FastCaCtl,
    _reserved18: [u8; 0x04],
    fast_ca_cmd: FastCaCmd,
    _reserved19: [u8; 0x74],
    crypto_cmd: CryptoCmd,
    _reserved20: [u8; 0x1c],
    crypto_input0: CryptoInput0,
    crypto_input1: CryptoInput1,
    crypto_input2: CryptoInput2,
    crypto_input3: CryptoInput3,
    _reserved24: [u8; 0x10],
    crypto_key0: CryptoKey0,
    crypto_key1: CryptoKey1,
    crypto_key2: CryptoKey2,
    crypto_key3: CryptoKey3,
    _reserved28: [u8; 0x10],
    crypto_output0: CryptoOutput0,
    crypto_output1: CryptoOutput1,
    crypto_output2: CryptoOutput2,
    crypto_output3: CryptoOutput3,
    _reserved32: [u8; 0x0550],
    intr: Intr,
    intr_set: IntrSet,
    intr_mask: IntrMask,
    intr_masked: IntrMasked,
    _reserved36: [u8; 0x30],
    device: (),
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x44 - Transmitter command FIFO status"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_status(&self) -> &TxCmdFifoStatus {
        &self.tx_cmd_fifo_status
    }
    #[doc = "0x50 - Transmitter command FIFO write"]
    #[inline(always)]
    pub const fn tx_cmd_fifo_wr(&self) -> &TxCmdFifoWr {
        &self.tx_cmd_fifo_wr
    }
    #[doc = "0x80 - Transmitter data FIFO control"]
    #[inline(always)]
    pub const fn tx_data_fifo_ctl(&self) -> &TxDataFifoCtl {
        &self.tx_data_fifo_ctl
    }
    #[doc = "0x84 - Transmitter data FIFO status"]
    #[inline(always)]
    pub const fn tx_data_fifo_status(&self) -> &TxDataFifoStatus {
        &self.tx_data_fifo_status
    }
    #[doc = "0x90 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr1(&self) -> &TxDataFifoWr1 {
        &self.tx_data_fifo_wr1
    }
    #[doc = "0x94 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr2(&self) -> &TxDataFifoWr2 {
        &self.tx_data_fifo_wr2
    }
    #[doc = "0x98 - Transmitter data FIFO write"]
    #[inline(always)]
    pub const fn tx_data_fifo_wr4(&self) -> &TxDataFifoWr4 {
        &self.tx_data_fifo_wr4
    }
    #[doc = "0xc0 - Receiver data FIFO control"]
    #[inline(always)]
    pub const fn rx_data_fifo_ctl(&self) -> &RxDataFifoCtl {
        &self.rx_data_fifo_ctl
    }
    #[doc = "0xc4 - Receiver data FIFO status"]
    #[inline(always)]
    pub const fn rx_data_fifo_status(&self) -> &RxDataFifoStatus {
        &self.rx_data_fifo_status
    }
    #[doc = "0xd0 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1(&self) -> &RxDataFifoRd1 {
        &self.rx_data_fifo_rd1
    }
    #[doc = "0xd4 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd2(&self) -> &RxDataFifoRd2 {
        &self.rx_data_fifo_rd2
    }
    #[doc = "0xd8 - Receiver data FIFO read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd4(&self) -> &RxDataFifoRd4 {
        &self.rx_data_fifo_rd4
    }
    #[doc = "0xe0 - Receiver data FIFO silent read"]
    #[inline(always)]
    pub const fn rx_data_fifo_rd1_silent(&self) -> &RxDataFifoRd1Silent {
        &self.rx_data_fifo_rd1_silent
    }
    #[doc = "0x100 - Slow cache control"]
    #[inline(always)]
    pub const fn slow_ca_ctl(&self) -> &SlowCaCtl {
        &self.slow_ca_ctl
    }
    #[doc = "0x108 - Slow cache command"]
    #[inline(always)]
    pub const fn slow_ca_cmd(&self) -> &SlowCaCmd {
        &self.slow_ca_cmd
    }
    #[doc = "0x180 - Fast cache control"]
    #[inline(always)]
    pub const fn fast_ca_ctl(&self) -> &FastCaCtl {
        &self.fast_ca_ctl
    }
    #[doc = "0x188 - Fast cache command"]
    #[inline(always)]
    pub const fn fast_ca_cmd(&self) -> &FastCaCmd {
        &self.fast_ca_cmd
    }
    #[doc = "0x200 - Cryptography Command"]
    #[inline(always)]
    pub const fn crypto_cmd(&self) -> &CryptoCmd {
        &self.crypto_cmd
    }
    #[doc = "0x220 - Cryptography input 0"]
    #[inline(always)]
    pub const fn crypto_input0(&self) -> &CryptoInput0 {
        &self.crypto_input0
    }
    #[doc = "0x224 - Cryptography input 1"]
    #[inline(always)]
    pub const fn crypto_input1(&self) -> &CryptoInput1 {
        &self.crypto_input1
    }
    #[doc = "0x228 - Cryptography input 2"]
    #[inline(always)]
    pub const fn crypto_input2(&self) -> &CryptoInput2 {
        &self.crypto_input2
    }
    #[doc = "0x22c - Cryptography input 3"]
    #[inline(always)]
    pub const fn crypto_input3(&self) -> &CryptoInput3 {
        &self.crypto_input3
    }
    #[doc = "0x240 - Cryptography key 0"]
    #[inline(always)]
    pub const fn crypto_key0(&self) -> &CryptoKey0 {
        &self.crypto_key0
    }
    #[doc = "0x244 - Cryptography key 1"]
    #[inline(always)]
    pub const fn crypto_key1(&self) -> &CryptoKey1 {
        &self.crypto_key1
    }
    #[doc = "0x248 - Cryptography key 2"]
    #[inline(always)]
    pub const fn crypto_key2(&self) -> &CryptoKey2 {
        &self.crypto_key2
    }
    #[doc = "0x24c - Cryptography key 3"]
    #[inline(always)]
    pub const fn crypto_key3(&self) -> &CryptoKey3 {
        &self.crypto_key3
    }
    #[doc = "0x260 - Cryptography output 0"]
    #[inline(always)]
    pub const fn crypto_output0(&self) -> &CryptoOutput0 {
        &self.crypto_output0
    }
    #[doc = "0x264 - Cryptography output 1"]
    #[inline(always)]
    pub const fn crypto_output1(&self) -> &CryptoOutput1 {
        &self.crypto_output1
    }
    #[doc = "0x268 - Cryptography output 2"]
    #[inline(always)]
    pub const fn crypto_output2(&self) -> &CryptoOutput2 {
        &self.crypto_output2
    }
    #[doc = "0x26c - Cryptography output 3"]
    #[inline(always)]
    pub const fn crypto_output3(&self) -> &CryptoOutput3 {
        &self.crypto_output3
    }
    #[doc = "0x7c0 - Interrupt register"]
    #[inline(always)]
    pub const fn intr(&self) -> &Intr {
        &self.intr
    }
    #[doc = "0x7c4 - Interrupt set register"]
    #[inline(always)]
    pub const fn intr_set(&self) -> &IntrSet {
        &self.intr_set
    }
    #[doc = "0x7c8 - Interrupt mask register"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x7cc - Interrupt masked register"]
    #[inline(always)]
    pub const fn intr_masked(&self) -> &IntrMasked {
        &self.intr_masked
    }
    #[doc = "0x800..0x9d0 - Device (only used in XIP mode)"]
    #[inline(always)]
    pub const fn device(&self, n: usize) -> &Device {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(128 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x9d0 - Device (only used in XIP mode)"]
    #[inline(always)]
    pub fn device_iter(&self) -> impl Iterator<Item = &Device> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(2048)
                .add(128 * n)
                .cast()
        })
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "TX_CMD_FIFO_STATUS (r) register accessor: Transmitter command FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cmd_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_cmd_fifo_status`]
module"]
#[doc(alias = "TX_CMD_FIFO_STATUS")]
pub type TxCmdFifoStatus = crate::Reg<tx_cmd_fifo_status::TxCmdFifoStatusSpec>;
#[doc = "Transmitter command FIFO status"]
pub mod tx_cmd_fifo_status;
#[doc = "TX_CMD_FIFO_WR (w) register accessor: Transmitter command FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cmd_fifo_wr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_cmd_fifo_wr`]
module"]
#[doc(alias = "TX_CMD_FIFO_WR")]
pub type TxCmdFifoWr = crate::Reg<tx_cmd_fifo_wr::TxCmdFifoWrSpec>;
#[doc = "Transmitter command FIFO write"]
pub mod tx_cmd_fifo_wr;
#[doc = "TX_DATA_FIFO_CTL (rw) register accessor: Transmitter data FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_fifo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_ctl`]
module"]
#[doc(alias = "TX_DATA_FIFO_CTL")]
pub type TxDataFifoCtl = crate::Reg<tx_data_fifo_ctl::TxDataFifoCtlSpec>;
#[doc = "Transmitter data FIFO control"]
pub mod tx_data_fifo_ctl;
#[doc = "TX_DATA_FIFO_STATUS (r) register accessor: Transmitter data FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_status`]
module"]
#[doc(alias = "TX_DATA_FIFO_STATUS")]
pub type TxDataFifoStatus = crate::Reg<tx_data_fifo_status::TxDataFifoStatusSpec>;
#[doc = "Transmitter data FIFO status"]
pub mod tx_data_fifo_status;
#[doc = "TX_DATA_FIFO_WR1 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr1`]
module"]
#[doc(alias = "TX_DATA_FIFO_WR1")]
pub type TxDataFifoWr1 = crate::Reg<tx_data_fifo_wr1::TxDataFifoWr1Spec>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr1;
#[doc = "TX_DATA_FIFO_WR2 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr2`]
module"]
#[doc(alias = "TX_DATA_FIFO_WR2")]
pub type TxDataFifoWr2 = crate::Reg<tx_data_fifo_wr2::TxDataFifoWr2Spec>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr2;
#[doc = "TX_DATA_FIFO_WR4 (w) register accessor: Transmitter data FIFO write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_wr4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_data_fifo_wr4`]
module"]
#[doc(alias = "TX_DATA_FIFO_WR4")]
pub type TxDataFifoWr4 = crate::Reg<tx_data_fifo_wr4::TxDataFifoWr4Spec>;
#[doc = "Transmitter data FIFO write"]
pub mod tx_data_fifo_wr4;
#[doc = "RX_DATA_FIFO_CTL (rw) register accessor: Receiver data FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_data_fifo_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_ctl`]
module"]
#[doc(alias = "RX_DATA_FIFO_CTL")]
pub type RxDataFifoCtl = crate::Reg<rx_data_fifo_ctl::RxDataFifoCtlSpec>;
#[doc = "Receiver data FIFO control"]
pub mod rx_data_fifo_ctl;
#[doc = "RX_DATA_FIFO_STATUS (r) register accessor: Receiver data FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_status`]
module"]
#[doc(alias = "RX_DATA_FIFO_STATUS")]
pub type RxDataFifoStatus = crate::Reg<rx_data_fifo_status::RxDataFifoStatusSpec>;
#[doc = "Receiver data FIFO status"]
pub mod rx_data_fifo_status;
#[doc = "RX_DATA_FIFO_RD1 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd1`]
module"]
#[doc(alias = "RX_DATA_FIFO_RD1")]
pub type RxDataFifoRd1 = crate::Reg<rx_data_fifo_rd1::RxDataFifoRd1Spec>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd1;
#[doc = "RX_DATA_FIFO_RD2 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd2`]
module"]
#[doc(alias = "RX_DATA_FIFO_RD2")]
pub type RxDataFifoRd2 = crate::Reg<rx_data_fifo_rd2::RxDataFifoRd2Spec>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd2;
#[doc = "RX_DATA_FIFO_RD4 (r) register accessor: Receiver data FIFO read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd4`]
module"]
#[doc(alias = "RX_DATA_FIFO_RD4")]
pub type RxDataFifoRd4 = crate::Reg<rx_data_fifo_rd4::RxDataFifoRd4Spec>;
#[doc = "Receiver data FIFO read"]
pub mod rx_data_fifo_rd4;
#[doc = "RX_DATA_FIFO_RD1_SILENT (r) register accessor: Receiver data FIFO silent read\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_data_fifo_rd1_silent::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_data_fifo_rd1_silent`]
module"]
#[doc(alias = "RX_DATA_FIFO_RD1_SILENT")]
pub type RxDataFifoRd1Silent = crate::Reg<rx_data_fifo_rd1_silent::RxDataFifoRd1SilentSpec>;
#[doc = "Receiver data FIFO silent read"]
pub mod rx_data_fifo_rd1_silent;
#[doc = "SLOW_CA_CTL (rw) register accessor: Slow cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`slow_ca_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slow_ca_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slow_ca_ctl`]
module"]
#[doc(alias = "SLOW_CA_CTL")]
pub type SlowCaCtl = crate::Reg<slow_ca_ctl::SlowCaCtlSpec>;
#[doc = "Slow cache control"]
pub mod slow_ca_ctl;
#[doc = "SLOW_CA_CMD (rw) register accessor: Slow cache command\n\nYou can [`read`](crate::Reg::read) this register and get [`slow_ca_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slow_ca_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slow_ca_cmd`]
module"]
#[doc(alias = "SLOW_CA_CMD")]
pub type SlowCaCmd = crate::Reg<slow_ca_cmd::SlowCaCmdSpec>;
#[doc = "Slow cache command"]
pub mod slow_ca_cmd;
#[doc = "FAST_CA_CTL (rw) register accessor: Fast cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_ca_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_ca_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_ca_ctl`]
module"]
#[doc(alias = "FAST_CA_CTL")]
pub type FastCaCtl = crate::Reg<fast_ca_ctl::FastCaCtlSpec>;
#[doc = "Fast cache control"]
pub mod fast_ca_ctl;
#[doc = "FAST_CA_CMD (rw) register accessor: Fast cache command\n\nYou can [`read`](crate::Reg::read) this register and get [`fast_ca_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fast_ca_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fast_ca_cmd`]
module"]
#[doc(alias = "FAST_CA_CMD")]
pub type FastCaCmd = crate::Reg<fast_ca_cmd::FastCaCmdSpec>;
#[doc = "Fast cache command"]
pub mod fast_ca_cmd;
#[doc = "CRYPTO_CMD (rw) register accessor: Cryptography Command\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_cmd`]
module"]
#[doc(alias = "CRYPTO_CMD")]
pub type CryptoCmd = crate::Reg<crypto_cmd::CryptoCmdSpec>;
#[doc = "Cryptography Command"]
pub mod crypto_cmd;
#[doc = "CRYPTO_INPUT0 (rw) register accessor: Cryptography input 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_input0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_input0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input0`]
module"]
#[doc(alias = "CRYPTO_INPUT0")]
pub type CryptoInput0 = crate::Reg<crypto_input0::CryptoInput0Spec>;
#[doc = "Cryptography input 0"]
pub mod crypto_input0;
#[doc = "CRYPTO_INPUT1 (rw) register accessor: Cryptography input 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_input1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_input1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input1`]
module"]
#[doc(alias = "CRYPTO_INPUT1")]
pub type CryptoInput1 = crate::Reg<crypto_input1::CryptoInput1Spec>;
#[doc = "Cryptography input 1"]
pub mod crypto_input1;
#[doc = "CRYPTO_INPUT2 (rw) register accessor: Cryptography input 2\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_input2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_input2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input2`]
module"]
#[doc(alias = "CRYPTO_INPUT2")]
pub type CryptoInput2 = crate::Reg<crypto_input2::CryptoInput2Spec>;
#[doc = "Cryptography input 2"]
pub mod crypto_input2;
#[doc = "CRYPTO_INPUT3 (rw) register accessor: Cryptography input 3\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_input3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_input3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_input3`]
module"]
#[doc(alias = "CRYPTO_INPUT3")]
pub type CryptoInput3 = crate::Reg<crypto_input3::CryptoInput3Spec>;
#[doc = "Cryptography input 3"]
pub mod crypto_input3;
#[doc = "CRYPTO_KEY0 (w) register accessor: Cryptography key 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key0`]
module"]
#[doc(alias = "CRYPTO_KEY0")]
pub type CryptoKey0 = crate::Reg<crypto_key0::CryptoKey0Spec>;
#[doc = "Cryptography key 0"]
pub mod crypto_key0;
#[doc = "CRYPTO_KEY1 (w) register accessor: Cryptography key 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key1`]
module"]
#[doc(alias = "CRYPTO_KEY1")]
pub type CryptoKey1 = crate::Reg<crypto_key1::CryptoKey1Spec>;
#[doc = "Cryptography key 1"]
pub mod crypto_key1;
#[doc = "CRYPTO_KEY2 (w) register accessor: Cryptography key 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key2`]
module"]
#[doc(alias = "CRYPTO_KEY2")]
pub type CryptoKey2 = crate::Reg<crypto_key2::CryptoKey2Spec>;
#[doc = "Cryptography key 2"]
pub mod crypto_key2;
#[doc = "CRYPTO_KEY3 (w) register accessor: Cryptography key 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_key3`]
module"]
#[doc(alias = "CRYPTO_KEY3")]
pub type CryptoKey3 = crate::Reg<crypto_key3::CryptoKey3Spec>;
#[doc = "Cryptography key 3"]
pub mod crypto_key3;
#[doc = "CRYPTO_OUTPUT0 (rw) register accessor: Cryptography output 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_output0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_output0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output0`]
module"]
#[doc(alias = "CRYPTO_OUTPUT0")]
pub type CryptoOutput0 = crate::Reg<crypto_output0::CryptoOutput0Spec>;
#[doc = "Cryptography output 0"]
pub mod crypto_output0;
#[doc = "CRYPTO_OUTPUT1 (rw) register accessor: Cryptography output 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_output1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_output1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output1`]
module"]
#[doc(alias = "CRYPTO_OUTPUT1")]
pub type CryptoOutput1 = crate::Reg<crypto_output1::CryptoOutput1Spec>;
#[doc = "Cryptography output 1"]
pub mod crypto_output1;
#[doc = "CRYPTO_OUTPUT2 (rw) register accessor: Cryptography output 2\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_output2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_output2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output2`]
module"]
#[doc(alias = "CRYPTO_OUTPUT2")]
pub type CryptoOutput2 = crate::Reg<crypto_output2::CryptoOutput2Spec>;
#[doc = "Cryptography output 2"]
pub mod crypto_output2;
#[doc = "CRYPTO_OUTPUT3 (rw) register accessor: Cryptography output 3\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_output3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_output3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crypto_output3`]
module"]
#[doc(alias = "CRYPTO_OUTPUT3")]
pub type CryptoOutput3 = crate::Reg<crypto_output3::CryptoOutput3Spec>;
#[doc = "Cryptography output 3"]
pub mod crypto_output3;
#[doc = "INTR (rw) register accessor: Interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr`]
module"]
#[doc(alias = "INTR")]
pub type Intr = crate::Reg<intr::IntrSpec>;
#[doc = "Interrupt register"]
pub mod intr;
#[doc = "INTR_SET (rw) register accessor: Interrupt set register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_set`]
module"]
#[doc(alias = "INTR_SET")]
pub type IntrSet = crate::Reg<intr_set::IntrSetSpec>;
#[doc = "Interrupt set register"]
pub mod intr_set;
#[doc = "INTR_MASK (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "INTR_MASK")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "Interrupt mask register"]
pub mod intr_mask;
#[doc = "INTR_MASKED (r) register accessor: Interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_masked`]
module"]
#[doc(alias = "INTR_MASKED")]
pub type IntrMasked = crate::Reg<intr_masked::IntrMaskedSpec>;
#[doc = "Interrupt masked register"]
pub mod intr_masked;
#[doc = "Device (only used in XIP mode)"]
pub use self::device::Device;
#[doc = r"Cluster"]
#[doc = "Device (only used in XIP mode)"]
pub mod device;
