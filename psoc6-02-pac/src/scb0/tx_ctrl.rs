#[doc = "Register `TX_CTRL` reader"]
pub type R = crate::R<TxCtrlSpec>;
#[doc = "Register `TX_CTRL` writer"]
pub type W = crate::W<TxCtrlSpec>;
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
pub type DataWidthR = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MsbFirstR = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPEN_DRAIN` reader - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell. - SPI mode, 'spi_miso' IO cell."]
pub type OpenDrainR = crate::BitReader;
#[doc = "Field `OPEN_DRAIN` writer - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell. - SPI mode, 'spi_miso' IO cell."]
pub type OpenDrainW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&self) -> MsbFirstR {
        MsbFirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell. - SPI mode, 'spi_miso' IO cell."]
    #[inline(always)]
    pub fn open_drain(&self) -> OpenDrainR {
        OpenDrainR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the amount of bits in a transmitted data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DataWidthW<TxCtrlSpec> {
        DataWidthW::new(self, 0)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MsbFirstW<TxCtrlSpec> {
        MsbFirstW::new(self, 8)
    }
    #[doc = "Bit 16 - Each IO cell 'xxx' has two associated IP output signals 'xxx_out_en' and 'xxx_out'. '0': Normal operation mode. Typically, this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by a single IO cell. In this operation mode, for an IO cell 'xxx' that is used as an output, the 'xxx_out_en' output enable signal is typically constant '1' the 'xxx_out' output is the outputted value. In other words, in normal operation mode, the 'xxx_out' output is used to control the IO cell output value: 'xxx_out' is '0' to drive an IO cell output value of '0' and 'xxx_out' is '1' to drive an IO cell output value of '1'. '1': Open drain operation mode. Typically this operation mode is used for IO cells that are connected to (board) wires/lines that are driven by multiple IO cells (possibly on multiple chips). In this operation mode, for and IO cell 'xxx' that is used as an output, the 'xxx_out_en' output controls the outputted value. Typically, open drain operation mode drives low/'0' and the 'xxx_out' output is constant '1'. In other words, in open drain operation mode, the 'xxx_out_en' output is used to control the IO cell output value: in drive low/'0' mode: 'xxx_out_en' is '1' (drive enabled) to drive an IO cell output value of '0' and 'xxx_out_en' is '1' (drive disabled) to not drive an IO cell output value (another IO cell can drive the wire/line or a pull up results in a wire/line value '1'). The open drain mode is supported for: - I2C mode, 'i2c_scl' and 'i2c_sda' IO cells. - UART mode, 'uart_tx' IO cell. - SPI mode, 'spi_miso' IO cell."]
    #[inline(always)]
    #[must_use]
    pub fn open_drain(&mut self) -> OpenDrainW<TxCtrlSpec> {
        OpenDrainW::new(self, 16)
    }
}
#[doc = "Transmitter control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCtrlSpec;
impl crate::RegisterSpec for TxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ctrl::R`](R) reader structure"]
impl crate::Readable for TxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_ctrl::W`](W) writer structure"]
impl crate::Writable for TxCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_CTRL to value 0x0107"]
impl crate::Resettable for TxCtrlSpec {
    const RESET_VALUE: u32 = 0x0107;
}
