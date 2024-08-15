#[doc = "Register `SPI_STATUS` reader"]
pub type R = crate::R<SpiStatusSpec>;
#[doc = "Field `BUS_BUSY` reader - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
pub type BusBusyR = crate::BitReader;
#[doc = "Field `SPI_EC_BUSY` reader - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
pub type SpiEcBusyR = crate::BitReader;
#[doc = "Field `CURR_EZ_ADDR` reader - SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
pub type CurrEzAddrR = crate::FieldReader;
#[doc = "Field `BASE_EZ_ADDR` reader - SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
pub type BaseEzAddrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SPI bus is busy. The bus is considered busy ('1') during an ongoing transaction. For Motorola and National submodes, the busy bit is '1', when the slave selection is activated. For TI submode, the busy bit is '1' from the time the preceding/coinciding slave select is activated for the first transmitted data frame, till the last MOSI/MISO bit of the last data frame is transmitted."]
    #[inline(always)]
    pub fn bus_busy(&self) -> BusBusyR {
        BusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates whether the externally clocked logic is potentially accessing the EZ memory and/or updating BASE_ADDR or CURR_ADDR (this is only possible in EZ mode). This bit can be used by SW to determine whether BASE_ADDR and CURR_ADDR are reliable."]
    #[inline(always)]
    pub fn spi_ec_busy(&self) -> SpiEcBusyR {
        SpiEcBusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SPI current EZ address. Current address pointer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable (during an ongoing transfer when SPI_EC_BUSY is '1'), as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn curr_ez_addr(&self) -> CurrEzAddrR {
        CurrEzAddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI base EZ address. Address as provided by a SPI write transfer. This field is only reliable in internally clocked mode. In externally clocked mode the field may be unreliable, as clock domain synchronization is not performed in the design."]
    #[inline(always)]
    pub fn base_ez_addr(&self) -> BaseEzAddrR {
        BaseEzAddrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SPI status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpiStatusSpec;
impl crate::RegisterSpec for SpiStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_status::R`](R) reader structure"]
impl crate::Readable for SpiStatusSpec {}
#[doc = "`reset()` method sets SPI_STATUS to value 0"]
impl crate::Resettable for SpiStatusSpec {
    const RESET_VALUE: u32 = 0;
}
