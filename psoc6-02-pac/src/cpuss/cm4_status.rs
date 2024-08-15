#[doc = "Register `CM4_STATUS` reader"]
pub type R = crate::R<Cm4StatusSpec>;
#[doc = "Field `SLEEPING` reader - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
pub type SleepingR = crate::BitReader;
#[doc = "Field `SLEEPDEEP` reader - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
pub type SleepdeepR = crate::BitReader;
#[doc = "Field `PWR_DONE` reader - After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
pub type PwrDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
    #[inline(always)]
    pub fn sleeping(&self) -> SleepingR {
        SleepingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - After a PWR_MODE change this flag indicates if the new power mode has taken effect or not. Note: this flag can also change as a result of a change in debug power up req"]
    #[inline(always)]
    pub fn pwr_done(&self) -> PwrDoneR {
        PwrDoneR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "CM4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4StatusSpec;
impl crate::RegisterSpec for Cm4StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_status::R`](R) reader structure"]
impl crate::Readable for Cm4StatusSpec {}
#[doc = "`reset()` method sets CM4_STATUS to value 0x13"]
impl crate::Resettable for Cm4StatusSpec {
    const RESET_VALUE: u32 = 0x13;
}
