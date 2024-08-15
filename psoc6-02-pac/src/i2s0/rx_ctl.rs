#[doc = "Register `RX_CTL` reader"]
pub type R = crate::R<RxCtlSpec>;
#[doc = "Register `RX_CTL` writer"]
pub type W = crate::W<RxCtlSpec>;
#[doc = "Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BClockInv {
    #[doc = "0: SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    RisingEdgeRx = 0,
    #[doc = "1: SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    FallingEdgeRx = 1,
}
impl From<BClockInv> for bool {
    #[inline(always)]
    fn from(variant: BClockInv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B_CLOCK_INV` reader - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
pub type BClockInvR = crate::BitReader<BClockInv>;
impl BClockInvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BClockInv {
        match self.bits {
            false => BClockInv::RisingEdgeRx,
            true => BClockInv::FallingEdgeRx,
        }
    }
    #[doc = "SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn is_rising_edge_rx(&self) -> bool {
        *self == BClockInv::RisingEdgeRx
    }
    #[doc = "SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn is_falling_edge_rx(&self) -> bool {
        *self == BClockInv::FallingEdgeRx
    }
}
#[doc = "Field `B_CLOCK_INV` writer - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
pub type BClockInvW<'a, REG> = crate::BitWriter<'a, REG, BClockInv>;
impl<'a, REG> BClockInvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDI received at SCK rising edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn rising_edge_rx(self) -> &'a mut crate::W<REG> {
        self.variant(BClockInv::RisingEdgeRx)
    }
    #[doc = "SDI received at SCK falling edge when RX_CTL.SCKO_POL=0"]
    #[inline(always)]
    pub fn falling_edge_rx(self) -> &'a mut crate::W<REG> {
        self.variant(BClockInv::FallingEdgeRx)
    }
}
#[doc = "Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChNr {
    #[doc = "0: 1 channel"]
    ChNum1 = 0,
    #[doc = "1: 2 channels"]
    ChNum2 = 1,
    #[doc = "2: 3 channels"]
    ChNum3 = 2,
    #[doc = "3: 4 channels"]
    ChNum4 = 3,
    #[doc = "4: 5 channels"]
    ChNum5 = 4,
    #[doc = "5: 6 channels"]
    ChNum6 = 5,
    #[doc = "6: 7 channels"]
    ChNum7 = 6,
    #[doc = "7: 8 channels"]
    ChNum8 = 7,
}
impl From<ChNr> for u8 {
    #[inline(always)]
    fn from(variant: ChNr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChNr {
    type Ux = u8;
}
impl crate::IsEnum for ChNr {}
#[doc = "Field `CH_NR` reader - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
pub type ChNrR = crate::FieldReader<ChNr>;
impl ChNrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ChNr {
        match self.bits {
            0 => ChNr::ChNum1,
            1 => ChNr::ChNum2,
            2 => ChNr::ChNum3,
            3 => ChNr::ChNum4,
            4 => ChNr::ChNum5,
            5 => ChNr::ChNum6,
            6 => ChNr::ChNum7,
            7 => ChNr::ChNum8,
            _ => unreachable!(),
        }
    }
    #[doc = "1 channel"]
    #[inline(always)]
    pub fn is_ch_num1(&self) -> bool {
        *self == ChNr::ChNum1
    }
    #[doc = "2 channels"]
    #[inline(always)]
    pub fn is_ch_num2(&self) -> bool {
        *self == ChNr::ChNum2
    }
    #[doc = "3 channels"]
    #[inline(always)]
    pub fn is_ch_num3(&self) -> bool {
        *self == ChNr::ChNum3
    }
    #[doc = "4 channels"]
    #[inline(always)]
    pub fn is_ch_num4(&self) -> bool {
        *self == ChNr::ChNum4
    }
    #[doc = "5 channels"]
    #[inline(always)]
    pub fn is_ch_num5(&self) -> bool {
        *self == ChNr::ChNum5
    }
    #[doc = "6 channels"]
    #[inline(always)]
    pub fn is_ch_num6(&self) -> bool {
        *self == ChNr::ChNum6
    }
    #[doc = "7 channels"]
    #[inline(always)]
    pub fn is_ch_num7(&self) -> bool {
        *self == ChNr::ChNum7
    }
    #[doc = "8 channels"]
    #[inline(always)]
    pub fn is_ch_num8(&self) -> bool {
        *self == ChNr::ChNum8
    }
}
#[doc = "Field `CH_NR` writer - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
pub type ChNrW<'a, REG> = crate::FieldWriter<'a, REG, 3, ChNr, crate::Safe>;
impl<'a, REG> ChNrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 channel"]
    #[inline(always)]
    pub fn ch_num1(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum1)
    }
    #[doc = "2 channels"]
    #[inline(always)]
    pub fn ch_num2(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum2)
    }
    #[doc = "3 channels"]
    #[inline(always)]
    pub fn ch_num3(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum3)
    }
    #[doc = "4 channels"]
    #[inline(always)]
    pub fn ch_num4(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum4)
    }
    #[doc = "5 channels"]
    #[inline(always)]
    pub fn ch_num5(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum5)
    }
    #[doc = "6 channels"]
    #[inline(always)]
    pub fn ch_num6(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum6)
    }
    #[doc = "7 channels"]
    #[inline(always)]
    pub fn ch_num7(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum7)
    }
    #[doc = "8 channels"]
    #[inline(always)]
    pub fn ch_num8(self) -> &'a mut crate::W<REG> {
        self.variant(ChNr::ChNum8)
    }
}
#[doc = "Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ms {
    #[doc = "0: Slave"]
    Slave = 0,
    #[doc = "1: Master"]
    Master = 1,
}
impl From<Ms> for bool {
    #[inline(always)]
    fn from(variant: Ms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MS` reader - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
pub type MsR = crate::BitReader<Ms>;
impl MsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ms {
        match self.bits {
            false => Ms::Slave,
            true => Ms::Master,
        }
    }
    #[doc = "Slave"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Ms::Slave
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Ms::Master
    }
}
#[doc = "Field `MS` writer - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG, Ms>;
impl<'a, REG> MsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Slave)
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Ms::Master)
    }
}
#[doc = "Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2sMode {
    #[doc = "0: Left Justified"]
    LeftJustified = 0,
    #[doc = "1: I2S mode"]
    I2s = 1,
    #[doc = "2: TDM mode A, the 1st Channel align to WSO Rising Edge"]
    TdmA = 2,
    #[doc = "3: TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    TdmB = 3,
}
impl From<I2sMode> for u8 {
    #[inline(always)]
    fn from(variant: I2sMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2sMode {
    type Ux = u8;
}
impl crate::IsEnum for I2sMode {}
#[doc = "Field `I2S_MODE` reader - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
pub type I2sModeR = crate::FieldReader<I2sMode>;
impl I2sModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2sMode {
        match self.bits {
            0 => I2sMode::LeftJustified,
            1 => I2sMode::I2s,
            2 => I2sMode::TdmA,
            3 => I2sMode::TdmB,
            _ => unreachable!(),
        }
    }
    #[doc = "Left Justified"]
    #[inline(always)]
    pub fn is_left_justified(&self) -> bool {
        *self == I2sMode::LeftJustified
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn is_i2s(&self) -> bool {
        *self == I2sMode::I2s
    }
    #[doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"]
    #[inline(always)]
    pub fn is_tdm_a(&self) -> bool {
        *self == I2sMode::TdmA
    }
    #[doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    #[inline(always)]
    pub fn is_tdm_b(&self) -> bool {
        *self == I2sMode::TdmB
    }
}
#[doc = "Field `I2S_MODE` writer - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
pub type I2sModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, I2sMode, crate::Safe>;
impl<'a, REG> I2sModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Left Justified"]
    #[inline(always)]
    pub fn left_justified(self) -> &'a mut crate::W<REG> {
        self.variant(I2sMode::LeftJustified)
    }
    #[doc = "I2S mode"]
    #[inline(always)]
    pub fn i2s(self) -> &'a mut crate::W<REG> {
        self.variant(I2sMode::I2s)
    }
    #[doc = "TDM mode A, the 1st Channel align to WSO Rising Edge"]
    #[inline(always)]
    pub fn tdm_a(self) -> &'a mut crate::W<REG> {
        self.variant(I2sMode::TdmA)
    }
    #[doc = "TDM mode B, the 1st Channel align to WSO Rising edge with1 SCK Delay"]
    #[inline(always)]
    pub fn tdm_b(self) -> &'a mut crate::W<REG> {
        self.variant(I2sMode::TdmB)
    }
}
#[doc = "Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WsPulse {
    #[doc = "0: Pulse width is 1 SCK period"]
    SckPeriod = 0,
    #[doc = "1: Pulse width is 1 channel length"]
    ChLength = 1,
}
impl From<WsPulse> for bool {
    #[inline(always)]
    fn from(variant: WsPulse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WS_PULSE` reader - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
pub type WsPulseR = crate::BitReader<WsPulse>;
impl WsPulseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WsPulse {
        match self.bits {
            false => WsPulse::SckPeriod,
            true => WsPulse::ChLength,
        }
    }
    #[doc = "Pulse width is 1 SCK period"]
    #[inline(always)]
    pub fn is_sck_period(&self) -> bool {
        *self == WsPulse::SckPeriod
    }
    #[doc = "Pulse width is 1 channel length"]
    #[inline(always)]
    pub fn is_ch_length(&self) -> bool {
        *self == WsPulse::ChLength
    }
}
#[doc = "Field `WS_PULSE` writer - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
pub type WsPulseW<'a, REG> = crate::BitWriter<'a, REG, WsPulse>;
impl<'a, REG> WsPulseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pulse width is 1 SCK period"]
    #[inline(always)]
    pub fn sck_period(self) -> &'a mut crate::W<REG> {
        self.variant(WsPulse::SckPeriod)
    }
    #[doc = "Pulse width is 1 channel length"]
    #[inline(always)]
    pub fn ch_length(self) -> &'a mut crate::W<REG> {
        self.variant(WsPulse::ChLength)
    }
}
#[doc = "Field `WD_EN` reader - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
pub type WdEnR = crate::BitReader;
#[doc = "Field `WD_EN` writer - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
pub type WdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChLen {
    #[doc = "0: 8-bit"]
    BitLen8 = 0,
    #[doc = "1: 16-bit"]
    BitLen16 = 1,
    #[doc = "2: 18-bit"]
    BitLen18 = 2,
    #[doc = "3: 20-bit"]
    BitLen20 = 3,
    #[doc = "4: 24-bit"]
    BitLen24 = 4,
    #[doc = "5: 32-bit"]
    BitLen32 = 5,
}
impl From<ChLen> for u8 {
    #[inline(always)]
    fn from(variant: ChLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ChLen {
    type Ux = u8;
}
impl crate::IsEnum for ChLen {}
#[doc = "Field `CH_LEN` reader - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
pub type ChLenR = crate::FieldReader<ChLen>;
impl ChLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ChLen> {
        match self.bits {
            0 => Some(ChLen::BitLen8),
            1 => Some(ChLen::BitLen16),
            2 => Some(ChLen::BitLen18),
            3 => Some(ChLen::BitLen20),
            4 => Some(ChLen::BitLen24),
            5 => Some(ChLen::BitLen32),
            _ => None,
        }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_bit_len8(&self) -> bool {
        *self == ChLen::BitLen8
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == ChLen::BitLen16
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == ChLen::BitLen18
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == ChLen::BitLen20
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == ChLen::BitLen24
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_bit_len32(&self) -> bool {
        *self == ChLen::BitLen32
    }
}
#[doc = "Field `CH_LEN` writer - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
pub type ChLenW<'a, REG> = crate::FieldWriter<'a, REG, 3, ChLen>;
impl<'a, REG> ChLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bit_len8(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen8)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen24)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bit_len32(self) -> &'a mut crate::W<REG> {
        self.variant(ChLen::BitLen32)
    }
}
#[doc = "Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WordLen {
    #[doc = "0: 8-bit"]
    BitLen8 = 0,
    #[doc = "1: 16-bit"]
    BitLen16 = 1,
    #[doc = "2: 18-bit"]
    BitLen18 = 2,
    #[doc = "3: 20-bit"]
    BitLen20 = 3,
    #[doc = "4: 24-bit"]
    BitLen24 = 4,
    #[doc = "5: 32-bit"]
    BitLen32 = 5,
}
impl From<WordLen> for u8 {
    #[inline(always)]
    fn from(variant: WordLen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WordLen {
    type Ux = u8;
}
impl crate::IsEnum for WordLen {}
#[doc = "Field `WORD_LEN` reader - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
pub type WordLenR = crate::FieldReader<WordLen>;
impl WordLenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<WordLen> {
        match self.bits {
            0 => Some(WordLen::BitLen8),
            1 => Some(WordLen::BitLen16),
            2 => Some(WordLen::BitLen18),
            3 => Some(WordLen::BitLen20),
            4 => Some(WordLen::BitLen24),
            5 => Some(WordLen::BitLen32),
            _ => None,
        }
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_bit_len8(&self) -> bool {
        *self == WordLen::BitLen8
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_bit_len16(&self) -> bool {
        *self == WordLen::BitLen16
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn is_bit_len18(&self) -> bool {
        *self == WordLen::BitLen18
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn is_bit_len20(&self) -> bool {
        *self == WordLen::BitLen20
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn is_bit_len24(&self) -> bool {
        *self == WordLen::BitLen24
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_bit_len32(&self) -> bool {
        *self == WordLen::BitLen32
    }
}
#[doc = "Field `WORD_LEN` writer - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
pub type WordLenW<'a, REG> = crate::FieldWriter<'a, REG, 3, WordLen>;
impl<'a, REG> WordLenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bit_len8(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen8)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn bit_len16(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen16)
    }
    #[doc = "18-bit"]
    #[inline(always)]
    pub fn bit_len18(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen18)
    }
    #[doc = "20-bit"]
    #[inline(always)]
    pub fn bit_len20(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen20)
    }
    #[doc = "24-bit"]
    #[inline(always)]
    pub fn bit_len24(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen24)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn bit_len32(self) -> &'a mut crate::W<REG> {
        self.variant(WordLen::BitLen32)
    }
}
#[doc = "Field `BIT_EXTENSION` reader - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BitExtensionR = crate::BitReader;
#[doc = "Field `BIT_EXTENSION` writer - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
pub type BitExtensionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKO_POL` reader - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
pub type SckoPolR = crate::BitReader;
#[doc = "Field `SCKO_POL` writer - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
pub type SckoPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCKI_POL` reader - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
pub type SckiPolR = crate::BitReader;
#[doc = "Field `SCKI_POL` writer - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
pub type SckiPolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    pub fn b_clock_inv(&self) -> BClockInvR {
        BClockInvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    pub fn ch_nr(&self) -> ChNrR {
        ChNrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    pub fn i2s_mode(&self) -> I2sModeR {
        I2sModeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    pub fn ws_pulse(&self) -> WsPulseR {
        WsPulseR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn wd_en(&self) -> WdEnR {
        WdEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    pub fn ch_len(&self) -> ChLenR {
        ChLenR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    pub fn word_len(&self) -> WordLenR {
        WordLenR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    pub fn bit_extension(&self) -> BitExtensionR {
        BitExtensionR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    pub fn scko_pol(&self) -> SckoPolR {
        SckoPolR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    pub fn scki_pol(&self) -> SckiPolR {
        SckiPolR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Serial data capture is delayed by 0.5 SCK cycles. This bit is valid only in RX master mode. When set to '1', the serial data will be captured 0.5 SCK cycles later than when set to '0'. 1) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK rising edge 2) RX_CTL.SCKO_POL=0 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK falling edge that is 0.5 SCK cycles after the SCK rising edge in 1) 3) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=0: Serial data will be captured by the SCK falling edge 4) RX_CTL.SCKO_POL=1 and RX_CTL.B_CLOCK_INV=1: Serial data will be captured by the SCK rising edge that is 0.5 SCK cycles after the SCK falling edge in 3) (Note that this is only the appearance w.r.t. SCK edge, the actual capture timing is derived from an internal clock that runs 8x the SCK frequency). The word sync (RX_WS) signal is not affected by this bit setting. Note: When Slave mode, must be '0'. (Note: This bit is connected to AR38U12.TX_CFG.RX_BCLKINV)"]
    #[inline(always)]
    #[must_use]
    pub fn b_clock_inv(&mut self) -> BClockInvW<RxCtlSpec> {
        BClockInvW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Specifies number of channels per frame: Note: only '2channels' is supported during Left Justfied or I2S mode. Hence software must set '1' to this field in the modes. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHSET)"]
    #[inline(always)]
    #[must_use]
    pub fn ch_nr(&mut self) -> ChNrW<RxCtlSpec> {
        ChNrW::new(self, 4)
    }
    #[doc = "Bit 7 - Set interface in master or slave mode: (Note: This bit is connected to AR38U12.TX_CFG.RX_MS)"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<RxCtlSpec> {
        MsW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Select I2S, left-justified or TDM: (Note: These bits are connected to AR38U12.RX_CFG.RX_I2S_MODE)"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_mode(&mut self) -> I2sModeW<RxCtlSpec> {
        I2sModeW::new(self, 8)
    }
    #[doc = "Bit 10 - Set WS pulse width in TDM mode: (Note: This bit is connected to AR38U12.RX_CFG.RX_WS_PULSE) Note: When not TDM mode, must be '1'."]
    #[inline(always)]
    #[must_use]
    pub fn ws_pulse(&mut self) -> WsPulseW<RxCtlSpec> {
        WsPulseW::new(self, 10)
    }
    #[doc = "Bit 13 - Set watchdog for 'rx_ws_in' '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn wd_en(&mut self) -> WdEnW<RxCtlSpec> {
        WdEnW::new(self, 13)
    }
    #[doc = "Bits 16:18 - Channel length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - When TDM mode, must be 32-bit length to this field. (Note: These bits are connected to AR38U12.RX_CFG.RX_CHLEN)"]
    #[inline(always)]
    #[must_use]
    pub fn ch_len(&mut self) -> ChLenW<RxCtlSpec> {
        ChLenW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Word length in number of bits: Note: - When this field is configured to '6' or '7', the length is set to 32-bit (same as '5'). - Don't configure this field as beyond Channel length. (Note: These bits are connected to AR38U12.RX_CFG.RX_IWL)"]
    #[inline(always)]
    #[must_use]
    pub fn word_len(&mut self) -> WordLenW<RxCtlSpec> {
        WordLenW::new(self, 20)
    }
    #[doc = "Bit 23 - When reception word length is shorter than the word length of RX_FIFO_RD, extension mode of upper bit should be set. '0': Extended by '0' '1': Extended by sign bit (if MSB word is '1', then it is extended by '1', if MSB is '0' then it is extended by '0')"]
    #[inline(always)]
    #[must_use]
    pub fn bit_extension(&mut self) -> BitExtensionW<RxCtlSpec> {
        BitExtensionW::new(self, 23)
    }
    #[doc = "Bit 24 - RX master bit clock polarity. When this bit is 1, the outgoing rx_sck signal is inverted after it has been transmitted from the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting.See RX_CTL.B_CLOCK_INV for more details."]
    #[inline(always)]
    #[must_use]
    pub fn scko_pol(&mut self) -> SckoPolW<RxCtlSpec> {
        SckoPolW::new(self, 24)
    }
    #[doc = "Bit 25 - RX slave bit clock polarity. When this bit is 1, the incoming rx_sck signal is inverted before it is received by the I2S receiver core. This bit does not affect the internal serial data capture timing. The word sync (RX_WS) signal is not affected by this bit setting. '0': When receiver is in slave mode, serial data is sampled on the rising bit clock edge '1': When receiver is in slave mode, serial data is sampled on the falling bit clock edge"]
    #[inline(always)]
    #[must_use]
    pub fn scki_pol(&mut self) -> SckiPolW<RxCtlSpec> {
        SckiPolW::new(self, 25)
    }
}
#[doc = "Receiver control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCtlSpec;
impl crate::RegisterSpec for RxCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctl::R`](R) reader structure"]
impl crate::Readable for RxCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_ctl::W`](W) writer structure"]
impl crate::Writable for RxCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CTL to value 0x0044_0510"]
impl crate::Resettable for RxCtlSpec {
    const RESET_VALUE: u32 = 0x0044_0510;
}
