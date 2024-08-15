#[doc = "Register `SIE_EP2_CNT0` reader"]
pub type R = crate::R<SieEp2Cnt0Spec>;
#[doc = "Register `SIE_EP2_CNT0` writer"]
pub type W = crate::W<SieEp2Cnt0Spec>;
#[doc = "Field `DATA_COUNT_MSB` reader - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DataCountMsbR = crate::FieldReader;
#[doc = "Field `DATA_COUNT_MSB` writer - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
pub type DataCountMsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings.\n\nValue on reset: 0"]
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
#[doc = "Field `DATA_VALID` reader - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
#[doc = "Field `DATA_VALID` writer - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    pub fn data_count_msb(&self) -> DataCountMsbR {
        DataCountMsbR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
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
    #[doc = "Bits 0:2 - These bits are the 3 MSb bits of an 11-bit counter. The LSb are the Data Count\\[7:0\\]
bits of the CNT1 register. Refer to the CNT1 register for more information."]
    #[inline(always)]
    #[must_use]
    pub fn data_count_msb(&mut self) -> DataCountMsbW<SieEp2Cnt0Spec> {
        DataCountMsbW::new(self, 0)
    }
    #[doc = "Bit 6 - This bit is used for OUT transactions only and is read only. It is cleared to '0' if CRC bit stuffing errors or PID errors occur. This bit does not update for some endpoint mode settings."]
    #[inline(always)]
    #[must_use]
    pub fn data_valid(&mut self) -> DataValidW<SieEp2Cnt0Spec> {
        DataValidW::new(self, 6)
    }
    #[doc = "Bit 7 - This bit selects the DATA packet's toggle state. For IN transactions firmware must set this bit to the expected state. For OUT transactions the hardware sets this bit to the state of the received Data Toggle bit."]
    #[inline(always)]
    #[must_use]
    pub fn data_toggle(&mut self) -> DataToggleW<SieEp2Cnt0Spec> {
        DataToggleW::new(self, 7)
    }
}
#[doc = "Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep2_cnt0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep2_cnt0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SieEp2Cnt0Spec;
impl crate::RegisterSpec for SieEp2Cnt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep2_cnt0::R`](R) reader structure"]
impl crate::Readable for SieEp2Cnt0Spec {}
#[doc = "`write(|w| ..)` method takes [`sie_ep2_cnt0::W`](W) writer structure"]
impl crate::Writable for SieEp2Cnt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIE_EP2_CNT0 to value 0"]
impl crate::Resettable for SieEp2Cnt0Spec {
    const RESET_VALUE: u32 = 0;
}
