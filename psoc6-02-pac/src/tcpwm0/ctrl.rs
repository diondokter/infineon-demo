#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `COUNTER_ENABLED` reader - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
pub type CounterEnabledR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER_ENABLED` writer - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
pub type CounterEnabledW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    pub fn counter_enabled(&self) -> CounterEnabledR {
        CounterEnabledR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter enables for counters 0 up to CNT_NR-1. '0': counter disabled. '1': counter enabled. Counter static configuration information (e.g. CTRL.MODE, all TR_CTRL0, TR_CTRL1, and TR_CTRL2 register fields) should only be modified when the counter is disabled. When a counter is disabled, command and status information associated to the counter is cleared by HW, this includes: - the associated counter triggers in the CMD register are set to '0'. - the counter's interrupt cause fields in counter's INTR register. - the counter's status fields in counter's STATUS register.. - the counter's trigger outputs ('tr_overflow', 'tr_underflow' and 'tr_compare_match'). - the counter's line outputs ('line_out' and 'line_compl_out'). In multi-core environments, use the CTRL_SET/CTRL_CLR registers to avoid race-conditions on read-modify-write attempts to this register."]
    #[inline(always)]
    #[must_use]
    pub fn counter_enabled(&mut self) -> CounterEnabledW<CtrlSpec> {
        CounterEnabledW::new(self, 0)
    }
}
#[doc = "TCPWM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
