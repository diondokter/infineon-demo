#[doc = "Register `TX_WATCHDOG` reader"]
pub type R = crate::R<TxWatchdogSpec>;
#[doc = "Register `TX_WATCHDOG` writer"]
pub type W = crate::W<TxWatchdogSpec>;
#[doc = "Field `WD_COUNTER` reader - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WdCounterR = crate::FieldReader<u32>;
#[doc = "Field `WD_COUNTER` writer - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
pub type WdCounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    pub fn wd_counter(&self) -> WdCounterR {
        WdCounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start value of the TX watchdog. With the reset value of 0x0000:0000 the counter is disabled. This is clocked by the AHB-Lite system clock 'clk_sys'."]
    #[inline(always)]
    #[must_use]
    pub fn wd_counter(&mut self) -> WdCounterW<TxWatchdogSpec> {
        WdCounterW::new(self, 0)
    }
}
#[doc = "Transmitter watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_watchdog::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_watchdog::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxWatchdogSpec;
impl crate::RegisterSpec for TxWatchdogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_watchdog::R`](R) reader structure"]
impl crate::Readable for TxWatchdogSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_watchdog::W`](W) writer structure"]
impl crate::Writable for TxWatchdogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_WATCHDOG to value 0"]
impl crate::Resettable for TxWatchdogSpec {
    const RESET_VALUE: u32 = 0;
}
