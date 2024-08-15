#[doc = "Register `TIMER_CTL` reader"]
pub type R = crate::R<TimerCtlSpec>;
#[doc = "Register `TIMER_CTL` writer"]
pub type W = crate::W<TimerCtlSpec>;
#[doc = "Field `PERIOD` reader - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PeriodR = crate::FieldReader<u16>;
#[doc = "Field `PERIOD` writer - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
pub type PeriodW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SCALE` reader - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
pub type ScaleR = crate::BitReader;
#[doc = "Field `SCALE` writer - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
pub type ScaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_SEQUENCE` reader - 1': Starts1 the HV automatic sequencing Cleared by HW"]
pub type AutoSequenceR = crate::BitReader;
#[doc = "Field `AUTO_SEQUENCE` writer - 1': Starts1 the HV automatic sequencing Cleared by HW"]
pub type AutoSequenceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_PROG` reader - 1 during pre-program operation"]
pub type PreProgR = crate::BitReader;
#[doc = "Field `PRE_PROG` writer - 1 during pre-program operation"]
pub type PreProgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_PROG_CSL` reader - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
pub type PreProgCslR = crate::BitReader;
#[doc = "Field `PRE_PROG_CSL` writer - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
pub type PreProgCslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUMP_EN` reader - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PumpEnR = crate::BitReader;
#[doc = "Field `PUMP_EN` writer - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
pub type PumpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EN` reader - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type AclkEnR = crate::BitReader;
#[doc = "Field `ACLK_EN` writer - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
pub type AclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_EN` reader - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TimerEnR = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
pub type TimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    pub fn period(&self) -> PeriodR {
        PeriodR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15 - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    pub fn scale(&self) -> ScaleR {
        ScaleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - 1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    pub fn auto_sequence(&self) -> AutoSequenceR {
        AutoSequenceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1 during pre-program operation"]
    #[inline(always)]
    pub fn pre_prog(&self) -> PreProgR {
        PreProgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    pub fn pre_prog_csl(&self) -> PreProgCslR {
        PreProgCslR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    pub fn pump_en(&self) -> PumpEnR {
        PumpEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    pub fn aclk_en(&self) -> AclkEnR {
        AclkEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    pub fn timer_en(&self) -> TimerEnR {
        TimerEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Timer period in either microseconds (SCALE is '0') or 100's of microseconds (SCALE is '1') multiples."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PeriodW<TimerCtlSpec> {
        PeriodW::new(self, 0)
    }
    #[doc = "Bit 15 - Timer tick scale: 0: 1 microsecond. 1: 100 microseconds."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> ScaleW<TimerCtlSpec> {
        ScaleW::new(self, 15)
    }
    #[doc = "Bit 24 - 1': Starts1 the HV automatic sequencing Cleared by HW"]
    #[inline(always)]
    #[must_use]
    pub fn auto_sequence(&mut self) -> AutoSequenceW<TimerCtlSpec> {
        AutoSequenceW::new(self, 24)
    }
    #[doc = "Bit 25 - 1 during pre-program operation"]
    #[inline(always)]
    #[must_use]
    pub fn pre_prog(&mut self) -> PreProgW<TimerCtlSpec> {
        PreProgW::new(self, 25)
    }
    #[doc = "Bit 26 - 0: CSL lines driven by CSL_DAC 1: CSL lines driven by VNEG_G"]
    #[inline(always)]
    #[must_use]
    pub fn pre_prog_csl(&mut self) -> PreProgCslW<TimerCtlSpec> {
        PreProgCslW::new(self, 26)
    }
    #[doc = "Bit 29 - Pump enable: 0: disabled 1: enabled (also requires FM_CTL.IF_SEL to be'1', this additional restriction is required to prevent non intentional clearing of the FM). SW sets this field to '1' to generate a single PE pulse. HW clears this field when timer is expired."]
    #[inline(always)]
    #[must_use]
    pub fn pump_en(&mut self) -> PumpEnW<TimerCtlSpec> {
        PumpEnW::new(self, 29)
    }
    #[doc = "Bit 30 - ACLK enable (generates a single cycle pulse for the FM): 0: disabled 1: enabled. SW set this field to '1' to generate a single cycle pulse. HW sets this field to '0' when the pulse is generated."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_en(&mut self) -> AclkEnW<TimerCtlSpec> {
        AclkEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Timer enable: 0: disabled 1: enabled. SW sets this field to '1' to start the timer. HW sets this field to '0' when the timer is expired."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TimerEnW<TimerCtlSpec> {
        TimerEnW::new(self, 31)
    }
}
#[doc = "Timer control\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerCtlSpec;
impl crate::RegisterSpec for TimerCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ctl::R`](R) reader structure"]
impl crate::Readable for TimerCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_ctl::W`](W) writer structure"]
impl crate::Writable for TimerCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_CTL to value 0x0400_0001"]
impl crate::Resettable for TimerCtlSpec {
    const RESET_VALUE: u32 = 0x0400_0001;
}
