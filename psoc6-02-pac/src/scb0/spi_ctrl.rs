#[doc = "Register `SPI_CTRL` reader"]
pub type R = crate::R<SpiCtrlSpec>;
#[doc = "Register `SPI_CTRL` writer"]
pub type W = crate::W<SpiCtrlSpec>;
#[doc = "Field `SSEL_CONTINUOUS` reader - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames."]
pub type SselContinuousR = crate::BitReader;
#[doc = "Field `SSEL_CONTINUOUS` writer - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames."]
pub type SselContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELECT_PRECEDE` reader - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SelectPrecedeR = crate::BitReader;
#[doc = "Field `SELECT_PRECEDE` writer - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
pub type SelectPrecedeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. In SPI Motorola submode, all four CPOL/CPHA modes are valid. in SPI NS submode, only CPOL=0 CPHA=0 mode is valid. in SPI TI submode, only CPOL=0 CPHA=1 mode is valid."]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. In SPI Motorola submode, all four CPOL/CPHA modes are valid. in SPI NS submode, only CPOL=0 CPHA=0 mode is valid. in SPI TI submode, only CPOL=0 CPHA=1 mode is valid."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Indicates the clock polarity. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Indicates the clock polarity. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LATE_MISO_SAMPLE` reader - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LateMisoSampleR = crate::BitReader;
#[doc = "Field `LATE_MISO_SAMPLE` writer - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
pub type LateMisoSampleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLK_CONTINUOUS` reader - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
pub type SclkContinuousR = crate::BitReader;
#[doc = "Field `SCLK_CONTINUOUS` writer - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
pub type SclkContinuousW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY0` reader - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
pub type SselPolarity0R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY0` writer - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
pub type SselPolarity0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY1` reader - Slave select polarity."]
pub type SselPolarity1R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY1` writer - Slave select polarity."]
pub type SselPolarity1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY2` reader - Slave select polarity."]
pub type SselPolarity2R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY2` writer - Slave select polarity."]
pub type SselPolarity2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEL_POLARITY3` reader - Slave select polarity."]
pub type SselPolarity3R = crate::BitReader;
#[doc = "Field `SSEL_POLARITY3` writer - Slave select polarity."]
pub type SselPolarity3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SpiMotorola = 0,
    #[doc = "1: SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    SpiTi = 1,
    #[doc = "2: SPI National Semiconductors submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    SpiNs = 2,
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
            0 => Some(Mode::SpiMotorola),
            1 => Some(Mode::SpiTi),
            2 => Some(Mode::SpiNs),
            _ => None,
        }
    }
    #[doc = "SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn is_spi_motorola(&self) -> bool {
        *self == Mode::SpiMotorola
    }
    #[doc = "SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    #[inline(always)]
    pub fn is_spi_ti(&self) -> bool {
        *self == Mode::SpiTi
    }
    #[doc = "SPI National Semiconductors submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn is_spi_ns(&self) -> bool {
        *self == Mode::SpiNs
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI Motorola submode. In master mode, when not transmitting data (SELECT is inactive), SCLK is stable at CPOL. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn spi_motorola(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SpiMotorola)
    }
    #[doc = "SPI Texas Instruments submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive; i.e. no pulse is generated."]
    #[inline(always)]
    pub fn spi_ti(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SpiTi)
    }
    #[doc = "SPI National Semiconductors submode. In master mode, when not transmitting data, SCLK is stable at '0'. In slave mode, when not selected, SCLK is ignored; i.e. it can be either stable or clocking. In master mode, when there is no data to transmit (TX FIFO is empty), SELECT is inactive."]
    #[inline(always)]
    pub fn spi_ns(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SpiNs)
    }
}
#[doc = "Field `SSEL` reader - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The IP should be disabled when changes are made to this field."]
pub type SselR = crate::FieldReader;
#[doc = "Field `SSEL` writer - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The IP should be disabled when changes are made to this field."]
pub type SselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MASTER_MODE` reader - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
pub type MasterModeR = crate::BitReader;
#[doc = "Field `MASTER_MODE` writer - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
pub type MasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames."]
    #[inline(always)]
    pub fn ssel_continuous(&self) -> SselContinuousR {
        SselContinuousR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    pub fn select_precede(&self) -> SelectPrecedeR {
        SelectPrecedeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. In SPI Motorola submode, all four CPOL/CPHA modes are valid. in SPI NS submode, only CPOL=0 CPHA=0 mode is valid. in SPI TI submode, only CPOL=0 CPHA=1 mode is valid."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates the clock polarity. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    pub fn late_miso_sample(&self) -> LateMisoSampleR {
        LateMisoSampleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    pub fn sclk_continuous(&self) -> SclkContinuousR {
        SclkContinuousR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    pub fn ssel_polarity0(&self) -> SselPolarity0R {
        SselPolarity0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Slave select polarity."]
    #[inline(always)]
    pub fn ssel_polarity1(&self) -> SselPolarity1R {
        SselPolarity1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave select polarity."]
    #[inline(always)]
    pub fn ssel_polarity2(&self) -> SselPolarity2R {
        SselPolarity2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Slave select polarity."]
    #[inline(always)]
    pub fn ssel_polarity3(&self) -> SselPolarity3R {
        SselPolarity3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The IP should be disabled when changes are made to this field."]
    #[inline(always)]
    pub fn ssel(&self) -> SselR {
        SselR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
    #[inline(always)]
    pub fn master_mode(&self) -> MasterModeR {
        MasterModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Continuous SPI data transfers enabled ('1') or not ('0'). This field is used in master mode. In slave mode, both continuous and non-continuous SPI data transfers are supported independent of this field. When continuous transfers are enabled individual data frame transfers are not necessarily separated by slave deselection (as indicated by the level or pulse on the SELECT line): if the TX FIFO has multiple data frames, data frames are send out without slave deselection. When continuous transfers are not enabled individual data frame transfers are always separated by slave deselection: independent of the availability of TX FIFO data frames."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_continuous(&mut self) -> SselContinuousW<SpiCtrlSpec> {
        SselContinuousW::new(self, 0)
    }
    #[doc = "Bit 1 - Only used in SPI Texas Instruments' submode. When '1', the data frame start indication is a pulse on the SELECT line that precedes the transfer of the first data frame bit. When '0', the data frame start indication is a pulse on the SELECT line that coincides with the transfer of the first data frame bit."]
    #[inline(always)]
    #[must_use]
    pub fn select_precede(&mut self) -> SelectPrecedeW<SpiCtrlSpec> {
        SelectPrecedeW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates the clock phase. This field, together with the CPOL field, indicates when MOSI data is driven and MISO data is captured: - Motorola mode 0. CPOL is '0', CPHA is '0': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. - Motorola mode 1. CPOL is '0', CPHA is '1': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 2. CPOL is '1', CPHA is '0': MOSI is driven on a rising edge of SCLK. MISO is captured on a falling edge of SCLK. - Motorola mode 3. CPOL is '1', CPHA is '1': MOSI is driven on a falling edge of SCLK. MISO is captured on a rising edge of SCLK. In SPI Motorola submode, all four CPOL/CPHA modes are valid. in SPI NS submode, only CPOL=0 CPHA=0 mode is valid. in SPI TI submode, only CPOL=0 CPHA=1 mode is valid."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<SpiCtrlSpec> {
        CphaW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates the clock polarity. This field, together with the CPHA field, indicates when MOSI data is driven and MISO data is captured: - CPOL is '0': SCLK is '0' when not transmitting data. - CPOL is '1': SCLK is '1' when not transmitting data."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<SpiCtrlSpec> {
        CpolW::new(self, 3)
    }
    #[doc = "Bit 4 - Changes the SCLK edge on which MISO is captured. Only used in master mode. When '0', the default applies (for Motorola as determined by CPOL and CPHA, for Texas Instruments on the falling edge of SCLK and for National Semiconductors on the rising edge of SCLK). When '1', the alternate clock edge is used (which comes half a SPI SCLK period later). Late sampling addresses the round trip delay associated with transmitting SCLK from the master to the slave and transmitting MISO from the slave to the master."]
    #[inline(always)]
    #[must_use]
    pub fn late_miso_sample(&mut self) -> LateMisoSampleW<SpiCtrlSpec> {
        LateMisoSampleW::new(self, 4)
    }
    #[doc = "Bit 5 - Only applicable in master mode. '0': SCLK is generated, when the SPI master is enabled and data is transmitted. '1': SCLK is generated, when the SPI master is enabled. This mode is useful for slave devices that use SCLK for functional operation other than just SPI functionality."]
    #[inline(always)]
    #[must_use]
    pub fn sclk_continuous(&mut self) -> SclkContinuousW<SpiCtrlSpec> {
        SclkContinuousW::new(self, 5)
    }
    #[doc = "Bit 8 - Slave select polarity. SSEL_POLARITY0 applies to the outgoing SPI slave select signal 0 (master mode) and to the incoming SPI slave select signal (slave mode). For Motorola and National Semiconductors submodes: '0': slave select is low/'0' active. '1': slave select is high/'1' active. For Texas Instruments submode: '0': high/'1' active precede/coincide pulse. '1': low/'0' active precede/coincide pulse."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity0(&mut self) -> SselPolarity0W<SpiCtrlSpec> {
        SselPolarity0W::new(self, 8)
    }
    #[doc = "Bit 9 - Slave select polarity."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity1(&mut self) -> SselPolarity1W<SpiCtrlSpec> {
        SselPolarity1W::new(self, 9)
    }
    #[doc = "Bit 10 - Slave select polarity."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity2(&mut self) -> SselPolarity2W<SpiCtrlSpec> {
        SselPolarity2W::new(self, 10)
    }
    #[doc = "Bit 11 - Slave select polarity."]
    #[inline(always)]
    #[must_use]
    pub fn ssel_polarity3(&mut self) -> SselPolarity3W<SpiCtrlSpec> {
        SselPolarity3W::new(self, 11)
    }
    #[doc = "Bit 16 - Local loopback control (does NOT affect the information on the pins). Only used in master mode. Not used in National Semiconductors submode. '0': the SPI master MISO line 'spi_miso_in' is connected to the SPI MISO pin. '1': the SPI master MISO line 'spi_miso_in' is connected to the SPI master MOSI line 'spi_mosi_out'. In other words, in loopback mode the SPI master receives on MISO what it transmits on MOSI."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<SpiCtrlSpec> {
        LoopbackW::new(self, 16)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<SpiCtrlSpec> {
        ModeW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Selects one of the four incoming/outgoing SPI slave select signals: - 0: Slave 0, SSEL\\[0\\]. - 1: Slave 1, SSEL\\[1\\]. - 2: Slave 2, SSEL\\[2\\]. - 3: Slave 3, SSEL\\[3\\]. The IP should be disabled when changes are made to this field."]
    #[inline(always)]
    #[must_use]
    pub fn ssel(&mut self) -> SselW<SpiCtrlSpec> {
        SselW::new(self, 26)
    }
    #[doc = "Bit 31 - Master ('1') or slave ('0') mode. In master mode, transmission will commence on availability of data frames in the TX FIFO. In slave mode, when selected and there is no data frame in the TX FIFO, the slave will transmit all '1's. In both master and slave modes, received data frames will be lost if the RX FIFO is full."]
    #[inline(always)]
    #[must_use]
    pub fn master_mode(&mut self) -> MasterModeW<SpiCtrlSpec> {
        MasterModeW::new(self, 31)
    }
}
#[doc = "SPI control\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiCtrlSpec;
impl crate::RegisterSpec for SpiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrl::R`](R) reader structure"]
impl crate::Readable for SpiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrl::W`](W) writer structure"]
impl crate::Writable for SpiCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPI_CTRL to value 0x0300_0000"]
impl crate::Resettable for SpiCtrlSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
