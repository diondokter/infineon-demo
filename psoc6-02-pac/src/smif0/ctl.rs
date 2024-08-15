#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XipMode {
    #[doc = "0: '0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    MmioMode = 0,
    #[doc = "1: 1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    XipMode = 1,
}
impl From<XipMode> for bool {
    #[inline(always)]
    fn from(variant: XipMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIP_MODE` reader - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XipModeR = crate::BitReader<XipMode>;
impl XipModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XipMode {
        match self.bits {
            false => XipMode::MmioMode,
            true => XipMode::XipMode,
        }
    }
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn is_mmio_mode(&self) -> bool {
        *self == XipMode::MmioMode
    }
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn is_xip_mode(&self) -> bool {
        *self == XipMode::XipMode
    }
}
#[doc = "Field `XIP_MODE` writer - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XipModeW<'a, REG> = crate::BitWriter<'a, REG, XipMode>;
impl<'a, REG> XipModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn mmio_mode(self) -> &'a mut crate::W<REG> {
        self.variant(XipMode::MmioMode)
    }
    #[doc = "1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn xip_mode(self) -> &'a mut crate::W<REG> {
        self.variant(XipMode::XipMode)
    }
}
#[doc = "Field `CLOCK_IF_RX_SEL` reader - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
pub type ClockIfRxSelR = crate::FieldReader;
#[doc = "Field `CLOCK_IF_RX_SEL` writer - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
pub type ClockIfRxSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DESELECT_DELAY` reader - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DeselectDelayR = crate::FieldReader;
#[doc = "Field `DESELECT_DELAY` writer - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DeselectDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    #[doc = "0: 0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BusError = 0,
    #[doc = "1: 1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    WaitStates = 1,
}
impl From<Block> for bool {
    #[inline(always)]
    fn from(variant: Block) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLOCK` reader - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BlockR = crate::BitReader<Block>;
impl BlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Block {
        match self.bits {
            false => Block::BusError,
            true => Block::WaitStates,
        }
    }
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn is_bus_error(&self) -> bool {
        *self == Block::BusError
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn is_wait_states(&self) -> bool {
        *self == Block::WaitStates
    }
}
#[doc = "Field `BLOCK` writer - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BlockW<'a, REG> = crate::BitWriter<'a, REG, Block>;
impl<'a, REG> BlockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn bus_error(self) -> &'a mut crate::W<REG> {
        self.variant(Block::BusError)
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn wait_states(self) -> &'a mut crate::W<REG> {
        self.variant(Block::WaitStates)
    }
}
#[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enabled {
    #[doc = "0: N/A"]
    Disabled = 0,
    #[doc = "1: N/A"]
    Enabled = 1,
}
impl From<Enabled> for bool {
    #[inline(always)]
    fn from(variant: Enabled) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
pub type EnabledR = crate::BitReader<Enabled>;
impl EnabledR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enabled {
        match self.bits {
            false => Enabled::Disabled,
            true => Enabled::Enabled,
        }
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enabled::Disabled
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enabled::Enabled
    }
}
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG, Enabled>;
impl<'a, REG> EnabledW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Disabled)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enabled::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn xip_mode(&self) -> XipModeR {
        XipModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    pub fn clock_if_rx_sel(&self) -> ClockIfRxSelR {
        ClockIfRxSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&self) -> DeselectDelayR {
        DeselectDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    #[must_use]
    pub fn xip_mode(&mut self) -> XipModeW<CtlSpec> {
        XipModeW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Specifies device interface receiver clock 'clk_if_rx' source. MISO data is captured on the rising edge of 'clk_if_rx'. '0': 'spi_clk_out' (internal clock) '1': !'spi_clk_out' (internal clock) '2': 'spi_clk_in' (feedback clock) '3': !'spi_clk_in' (feedback clock) Note: the device interface transmitter clock 'clk_if_tx' is fixed and is 'spi_clk_out' MOSI data is driven on the falling edge of 'clk_if_tx'."]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_rx_sel(&mut self) -> ClockIfRxSelW<CtlSpec> {
        ClockIfRxSelW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 interface clock cycle. '1': 2 interface clock cycles. '2': 3 interface clock cycles. '3': 4 interface clock cycles. '4': 5 interface clock cycles. '5': 6 interface clock cycles. '6': 7 interface clock cycles. '7': 8 interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    #[must_use]
    pub fn deselect_delay(&mut self) -> DeselectDelayW<CtlSpec> {
        DeselectDelayW::new(self, 16)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BlockW<CtlSpec> {
        BlockW::new(self, 24)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Note: Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x3000"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x3000;
}
