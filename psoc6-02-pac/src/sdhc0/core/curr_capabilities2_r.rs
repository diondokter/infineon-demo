#[doc = "Register `CURR_CAPABILITIES2_R` reader"]
pub type R = crate::R<CurrCapabilities2RSpec>;
#[doc = "Field `MAX_CUR_VDD2_18V` reader - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
pub type MaxCurVdd2_18vR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Maximum Current for 1.8V VDD2 This bit specifies the Maximum Current for 1.8V VDD2 power supply for the UHS-II card. - 0: Get information through another method - 1: 4mA - 2: 8mA - 3: 13mA - ....... - 255: 1020mA"]
    #[inline(always)]
    pub fn max_cur_vdd2_18v(&self) -> MaxCurVdd2_18vR {
        MaxCurVdd2_18vR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Maximum Current Capabilities Register - 32 to 63\n\nYou can [`read`](crate::Reg::read) this register and get [`curr_capabilities2_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrCapabilities2RSpec;
impl crate::RegisterSpec for CurrCapabilities2RSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`curr_capabilities2_r::R`](R) reader structure"]
impl crate::Readable for CurrCapabilities2RSpec {}
#[doc = "`reset()` method sets CURR_CAPABILITIES2_R to value 0"]
impl crate::Resettable for CurrCapabilities2RSpec {
    const RESET_VALUE: u32 = 0;
}
