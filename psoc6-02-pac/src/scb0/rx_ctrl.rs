#[doc = "Register `RX_CTRL` reader"]
pub type R = crate::R<RxCtrlSpec>;
#[doc = "Register `RX_CTRL` writer"]
pub type W = crate::W<RxCtrlSpec>;
#[doc = "Field `DATA_WIDTH` reader - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
pub type DataWidthR = crate::FieldReader;
#[doc = "Field `DATA_WIDTH` writer - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
pub type DataWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSB_FIRST` reader - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MsbFirstR = crate::BitReader;
#[doc = "Field `MSB_FIRST` writer - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
pub type MsbFirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEDIAN` reader - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
pub type MedianR = crate::BitReader;
#[doc = "Field `MEDIAN` writer - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
pub type MedianW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    pub fn data_width(&self) -> DataWidthR {
        DataWidthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    pub fn msb_first(&self) -> MsbFirstR {
        MsbFirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    pub fn median(&self) -> MedianR {
        MedianR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dataframe width. DATA_WIDTH + 1 is the expected amount of bits in received data frame. This number does not include start, parity and stop bits. For UART mode, the valid range is \\[3, 8\\]. For SPI, the valid range is \\[3, 15\\]. For I2C the only valid value is 7. In EZ mode (for both SPI and I2C), the only valid value is 7."]
    #[inline(always)]
    #[must_use]
    pub fn data_width(&mut self) -> DataWidthW<RxCtrlSpec> {
        DataWidthW::new(self, 0)
    }
    #[doc = "Bit 8 - Least significant bit first ('0') or most significant bit first ('1'). For I2C, this field should be '1'."]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MsbFirstW<RxCtrlSpec> {
        MsbFirstW::new(self, 8)
    }
    #[doc = "Bit 9 - Median filter. When '1', a digital 3 taps median filter is performed on input interface lines. This filter should reduce the susceptibility to errors. However, its requires higher oversampling values. For UART IrDA submode, this field should always be '1'."]
    #[inline(always)]
    #[must_use]
    pub fn median(&mut self) -> MedianW<RxCtrlSpec> {
        MedianW::new(self, 9)
    }
}
#[doc = "Receiver control\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCtrlSpec;
impl crate::RegisterSpec for RxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ctrl::R`](R) reader structure"]
impl crate::Readable for RxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_ctrl::W`](W) writer structure"]
impl crate::Writable for RxCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_CTRL to value 0x0107"]
impl crate::Resettable for RxCtrlSpec {
    const RESET_VALUE: u32 = 0x0107;
}
