#[doc = "Register `EP0_CNT` reader"]
pub type R = crate::R<Ep0CntSpec>;
#[doc = "Register `EP0_CNT` writer"]
pub type W = crate::W<Ep0CntSpec>;
#[doc = "Field `BYTE_COUNT` reader - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
pub type ByteCountR = crate::FieldReader;
#[doc = "Field `BYTE_COUNT` writer - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
pub type ByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataValid {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    DataError = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    DataValid = 1,
}
impl From<DataValid> for bool {
    #[inline(always)]
    fn from(variant: DataValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_VALID` reader - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DataValidR = crate::BitReader<DataValid>;
impl DataValidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataValid {
        match self.bits {
            false => DataValid::DataError,
            true => DataValid::DataValid,
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn is_data_error(&self) -> bool {
        *self == DataValid::DataError
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn is_data_valid(&self) -> bool {
        *self == DataValid::DataValid
    }
}
#[doc = "Field `DATA_VALID` writer - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
pub type DataValidW<'a, REG> = crate::BitWriter<'a, REG, DataValid>;
impl<'a, REG> DataValidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn data_error(self) -> &'a mut crate::W<REG> {
        self.variant(DataValid::DataError)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn data_valid(self) -> &'a mut crate::W<REG> {
        self.variant(DataValid::DataValid)
    }
}
#[doc = "Field `DATA_TOGGLE` reader - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DataToggleR = crate::BitReader;
#[doc = "Field `DATA_TOGGLE` writer - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
pub type DataToggleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    pub fn byte_count(&self) -> ByteCountR {
        ByteCountR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    pub fn data_valid(&self) -> DataValidR {
        DataValidR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    pub fn data_toggle(&self) -> DataToggleR {
        DataToggleR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - These bits indicate the number of data bytes in a transaction. For IN transactions firmware loads the count with the number of bytes to be transmitted to the host from the endpoint FIFO. Valid values are 0 to 8. For OUT or SETUP transactions the count is updated by hardware to the number of data bytes received plus two for the CRC bytes. Valid values are 2 to 10."]
    #[inline(always)]
    #[must_use]
    pub fn byte_count(&mut self) -> ByteCountW<Ep0CntSpec> {
        ByteCountW::new(self, 0)
    }
    #[doc = "Bit 6 - This bit is used for OUT/SETUP transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    #[must_use]
    pub fn data_valid(&mut self) -> DataValidW<Ep0CntSpec> {
        DataValidW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    #[must_use]
    pub fn data_toggle(&mut self) -> DataToggleW<Ep0CntSpec> {
        DataToggleW::new(self, 7)
    }
}
#[doc = "Endpoint0 count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0CntSpec;
impl crate::RegisterSpec for Ep0CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0_cnt::R`](R) reader structure"]
impl crate::Readable for Ep0CntSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0_cnt::W`](W) writer structure"]
impl crate::Writable for Ep0CntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0_CNT to value 0"]
impl crate::Resettable for Ep0CntSpec {
    const RESET_VALUE: u32 = 0;
}
