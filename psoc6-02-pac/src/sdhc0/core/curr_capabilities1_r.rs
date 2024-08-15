#[doc = "Register `CURR_CAPABILITIES1_R` reader"]
pub type R = crate::R<CurrCapabilities1RSpec>;
#[doc = "Field `MAX_CUR_33V` reader - Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
pub type MaxCur33vR = crate::FieldReader;
#[doc = "Field `MAX_CUR_30V` reader - Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
pub type MaxCur30vR = crate::FieldReader;
#[doc = "Field `MAX_CUR_18V` reader - Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
pub type MaxCur18vR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 3.3V This bit specifies the Maximum Current for 3.3V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_33v(&self) -> MaxCur33vR {
        MaxCur33vR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Current for 3.0V This bit specifies the Maximum Current for 3.0V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_30v(&self) -> MaxCur30vR {
        MaxCur30vR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Maximum Current for 1.8V This bit specifies the Maximum Current for 1.8V VDD1 power supply for the card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_18v(&self) -> MaxCur18vR {
        MaxCur18vR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Current Capabilities Register - 0 to 31\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_capabilities1_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrCapabilities1RSpec;
impl crate::RegisterSpec for CurrCapabilities1RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_capabilities1_r::R`](R) reader structure"]
impl crate::Readable for CurrCapabilities1RSpec {}
#[doc = "`reset()` method sets CURR_CAPABILITIES1_R to value 0"]
impl crate::Resettable for CurrCapabilities1RSpec {
    const RESET_VALUE: u32 = 0;
}
