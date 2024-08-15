#[doc = "Register `CM0_STATUS` reader"]
pub type R = crate::R<Cm0StatusSpec>;
#[doc = "Field `SLEEPING` reader - Specifies if the CPU is in Active, Sleep or DeepSleep power mode: - Active power mode: SLEEPING is '0'. - Sleep power mode: SLEEPING is '1' and SLEEPDEEP is '0'. - DeepSleep power mode: SLEEPING is '1' and SLEEPDEEP is '1'."]
pub type SleepingR = crate::BitReader;
#[doc = "Field `SLEEPDEEP` reader - Specifies if the CPU is in Sleep or DeepSleep power mode. See SLEEPING field."]
pub type SleepdeepR = crate::BitReader;
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
}
#[doc = "CM0+ status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0StatusSpec;
impl crate::RegisterSpec for Cm0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_status::R`](R) reader structure"]
impl crate::Readable for Cm0StatusSpec {}
#[doc = "`reset()` method sets CM0_STATUS to value 0"]
impl crate::Resettable for Cm0StatusSpec {
    const RESET_VALUE: u32 = 0;
}
