#[doc = "Register `TIMER_CLK_CTL` reader"]
pub type R = crate::R<TimerClkCtlSpec>;
#[doc = "Register `TIMER_CLK_CTL` writer"]
pub type W = crate::W<TimerClkCtlSpec>;
#[doc = "Field `TIMER_CLOCK_FREQ` reader - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
pub type TimerClockFreqR = crate::FieldReader;
#[doc = "Field `TIMER_CLOCK_FREQ` writer - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
pub type TimerClockFreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_PEON` reader - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgPeonR = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_PEON` writer - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgPeonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_PEOFF` reader - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgPeoffR = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_PEOFF` writer - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgPeoffW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ01` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq01R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ01` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    pub fn timer_clock_freq(&self) -> TimerClockFreqR {
        TimerClockFreqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peon(&self) -> RgrantDelayPrgPeonR {
        RgrantDelayPrgPeonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_peoff(&self) -> RgrantDelayPrgPeoffR {
        RgrantDelayPrgPeoffR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq01(&self) -> RgrantDelayPrgSeq01R {
        RgrantDelayPrgSeq01R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clk_t frequency divider to provide the 1MHz reference clock for the Regif Timer. Equal to the frequency in MHz of the timer clock 'clk_t'. Example: if 'clk_t' has a frequency of 4 MHz then this field value is '4' Max clk_t frequency = 100MHz. This field is updated at runtime with the 'SW_TIMER_CLOCK_FREQ ' value from the HV parameters table"]
    #[inline(always)]
    #[must_use]
    pub fn timer_clock_freq(&mut self) -> TimerClockFreqW<TimerClkCtlSpec> {
        TimerClockFreqW::new(self, 0)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_peon(&mut self) -> RgrantDelayPrgPeonW<TimerClkCtlSpec> {
        RgrantDelayPrgPeonW::new(self, 8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_peoff(&mut self) -> RgrantDelayPrgPeoffW<TimerClkCtlSpec> {
        RgrantDelayPrgPeoffW::new(self, 16)
    }
    #[doc = "Bits 24:31 - PROG&amp;PRE_PROG: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq01(&mut self) -> RgrantDelayPrgSeq01W<TimerClkCtlSpec> {
        RgrantDelayPrgSeq01W::new(self, 24)
    }
}
#[doc = "Timer prescaler (clk_t to timer clock frequency divider)\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_clk_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_clk_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerClkCtlSpec;
impl crate::RegisterSpec for TimerClkCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_clk_ctl::R`](R) reader structure"]
impl crate::Readable for TimerClkCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer_clk_ctl::W`](W) writer structure"]
impl crate::Writable for TimerClkCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_CLK_CTL to value 0x08"]
impl crate::Resettable for TimerClkCtlSpec {
    const RESET_VALUE: u32 = 0x08;
}
