#[doc = "Register `RES_CAUSE` reader"]
pub type R = crate::R<ResCauseSpec>;
#[doc = "Register `RES_CAUSE` writer"]
pub type W = crate::W<ResCauseSpec>;
#[doc = "Field `RESET_WDT` reader - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type ResetWdtR = crate::BitReader;
#[doc = "Field `RESET_WDT` writer - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
pub type ResetWdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ACT_FAULT` reader - Fault logging system requested a reset from its Active logic."]
pub type ResetActFaultR = crate::BitReader;
#[doc = "Field `RESET_ACT_FAULT` writer - Fault logging system requested a reset from its Active logic."]
pub type ResetActFaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_DPSLP_FAULT` reader - Fault logging system requested a reset from its DeepSleep logic."]
pub type ResetDpslpFaultR = crate::BitReader;
#[doc = "Field `RESET_DPSLP_FAULT` writer - Fault logging system requested a reset from its DeepSleep logic."]
pub type ResetDpslpFaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_CSV_WCO_LOSS` reader - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type ResetCsvWcoLossR = crate::BitReader;
#[doc = "Field `RESET_CSV_WCO_LOSS` writer - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
pub type ResetCsvWcoLossW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_SOFT` reader - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type ResetSoftR = crate::BitReader;
#[doc = "Field `RESET_SOFT` writer - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
pub type ResetSoftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT0` reader - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type ResetMcwdt0R = crate::BitReader;
#[doc = "Field `RESET_MCWDT0` writer - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
pub type ResetMcwdt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT1` reader - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type ResetMcwdt1R = crate::BitReader;
#[doc = "Field `RESET_MCWDT1` writer - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
pub type ResetMcwdt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT2` reader - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type ResetMcwdt2R = crate::BitReader;
#[doc = "Field `RESET_MCWDT2` writer - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
pub type ResetMcwdt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_MCWDT3` reader - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type ResetMcwdt3R = crate::BitReader;
#[doc = "Field `RESET_MCWDT3` writer - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
pub type ResetMcwdt3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_wdt(&self) -> ResetWdtR {
        ResetWdtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    pub fn reset_act_fault(&self) -> ResetActFaultR {
        ResetActFaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    pub fn reset_dpslp_fault(&self) -> ResetDpslpFaultR {
        ResetDpslpFaultR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    pub fn reset_csv_wco_loss(&self) -> ResetCsvWcoLossR {
        ResetCsvWcoLossR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    pub fn reset_soft(&self) -> ResetSoftR {
        ResetSoftR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt0(&self) -> ResetMcwdt0R {
        ResetMcwdt0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt1(&self) -> ResetMcwdt1R {
        ResetMcwdt1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt2(&self) -> ResetMcwdt2R {
        ResetMcwdt2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    pub fn reset_mcwdt3(&self) -> ResetMcwdt3R {
        ResetMcwdt3R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A basic WatchDog Timer (WDT) reset has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_wdt(&mut self) -> ResetWdtW<ResCauseSpec> {
        ResetWdtW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault logging system requested a reset from its Active logic."]
    #[inline(always)]
    #[must_use]
    pub fn reset_act_fault(&mut self) -> ResetActFaultW<ResCauseSpec> {
        ResetActFaultW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault logging system requested a reset from its DeepSleep logic."]
    #[inline(always)]
    #[must_use]
    pub fn reset_dpslp_fault(&mut self) -> ResetDpslpFaultW<ResCauseSpec> {
        ResetDpslpFaultW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock supervision logic requested a reset due to loss of a watch-crystal clock."]
    #[inline(always)]
    #[must_use]
    pub fn reset_csv_wco_loss(&mut self) -> ResetCsvWcoLossW<ResCauseSpec> {
        ResetCsvWcoLossW::new(self, 3)
    }
    #[doc = "Bit 4 - A CPU requested a system reset through it's SYSRESETREQ. This can be done via a debugger probe or in firmware."]
    #[inline(always)]
    #[must_use]
    pub fn reset_soft(&mut self) -> ResetSoftW<ResCauseSpec> {
        ResetSoftW::new(self, 4)
    }
    #[doc = "Bit 5 - Multi-Counter Watchdog timer reset #0 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt0(&mut self) -> ResetMcwdt0W<ResCauseSpec> {
        ResetMcwdt0W::new(self, 5)
    }
    #[doc = "Bit 6 - Multi-Counter Watchdog timer reset #1 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt1(&mut self) -> ResetMcwdt1W<ResCauseSpec> {
        ResetMcwdt1W::new(self, 6)
    }
    #[doc = "Bit 7 - Multi-Counter Watchdog timer reset #2 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt2(&mut self) -> ResetMcwdt2W<ResCauseSpec> {
        ResetMcwdt2W::new(self, 7)
    }
    #[doc = "Bit 8 - Multi-Counter Watchdog timer reset #3 has occurred since last power cycle."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mcwdt3(&mut self) -> ResetMcwdt3W<ResCauseSpec> {
        ResetMcwdt3W::new(self, 8)
    }
}
#[doc = "Reset Cause Observation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`res_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResCauseSpec;
impl crate::RegisterSpec for ResCauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res_cause::R`](R) reader structure"]
impl crate::Readable for ResCauseSpec {}
#[doc = "`write(|w| ..)` method takes [`res_cause::W`](W) writer structure"]
impl crate::Writable for ResCauseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES_CAUSE to value 0"]
impl crate::Resettable for ResCauseSpec {
    const RESET_VALUE: u32 = 0;
}
