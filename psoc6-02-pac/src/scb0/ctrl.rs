#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `OVS` reader - N/A"]
pub type OvsR = crate::FieldReader;
#[doc = "Field `OVS` writer - N/A"]
pub type OvsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EC_AM_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EcAmModeR = crate::BitReader;
#[doc = "Field `EC_AM_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EcAmModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EC_OP_MODE` reader - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EcOpModeR = crate::BitReader;
#[doc = "Field `EC_OP_MODE` writer - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EcOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EZ_MODE` reader - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EzModeR = crate::BitReader;
#[doc = "Field `EZ_MODE` writer - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EzModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_MODE` reader - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type ByteModeR = crate::BitReader;
#[doc = "Field `BYTE_MODE` writer - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
pub type ByteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_RESP_MODE` reader - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CmdRespModeR = crate::BitReader;
#[doc = "Field `CMD_RESP_MODE` writer - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
pub type CmdRespModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type AddrAcceptR = crate::BitReader;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type AddrAcceptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLOCK` reader - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BlockR = crate::BitReader;
#[doc = "Field `BLOCK` writer - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
pub type BlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Inter-Integrated Circuits (I2C) mode."]
    I2c = 0,
    #[doc = "1: Serial Peripheral Interface (SPI) mode."]
    Spi = 1,
    #[doc = "2: Universal Asynchronous Receiver/Transmitter (UART) mode."]
    Uart = 2,
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
            0 => Some(Mode::I2c),
            1 => Some(Mode::Spi),
            2 => Some(Mode::Uart),
            _ => None,
        }
    }
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == Mode::I2c
    }
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Mode::Spi
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == Mode::Uart
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inter-Integrated Circuits (I2C) mode."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::I2c)
    }
    #[doc = "Serial Peripheral Interface (SPI) mode."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Spi)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter (UART) mode."]
    #[inline(always)]
    pub fn uart(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Uart)
    }
}
#[doc = "Field `ENABLED` reader - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&self) -> OvsR {
        OvsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EcAmModeR {
        EcAmModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EcOpModeR {
        EcOpModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EzModeR {
        EzModeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    pub fn byte_mode(&self) -> ByteModeR {
        ByteModeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CmdRespModeR {
        CmdRespModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&self) -> AddrAcceptR {
        AddrAcceptR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    pub fn block(&self) -> BlockR {
        BlockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OvsW<CtrlSpec> {
        OvsW::new(self, 0)
    }
    #[doc = "Bit 8 - Internally clocked mode ('0') or externally clocked mode ('1') address matching (I2C) or selection (SPI). In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_am_mode(&mut self) -> EcAmModeW<CtrlSpec> {
        EcAmModeW::new(self, 8)
    }
    #[doc = "Bit 9 - Internally clocked mode ('0') or externally clocked mode ('1') operation. In internally clocked mode, the serial interface protocols run off the peripheral clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_op_mode(&mut self) -> EcOpModeW<CtrlSpec> {
        EcOpModeW::new(self, 9)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames mot separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of up to 256 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_mode(&mut self) -> EzModeW<CtrlSpec> {
        EzModeW::new(self, 10)
    }
    #[doc = "Bit 11 - Determines the number of bits per FIFO data element: '0': 16-bit FIFO data elements. '1': 8-bit FIFO data elements. This mode doubles the amount of FIFO entries, but TX_CTRL.DATA_WIDTH and RX_CTRL.DATA_WIDTH are restricted to \\[0, 7\\]."]
    #[inline(always)]
    #[must_use]
    pub fn byte_mode(&mut self) -> ByteModeW<CtrlSpec> {
        ByteModeW::new(self, 11)
    }
    #[doc = "Bit 12 - Determines CMD_RESP mode of operation: '0': CMD_RESP mode disabled. '1': CMD_RESP mode enabled (also requires EC_AM_MODE and EC_OP_MODE to be set to '1')."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_mode(&mut self) -> CmdRespModeW<CtrlSpec> {
        CmdRespModeW::new(self, 12)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when ADDR_ACCEPT is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn addr_accept(&mut self) -> AddrAcceptW<CtrlSpec> {
        AddrAcceptW::new(self, 16)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the MMIO SW accesses to EZ memory coincide/collide, this bit determines whether a SW access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, MMIO read operations return 0xffff:ffff and MMIO write operations are ignored. Colliding accesses are registered as interrupt causes: field BLOCKED of MMIO registers INTR_TX and INTR_RX."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BlockW<CtrlSpec> {
        BlockW::new(self, 17)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlSpec> {
        ModeW::new(self, 24)
    }
    #[doc = "Bit 31 - IP enabled ('1') or not ('0'). The proper order in which to initialize the IP is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL to enable IP, select the specific operation mode and oversampling factor. When the IP is enabled, no control information should be changed. Changes should be made AFTER disabling the IP, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the IP is re-enabled. Note that disabling the IP will cause re-initialization of the design and associated state is lost (e.g. FIFO content)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtrlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Generic control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_000f"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x0300_000f;
}
