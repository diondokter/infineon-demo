#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `WR_EN` reader - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WrEnR = crate::BitReader;
#[doc = "Field `WR_EN` writer - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_EN` reader - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CryptoEnR = crate::BitReader;
#[doc = "Field `CRYPTO_EN` writer - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CryptoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SEL` reader - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DataSelR = crate::FieldReader;
#[doc = "Field `DATA_SEL` writer - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DataSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENABLED` reader - Device enable: '0': Disabled. '1': Enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Device enable: '0': Disabled. '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&self) -> CryptoEnR {
        CryptoEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&self) -> DataSelR {
        DataSelR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WrEnW<CtlSpec> {
        WrEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_en(&mut self) -> CryptoEnW<CtlSpec> {
        CryptoEnW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    #[must_use]
    pub fn data_sel(&mut self) -> DataSelW<CtlSpec> {
        DataSelW::new(self, 16)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
